//! Terms.
//!
//! # References
//!
//! - [The BEAM Book - Compact Term Encoding](https://blog.stenmans.org/theBeamBook/#SEC-BeamModulesCTE)
//! - [erlang/otp/lib/compiler/src/beam_asm.erl](https://github.com/erlang/otp/blob/master/lib/compiler/src/beam_asm.erl)
use crate::{Decode, DecodeError, Encode, EncodeError};
use byteorder::{ReadBytesExt, WriteBytesExt};
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
    Character(char),
    List(List),
    FloatingPointRegister(FloatingPointRegister),
    AllocationList(AllocationList),
    Literal(Literal),
    TypedRegister(TypedRegister),
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
            TermKind::Character => Decode::decode_with_tag(reader, tag).map(Self::Character),
            TermKind::List => Decode::decode_with_tag(reader, tag).map(Self::List),
            TermKind::FloatingPointRegister => {
                Decode::decode_with_tag(reader, tag).map(Self::FloatingPointRegister)
            }
            TermKind::AllocationList => {
                Decode::decode_with_tag(reader, tag).map(Self::AllocationList)
            }
            TermKind::Literal => Decode::decode_with_tag(reader, tag).map(Self::Literal),
            TermKind::TypedRegister => {
                Decode::decode_with_tag(reader, tag).map(Self::TypedRegister)
            }
            TermKind::Unknown(_) => Err(DecodeError::UnknownTermTag { tag }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode)]
pub enum Allocation {
    Words(usize),
    List(AllocationList),
}

impl Decode for Allocation {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        let kind = TermKind::from_tag(tag);
        kind.expect(&[TermKind::Usize, TermKind::AllocationList])?;
        if kind == TermKind::Usize {
            Decode::decode_with_tag(reader, tag).map(Self::Words)
        } else {
            Decode::decode_with_tag(reader, tag).map(Self::List)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AllocationList {
    pub items: Vec<AllocationListItem>,
}

impl Decode for AllocationList {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::AllocationList])?;
        let size = usize::decode(reader)?;
        let items = (0..size)
            .map(|_| Decode::decode(reader))
            .collect::<Result<_, _>>()?;
        Ok(Self { items })
    }
}

impl Encode for AllocationList {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_u8(TermKind::AllocationList.tag())?;
        self.items.len().encode(writer)?;
        for item in &self.items {
            item.encode(writer)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllocationListItem {
    Words(usize),
    Floats(usize),
    Funs(usize),
}

impl Decode for AllocationListItem {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        match usize::decode_with_tag(reader, tag)? {
            0 => usize::decode(reader).map(Self::Words),
            1 => usize::decode(reader).map(Self::Floats),
            2 => usize::decode(reader).map(Self::Funs),
            tag => Err(DecodeError::UnknownAllocationListItemTag { tag }),
        }
    }
}

impl Encode for AllocationListItem {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        match self {
            Self::Words(v) => {
                0.encode(writer)?;
                v.encode(writer)?;
            }
            Self::Floats(v) => {
                1.encode(writer)?;
                v.encode(writer)?;
            }
            Self::Funs(v) => {
                2.encode(writer)?;
                v.encode(writer)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FloatingPointRegister {
    pub value: usize,
}

impl Decode for FloatingPointRegister {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::FloatingPointRegister])?;
        Ok(Self {
            value: usize::decode(reader)?,
        })
    }
}

impl Encode for FloatingPointRegister {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_u8(TermKind::FloatingPointRegister.tag())?;
        self.value.encode(writer)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        let value = usize::try_from(decode_integer(tag, reader)?)?;
        Ok(value)
    }
}

impl Encode for usize {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_integer(TermKind::Usize.tag(), &BigInt::from(*self), writer)
    }
}

impl Decode for char {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::Character])?;
        let value = u32::try_from(decode_integer(tag, reader)?)?;
        char::from_u32(value).ok_or_else(|| DecodeError::InvalidUnicodeCodepoint { value })
    }
}

impl Encode for char {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_integer(
            TermKind::Character.tag(),
            &BigInt::from(u32::from(*self)),
            writer,
        )
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
        let value = usize::try_from(decode_integer(tag, reader)?)?;
        Ok(Self { value })
    }
}

impl Encode for Atom {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_integer(TermKind::Atom.tag(), &BigInt::from(self.value), writer)
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
        let value = usize::try_from(decode_integer(tag, reader)?)?;
        Ok(Self { value, ty: None })
    }
}

impl Encode for XRegister {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_integer(TermKind::XRegister.tag(), &BigInt::from(self.value), writer)
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
        let value = usize::try_from(decode_integer(tag, reader)?)?;
        Ok(Self { value, ty: None })
    }
}

impl Encode for YRegister {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_integer(TermKind::YRegister.tag(), &BigInt::from(self.value), writer)
    }
}

impl Decode for Vec<YRegister> {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        let list = List::decode_with_tag(reader, tag)?;
        Ok(list.items)
    }
}

impl Encode for Vec<YRegister> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        let list = List {
            items: self.iter().copied().map(Term::YRegister).collect(),
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
        let value = usize::try_from(decode_integer(tag, reader)?)?;
        Ok(Self { value })
    }
}

impl Encode for Label {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_integer(TermKind::Label.tag(), &BigInt::from(self.value), writer)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct List<T = Term> {
    pub items: Vec<T>,
}

impl<T: Decode> Decode for List<T> {
    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError> {
        TermKind::from_tag(tag).expect(&[TermKind::List])?;

        let size = usize::decode(reader)?;
        let items = (0..size)
            .map(|_| T::decode(reader))
            .collect::<Result<_, _>>()?;
        Ok(Self { items })
    }
}

impl<T: Encode> Encode for List<T> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        writer.write_u8(TermKind::List.tag())?;
        self.items.len().encode(writer)?;
        for x in &self.items {
            x.encode(writer)?;
        }
        Ok(())
    }
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
    if let Ok(v) = i16::try_from(value.clone()) {
        if v < 0 {
            let bytes = v.to_be_bytes();
            return encode_integer_bytes(tag, &bytes, writer);
        } else if v < 16 {
            writer.write_u8((v << 4) as u8 | tag)?;
            return Ok(());
        } else if v < 0x800 {
            writer.write_u8(((v >> 3) as u8 & 0b1110_0000) | tag | 0b000_1000)?;
            writer.write_u8((v & 0xFF) as u8)?;
            return Ok(());
        }
    }

    let bytes = value.to_signed_bytes_be();
    encode_integer_bytes(tag, &bytes, writer)
}

fn encode_integer_bytes<W: Write>(
    tag: u8,
    bytes: &[u8],
    writer: &mut W,
) -> Result<(), EncodeError> {
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
