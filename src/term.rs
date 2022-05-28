use crate::{Decode, DecodeError, Encode, EncodeError, USIZE_BYTES};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use num::BigInt;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermKind {
    Usize,
    Integer,
    Atom,
    XRegister,
    YRegister,
    Label,
    Character,
    List,
    FloatingPointRegister,
    AllocationList,
    Literal,
    TypedRegister,
    Unknown,
}

impl TermKind {
    fn from_tag(tag: u8) -> Self {
        match tag & 0b111 {
            TAG_U => Self::Usize,
            TAG_I => Self::Integer,
            TAG_A => Self::Atom,
            TAG_X => Self::XRegister,
            TAG_Y => Self::YRegister,
            TAG_F => Self::Label,
            TAG_H => Self::Character,
            TAG_Z => match tag >> 4 {
                0b0001 => Self::List,
                0b0010 => Self::FloatingPointRegister,
                0b0011 => Self::AllocationList,
                0b0100 => Self::Literal,
                0b0101 => Self::TypedRegister,
                _ => Self::Unknown,
            },
            _ => unreachable!(),
        }
    }

    fn expect(&self, expected: &[Self]) -> Result<(), DecodeError> {
        if expected.iter().any(|x| x == self) {
            Ok(())
        } else {
            Err(DecodeError::UnexpectedTerm {
                expected: expected.to_owned(),
                actual: *self,
            })
        }
    }
}

// TODO: remove?
const TAG_U: u8 = 0; // Literal
const TAG_I: u8 = 1; // Integer
const TAG_A: u8 = 2; // Atom
const TAG_X: u8 = 3; // X regsiter
const TAG_Y: u8 = 4; // Y register
const TAG_F: u8 = 5; // Label
const TAG_H: u8 = 6; // Character
const TAG_Z: u8 = 7; // Extended

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode)]
pub enum Term {
    Literal(Literal),
    Integer(Integer),
    Atom(Atom),
    XRegister(XRegister),
    YRegister(YRegister),
    Label(Label),
    List(List),
    ExtendedLiteral(ExtendedLiteral),
    // TODO: Alloc List, etc
}

impl Decode for Term {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        let mut reader = once(tag).chain(reader);
        match TermKind::from_tag(tag) {
            TermKind::Usize => Literal::decode(&mut reader).map(Self::Literal),
            TermKind::Integer => Integer::decode(&mut reader).map(Self::Integer),
            TermKind::Atom => Atom::decode(&mut reader).map(Self::Atom),
            TermKind::XRegister => XRegister::decode(&mut reader).map(Self::XRegister),
            TermKind::YRegister => YRegister::decode(&mut reader).map(Self::YRegister),
            TermKind::Label => Label::decode(&mut reader).map(Self::Label),
            TermKind::Character => todo!(),
            TermKind::List => List::decode(&mut reader).map(Self::List),
            TermKind::FloatingPointRegister => todo!(),
            TermKind::AllocationList => todo!(),
            TermKind::TypedRegister => todo!(),
            TermKind::Literal => ExtendedLiteral::decode(&mut reader).map(Self::ExtendedLiteral),
            TermKind::Unknown => Err(DecodeError::UnknownTermTag { tag }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode)]
pub enum Register {
    X(XRegister),
    Y(YRegister),
}

impl From<Register> for Term {
    fn from(v: Register) -> Self {
        match v {
            Register::X(v) => Term::XRegister(v),
            Register::Y(v) => Term::YRegister(v),
        }
    }
}

impl Decode for Register {
    fn decode<R: Read>(mut reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        match tag & 0b111 {
            TAG_X => XRegister::decode(&mut once(tag).chain(reader)).map(Self::X),
            TAG_Y => YRegister::decode(&mut once(tag).chain(reader)).map(Self::Y),
            TAG_Z if tag >> 4 == 0b0101 => {
                let tag = reader.read_u8()?;
                let mut register = match tag & 0b111 {
                    TAG_X => XRegister::decode(&mut once(tag).chain(&mut reader)).map(Self::X)?,
                    TAG_Y => YRegister::decode(&mut once(tag).chain(&mut reader)).map(Self::Y)?,
                    _ => return Err(DecodeError::UnknownTermTag { tag }),
                };
                let ty = Literal::decode(&mut reader)?;
                match &mut register {
                    Self::X(r) => r.ty = Some(ty.value),
                    Self::Y(r) => r.ty = Some(ty.value),
                };
                Ok(register)
            }
            _ => Err(DecodeError::UnknownTermTag { tag }),
        }
    }
}

// TODO: move
#[derive(Debug)]
struct Once {
    byte: u8,
    read: bool,
}

impl Read for Once {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.read || buf.is_empty() {
            Ok(0)
        } else {
            buf[0] = self.byte;
            self.read = true;
            Ok(1)
        }
    }
}

fn once(byte: u8) -> Once {
    Once { byte, read: false }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    XRegister(XRegister),
    YRegister(YRegister),
    Literal(Literal),
    Integer(Integer),
    Atom(Atom),
}

impl Decode for usize {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        if tag & 0b111 != TAG_U {
            return Err(DecodeError::UnknownTermTag { tag });
        }
        let value = decode_usize(tag, reader)?;
        Ok(value)
    }
}

impl Encode for usize {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TAG_U, *self, writer)
    }
}

// TODO: impl Decode
// TODO(?): s/Literal/Usize/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Literal {
    pub value: usize,
}

impl Decode for Literal {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        if tag & 0b111 != TAG_U {
            return Err(DecodeError::UnknownTermTag { tag });
        }
        let value = decode_usize(tag, reader)?;
        Ok(Self { value })
    }
}

impl Encode for Literal {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TAG_U, self.value, writer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtendedLiteral {
    pub value: usize,
}

impl ExtendedLiteral {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        TermKind::from_tag(tag).expect(&[TermKind::Literal])?;

        let literal = Literal::decode(reader)?;
        Ok(Self {
            value: literal.value,
        })
    }
}

impl Encode for ExtendedLiteral {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_u8(TAG_Z | 0b0100_0000)?;
        let literal = Literal { value: self.value };
        literal.encode(writer)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Integer {
    pub value: BigInt,
}

impl Decode for Integer {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        TermKind::from_tag(tag).expect(&[TermKind::Integer])?;

        let value = decode_integer(tag, reader)?;
        Ok(Self { value })
    }
}

impl Encode for Integer {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_integer(TAG_I, &self.value, writer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Atom {
    pub value: usize,
}

impl Decode for Atom {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        TermKind::from_tag(tag).expect(&[TermKind::Atom])?;

        let value = decode_usize(tag, reader)?;
        Ok(Self { value })
    }
}

impl Encode for Atom {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TAG_A, self.value, writer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct XRegister {
    pub value: usize,
    pub ty: Option<usize>,
}

impl Decode for XRegister {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        TermKind::from_tag(tag).expect(&[TermKind::XRegister])?;

        let value = decode_usize(tag, reader)?;
        Ok(Self { value, ty: None })
    }
}

impl Encode for XRegister {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TAG_X, self.value, writer)
    }
}

// TODO
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct TypedXRegister {
//     pub value: usize,
//     pub ty: usize,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct YRegister {
    pub value: usize,
    pub ty: Option<usize>,
}

impl Decode for YRegister {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        TermKind::from_tag(tag).expect(&[TermKind::YRegister])?;

        let value = decode_usize(tag, reader)?;
        Ok(Self { value, ty: None })
    }
}

impl Encode for YRegister {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TAG_Y, self.value, writer)
    }
}

impl Decode for Vec<YRegister> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let list = List::decode(reader)?;
        Ok(list.elements)
    }
}

impl Encode for Vec<YRegister> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let list = List {
            elements: self.iter().copied().map(Term::YRegister).collect(),
        };
        list.encode(writer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label {
    pub value: usize,
}

impl Decode for Label {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        TermKind::from_tag(tag).expect(&[TermKind::Label])?;

        let value = decode_usize(tag, reader)?;
        Ok(Self { value })
    }
}

impl Encode for Label {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TAG_F, self.value, writer)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct List<T = Term> {
    pub elements: Vec<T>,
}

impl<T: Decode> Decode for List<T> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        TermKind::from_tag(tag).expect(&[TermKind::List])?;

        let size = Literal::decode(reader)?.value;
        let elements = (0..size)
            .map(|_| T::decode(reader))
            .collect::<Result<_, _>>()?;
        Ok(Self { elements })
    }
}

impl<T: Encode> Encode for List<T> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_u8(TAG_Z | 0b0001_0000)?;
        let size = Literal {
            value: self.elements.len(),
        };
        size.encode(writer)?;
        for x in &self.elements {
            x.encode(writer)?;
        }
        Ok(())
    }
}

// TODO: rename
fn decode_usize<R: Read>(tag: u8, reader: &mut R) -> Result<usize, DecodeError> {
    if (tag & 0b1_000) == 0 {
        Ok((tag >> 4) as usize)
    } else if (tag & 0b10_000) == 0 {
        let v = reader.read_u8()? as usize;
        Ok((usize::from(tag & 0b111_00_000) << 3) | v)
    } else if (tag >> 5) != 0b111 {
        let byte_size = usize::from(tag >> 5) + 2;
        if byte_size > USIZE_BYTES as usize {
            Err(DecodeError::TooLargeUsizeValue { byte_size })
        } else {
            Ok(reader.read_uint::<BigEndian>(byte_size)? as usize)
        }
    } else {
        let byte_size = Literal::decode(reader)?.value;
        Err(DecodeError::TooLargeUsizeValue { byte_size })
    }
}

// TODO: rename
fn encode_usize<W: Write>(tag: u8, value: usize, writer: &mut W) -> Result<(), EncodeError> {
    if value < 16 {
        writer.write_u8((value << 4) as u8 | tag)?;
    } else if value < 0x800 {
        writer.write_u8(((value >> 3) as u8 & 0b1110_0000) | tag | 0b000_1000)?;
        writer.write_u8((value & 0xFF) as u8)?;
    } else {
        let bytes = value.to_be_bytes();
        encode_num_bytes(tag, &bytes, writer)?;
    }
    Ok(())
}

fn decode_integer<R: Read>(tag: u8, reader: &mut R) -> Result<BigInt, DecodeError> {
    if (tag & 0b1_000) == 0 {
        Ok(BigInt::from(tag >> 4))
    } else if (tag & 0b10_000) == 0 {
        let v = u64::from(reader.read_u8()?);
        Ok(BigInt::from((u64::from(tag) & 0b111_00_000) << 3 | v))
    } else if (tag >> 5) != 0b111 {
        let byte_size = usize::from(tag >> 5) + 2;
        let mut buf = vec![0; byte_size];
        reader.read_exact(&mut buf)?;
        Ok(BigInt::from_signed_bytes_be(&buf))
    } else {
        let byte_size = Literal::decode(reader)?.value;
        let mut buf = vec![0; byte_size];
        reader.read_exact(&mut buf)?;
        Ok(BigInt::from_signed_bytes_be(&buf))
    }
}

fn encode_integer<W: Write>(tag: u8, value: &BigInt, writer: &mut W) -> Result<(), EncodeError> {
    if let Ok(v) = usize::try_from(value.clone()) {
        encode_usize(tag, v, writer)
    } else if let Ok(v) = i16::try_from(value.clone()) {
        let bytes = v.to_be_bytes();
        encode_num_bytes(tag, &bytes, writer)
    } else {
        let bytes = value.to_signed_bytes_be();
        encode_num_bytes(tag, &bytes, writer)
    }
}

fn encode_num_bytes<W: Write>(tag: u8, bytes: &[u8], writer: &mut W) -> Result<(), EncodeError> {
    assert!(bytes.len() >= 2, "bug");

    if bytes.len() <= 8 {
        let mut n = bytes.len();
        for (i, b) in bytes.iter().copied().enumerate() {
            if b != 0 {
                if (b & 0b1000_0000) != 0 {
                    n += 1;
                }
                writer.write_u8(((n - 2) << 5) as u8 | 0b0001_1000 | tag)?;
                if (b & 0b1000_0000) != 0 {
                    writer.write_u8(0)?;
                }
                for &b in &bytes[i..] {
                    writer.write_u8(b)?;
                }
                break;
            }
            n -= 1;
        }
    } else {
        writer.write_u8(tag | 0b1111_1000)?;
        let size = Literal {
            value: bytes.len() - 8,
        };
        size.encode(writer)?;
        writer.write_all(bytes)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_usize_works() {
        let data: &[(&[u8], usize)] = &[
            (&[0], 0),
            (&[16], 1),
            (&[8, 20], 20),
            (&[40, 144], 400),
            (&[24, 87, 28], 22300),
            (&[56, 15, 18, 6], 987654),
        ];
        for (input, expected) in data {
            let decoded = decode_usize(input[0], &mut &input[1..]).expect("decode failure");
            assert_eq!(decoded, *expected);
        }
    }

    #[test]
    fn too_large_usize_value() {
        let input = [248, 0, 0, 137, 16, 135, 184, 176, 52, 113, 21];
        assert!(matches!(
            decode_usize(input[0], &mut &input[1..]),
            Err(DecodeError::TooLargeUsizeValue { .. })
        ));
    }

    #[test]
    fn decode_integer_works() {
        let data: &[(&[u8], i64)] = &[
            (&[0], 0),
            (&[16], 1),
            (&[8, 20], 20),
            (&[40, 144], 400),
            (&[24, 87, 28], 22300),
            (&[56, 15, 18, 6], 987654),
            (&[24, 255, 255], -1),
            (&[24, 254, 189], -323),
            (&[88, 248, 164, 147, 83], -123432109),
        ];
        for (input, expected) in data {
            let decoded = decode_integer(input[0], &mut &input[1..]).expect("decode failure");
            assert_eq!(decoded, BigInt::from(*expected));
        }
    }
}
