use crate::{Decode, DecodeError, Encode, EncodeError, USIZE_BYTES};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use num::BigInt;
use std::io::{Read, Write};

#[derive(Debug, thiserror::Error)]
pub enum ConvertTermError {
    #[error("expected a literal, but got {term:?}")]
    NotLiteral { term: Term },

    #[error("expected an integer, but got {term:?}")]
    NotInteger { term: Term },

    #[error("expected an atom, but got {term:?}")]
    NotAtom { term: Term },

    #[error("expected a label, but got {term:?}")]
    NotLabel { term: Term },

    #[error("expected a x-register, but got {term:?}")]
    NotXRegister { term: Term },

    #[error("expected a y-register, but got {term:?}")]
    NotYRegister { term: Term },

    #[error("expected a register, but got {term:?}")]
    NotRegister { term: Term },

    #[error("expected a list, but got {term:?}")]
    NotList { term: Term },

    #[error("expected an extended literal, but got {term:?}")]
    NotExtendedLiteral { term: Term },
}

// From beam_opcodes.hrl file.
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

impl Term {
    fn decode_extended<R: Read>(tag: u8, reader: &mut R) -> Result<Self, DecodeError> {
        match tag >> 3 {
            0b00010 => {
                let size: Literal = Self::decode(reader)?.try_into()?;
                (0..size.value)
                    .map(|_| Self::decode(reader))
                    .collect::<Result<_, _>>()
                    .map(|elements| Self::List(List { elements }))
            }
            0b00100 => {
                todo!("floating piont register");
            }
            0b00110 => {
                todo!("allocation list");
            }
            0b01000 => ExtendedLiteral::decode(reader).map(Self::ExtendedLiteral),
            _ => Err(DecodeError::UnknownTermTag { tag }),
        }
    }
}

impl Decode for Term {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        match tag & 0b111 {
            TAG_U => Literal::decode(tag, reader).map(Self::Literal),
            TAG_I => Integer::decode(tag, reader).map(Self::Integer),
            TAG_A => Atom::decode(tag, reader).map(Self::Atom),
            TAG_X => XRegister::decode(tag, reader).map(Self::XRegister),
            TAG_Y => YRegister::decode(tag, reader).map(Self::YRegister),
            TAG_F => Label::decode(tag, reader).map(Self::Label),
            TAG_H => todo!(),
            TAG_Z => Self::decode_extended(tag, reader),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Encode)]
pub enum Register {
    X(XRegister),
    Y(YRegister),
}

impl TryFrom<Term> for Register {
    type Error = ConvertTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        match term {
            Term::XRegister(t) => Ok(Self::X(t)),
            Term::YRegister(t) => Ok(Self::Y(t)),
            _ => Err(ConvertTermError::NotRegister { term }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    XRegister(XRegister),
    YRegister(YRegister),
    Literal(Literal),
    Integer(Integer),
    Atom(Atom),
}

// TODO: impl Decode
// TODO(?): s/Literal/Usize/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Literal {
    pub value: usize,
}

impl Literal {
    fn decode<R: Read>(tag: u8, reader: &mut R) -> Result<Self, DecodeError> {
        let value = decode_usize(tag, reader)?;
        Ok(Self { value })
    }
}

impl Encode for Literal {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TAG_U, self.value, writer)
    }
}

impl TryFrom<Term> for Literal {
    type Error = ConvertTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        if let Term::Literal(t) = term {
            Ok(t)
        } else {
            Err(ConvertTermError::NotLiteral { term })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ExtendedLiteral {
    pub value: usize,
}

impl ExtendedLiteral {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let literal: Literal = Term::decode(reader)?.try_into()?;
        Ok(Self {
            value: literal.value,
        })
    }
}

impl Encode for ExtendedLiteral {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        todo!()
    }
}

impl TryFrom<Term> for ExtendedLiteral {
    type Error = ConvertTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        if let Term::ExtendedLiteral(t) = term {
            Ok(t)
        } else {
            Err(ConvertTermError::NotExtendedLiteral { term })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Integer {
    pub value: BigInt,
}

impl Integer {
    fn decode<R: Read>(tag: u8, reader: &mut R) -> Result<Self, DecodeError> {
        let value = decode_integer(tag, reader)?;
        Ok(Self { value })
    }
}

impl Encode for Integer {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        todo!()
    }
}

impl TryFrom<Term> for Integer {
    type Error = ConvertTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        if let Term::Integer(t) = term {
            Ok(t)
        } else {
            Err(ConvertTermError::NotInteger { term })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Atom {
    pub value: usize,
}

impl Atom {
    fn decode<R: Read>(tag: u8, reader: &mut R) -> Result<Self, DecodeError> {
        let value = decode_usize(tag, reader)?;
        Ok(Self { value })
    }
}

impl Encode for Atom {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TAG_A, self.value, writer)
    }
}

impl TryFrom<Term> for Atom {
    type Error = ConvertTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        if let Term::Atom(t) = term {
            Ok(t)
        } else {
            Err(ConvertTermError::NotAtom { term })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct XRegister {
    pub value: usize,
}

impl XRegister {
    fn decode<R: Read>(tag: u8, reader: &mut R) -> Result<Self, DecodeError> {
        let value = decode_usize(tag, reader)?;
        Ok(Self { value })
    }
}

impl Encode for XRegister {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TAG_X, self.value, writer)
    }
}

impl TryFrom<Term> for XRegister {
    type Error = ConvertTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        if let Term::XRegister(t) = term {
            Ok(t)
        } else {
            Err(ConvertTermError::NotXRegister { term })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct YRegister {
    pub value: usize,
}

impl YRegister {
    fn decode<R: Read>(tag: u8, reader: &mut R) -> Result<Self, DecodeError> {
        let value = decode_usize(tag, reader)?;
        Ok(Self { value })
    }
}

impl Encode for YRegister {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TAG_X, self.value, writer)
    }
}

impl TryFrom<Term> for YRegister {
    type Error = ConvertTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        if let Term::YRegister(t) = term {
            Ok(t)
        } else {
            Err(ConvertTermError::NotYRegister { term })
        }
    }
}

impl Encode for Vec<YRegister> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        todo!()
    }
}

impl TryFrom<Term> for Vec<YRegister> {
    type Error = ConvertTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        List::try_from(term).and_then(|list| {
            list.elements
                .into_iter()
                .map(|x| YRegister::try_from(x))
                .collect()
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label {
    pub value: usize,
}

impl Label {
    fn decode<R: Read>(tag: u8, reader: &mut R) -> Result<Self, DecodeError> {
        let value = decode_usize(tag, reader)?;
        Ok(Self { value })
    }
}

impl Encode for Label {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        encode_usize(TAG_F, self.value, writer)
    }
}

impl TryFrom<Term> for Label {
    type Error = ConvertTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        if let Term::Label(t) = term {
            Ok(t)
        } else {
            Err(ConvertTermError::NotLabel { term })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct List {
    pub elements: Vec<Term>,
}

impl Encode for List {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        todo!()
    }
}

impl TryFrom<Term> for List {
    type Error = ConvertTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        if let Term::List(t) = term {
            Ok(t)
        } else {
            Err(ConvertTermError::NotList { term })
        }
    }
}

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
        let byte_size = Literal::try_from(Term::decode(reader)?)?.value;
        Err(DecodeError::TooLargeUsizeValue { byte_size })
    }
}

fn encode_usize<W: Write>(tag: u8, value: usize, writer: &mut W) -> Result<(), EncodeError> {
    if value < 16 {
        writer.write_u8((value << 4) as u8 | tag)?;
    } else if value < 0x800 {
        writer.write_u8(((value >> 3) as u8 & 0b1110_0000) | tag | 0b000_1000)?;
        writer.write_u8((value & 0xFF) as u8)?;
    } else {
        let bytes = value.to_be_bytes();
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
        let byte_size = Literal::try_from(Term::decode(reader)?)?.value;
        let mut buf = vec![0; byte_size];
        reader.read_exact(&mut buf)?;
        Ok(BigInt::from_signed_bytes_be(&buf))
    }
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
