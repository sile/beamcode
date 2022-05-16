use crate::{DecodeError, USIZE_BYTES};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

#[derive(Debug, thiserror::Error)]
pub enum ConvertTermError {
    #[error("expected a literal, but got {term:?}")]
    NotLiteral { term: Term },
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Literal(Literal),
    Atom(Atom),
    XRegister(XRegister),
    YRegister(YRegister),
    Label(Label),
    List(List),
    // TODO: Integer (maybe a big-num)
    // TODO: Alloc List, etc
}

impl Term {
    // TODO: pub crate
    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        match tag & 0b111 {
            TAG_U => Literal::decode(tag, reader).map(Self::Literal),
            TAG_I => todo!(),
            TAG_A => todo!(),
            TAG_X => todo!(),
            TAG_Y => todo!(),
            TAG_F => todo!(),
            TAG_H => todo!(),
            TAG_Z => todo!(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Register {
    X(XRegister),
    Y(YRegister),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    XRegister(XRegister),
    YRegister(YRegister),
    Literal(Literal),
    Atom(Atom),
    // Integer
}

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

impl TryFrom<Term> for Literal {
    type Error = ConvertTermError;

    fn try_from(term: Term) -> Result<Self, Self::Error> {
        if let Term::Literal(t) = term {
            Ok(Self { value: t.value })
        } else {
            Err(ConvertTermError::NotLiteral { term })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Atom {
    pub value: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct XRegister {
    pub value: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct YRegister {
    pub value: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label {
    pub value: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct List {
    pub elements: Vec<Term>,
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
}
