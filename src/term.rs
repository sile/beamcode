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
    Unknown(u8),
}

impl TermKind {
    fn from_tag(tag: u8) -> Self {
        match tag & 0b111 {
            0 => Self::Usize,
            1 => Self::Integer,
            2 => Self::Atom,
            3 => Self::XRegister,
            4 => Self::YRegister,
            5 => Self::Label,
            6 => Self::Character,
            7 => match tag >> 4 {
                0b0001 => Self::List,
                0b0010 => Self::FloatingPointRegister,
                0b0011 => Self::AllocationList,
                0b0100 => Self::Literal,
                0b0101 => Self::TypedRegister,
                _ => Self::Unknown(tag),
            },
            _ => unreachable!(),
        }
    }

    fn expect(self, expected: &[Self]) -> Result<(), DecodeError> {
        if expected.iter().any(|&x| x == self) {
            Ok(())
        } else {
            Err(DecodeError::UnexpectedTerm {
                expected: expected.to_owned(),
                actual: self,
            })
        }
    }

    const fn tag(self) -> u8 {
        match self {
            Self::Usize => 0,
            Self::Integer => 1,
            Self::Atom => 2,
            Self::XRegister => 3,
            Self::YRegister => 4,
            Self::Label => 5,
            Self::Character => 6,
            Self::List => 0b0001_0111,
            Self::FloatingPointRegister => 0b0010_0111,
            Self::AllocationList => 0b0011_0111,
            Self::Literal => 0b0100_0111,
            Self::TypedRegister => 0b0101_0111,
            Self::Unknown(tag) => tag,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode)]
pub enum Term {
    Usize(usize),
    Integer(BigInt),
    Atom(Atom),
    XRegister(XRegister),
    YRegister(YRegister),
    Label(Label),
    List(List),
    Literal(Literal),
    TypedRegister(TypedRegister),
    // TODO: Alloc List, etc
}

impl Decode for Term {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        match TermKind::from_tag(tag) {
            TermKind::Usize => Decode::decode_with_tag(reader, tag).map(Self::Usize),
            TermKind::Integer => Decode::decode_with_tag(reader, tag).map(Self::Integer),
            TermKind::Atom => Decode::decode_with_tag(reader, tag).map(Self::Atom),
            TermKind::XRegister => Decode::decode_with_tag(reader, tag).map(Self::XRegister),
            TermKind::YRegister => Decode::decode_with_tag(reader, tag).map(Self::YRegister),
            TermKind::Label => Decode::decode_with_tag(reader, tag).map(Self::Label),
            TermKind::Character => todo!(),
            TermKind::List => Decode::decode_with_tag(reader, tag).map(Self::List),
            TermKind::FloatingPointRegister => todo!(),
            TermKind::AllocationList => todo!(),
            TermKind::TypedRegister => {
                Decode::decode_with_tag(reader, tag).map(Self::TypedRegister)
            }
            TermKind::Literal => Decode::decode_with_tag(reader, tag).map(Self::Literal),
            TermKind::Unknown(_) => Err(DecodeError::UnknownTermTag { tag }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypedRegister {
    X { register: XRegister, ty: usize },
    Y { register: YRegister, ty: usize },
}

impl Decode for TypedRegister {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::TypedRegister])?;

        let tag = reader.read_u8()?;
        let kind = TermKind::from_tag(tag);
        kind.expect(&[TermKind::XRegister, TermKind::YRegister])?;
        if kind == TermKind::XRegister {
            let register = XRegister::decode_with_tag(reader, tag)?;
            let ty = usize::decode(reader)?;
            Ok(Self::X { register, ty })
        } else {
            let register = YRegister::decode_with_tag(reader, tag)?;
            let ty = usize::decode(reader)?;
            Ok(Self::Y { register, ty })
        }
    }
}

impl Encode for TypedRegister {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_u8(TermKind::TypedRegister.tag())?;
        match self {
            Self::X { register, ty } => {
                register.encode(writer)?;
                ty.encode(writer)?;
            }
            Self::Y { register, ty } => {
                register.encode(writer)?;
                ty.encode(writer)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode)]
pub enum Register {
    X(XRegister),
    Y(YRegister),
    Typed(TypedRegister),
}

impl Decode for Register {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        match TermKind::from_tag(tag) {
            TermKind::XRegister => Decode::decode_with_tag(reader, tag).map(Self::X),
            TermKind::YRegister => Decode::decode_with_tag(reader, tag).map(Self::Y),
            TermKind::TypedRegister => Decode::decode_with_tag(reader, tag).map(Self::Typed),
            actual => Err(DecodeError::UnexpectedTerm {
                actual,
                expected: vec![
                    TermKind::XRegister,
                    TermKind::YRegister,
                    TermKind::TypedRegister,
                ],
            }),
        }
    }
}

impl Decode for usize {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::Usize])?;
        let value = decode_usize(tag, reader)?;
        Ok(value)
    }
}

impl Encode for usize {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TermKind::Usize.tag(), *self, writer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Literal {
    pub value: usize,
}

impl Decode for Literal {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::Literal])?;
        Ok(Self {
            value: usize::decode(reader)?,
        })
    }
}

impl Encode for Literal {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_u8(TermKind::Literal.tag())?;
        self.value.encode(writer)
    }
}

impl Decode for BigInt {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::Integer])?;
        let value = decode_integer(tag, reader)?;
        Ok(value)
    }
}

impl Encode for BigInt {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_integer(TermKind::Integer.tag(), self, writer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Atom {
    pub value: usize,
}

impl Decode for Atom {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::Atom])?;

        let value = decode_usize(tag, reader)?;
        Ok(Self { value })
    }
}

impl Encode for Atom {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TermKind::Atom.tag(), self.value, writer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct XRegister {
    pub value: usize,
    pub ty: Option<usize>,
}

impl Decode for XRegister {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::XRegister])?;

        let value = decode_usize(tag, reader)?;
        Ok(Self { value, ty: None })
    }
}

impl Encode for XRegister {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TermKind::XRegister.tag(), self.value, writer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct YRegister {
    pub value: usize,
    pub ty: Option<usize>,
}

impl Decode for YRegister {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::YRegister])?;

        let value = decode_usize(tag, reader)?;
        Ok(Self { value, ty: None })
    }
}

impl Encode for YRegister {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TermKind::YRegister.tag(), self.value, writer)
    }
}

impl Decode for Vec<YRegister> {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        let list = List::decode_with_tag(reader, tag)?;
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
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::Label])?;

        let value = decode_usize(tag, reader)?;
        Ok(Self { value })
    }
}

impl Encode for Label {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TermKind::Label.tag(), self.value, writer)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct List<T = Term> {
    pub elements: Vec<T>,
}

impl<T: Decode> Decode for List<T> {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::List])?;

        let size = usize::decode(reader)?;
        let elements = (0..size)
            .map(|_| T::decode(reader))
            .collect::<Result<_, _>>()?;
        Ok(Self { elements })
    }
}

impl<T: Encode> Encode for List<T> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_u8(TermKind::List.tag())?;
        self.elements.len().encode(writer)?;
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
        let byte_size = usize::decode(reader)?;
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
        let byte_size = usize::decode(reader)?;
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
        (bytes.len() - 8).encode(writer)?;
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
