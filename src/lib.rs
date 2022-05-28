//! Erlang BEAM instructions encoding / decoding library.
//!
//! # References
//!
//! - [The BEAM Book - Generic BEAM Instructions](https://blog.stenmans.org/theBeamBook/#CH-Instructions)
use crate::instruction::Instruction;
use crate::term::TermKind;
use beamcode_derive::{Decode, Encode};
use byteorder::ReadBytesExt as _;
use num::BigInt;
use std::io::{Read, Write};

pub mod instruction;
pub mod term;

/// This trait allows decoding an object from a byte sequence.
pub trait Decode: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        Self::decode_with_tag(reader, tag)
    }

    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError>;
}

/// This trait allows encoding an object into a byte sequence.
pub trait Encode {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError>;
}

/// Decoding errors.
#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("unknown term tag: {tag}")]
    UnknownTermTag { tag: u8 },

    #[error("unexpected term: expected={expected:?}, actual={actual:?}")]
    UnexpectedTerm {
        expected: Vec<TermKind>,
        actual: TermKind,
    },

    #[error("unknown allocation list item tag: {tag}")]
    UnknownAllocationListItemTag { tag: usize },

    #[error("unknown opcode: {opcode}")]
    UnknownOpcode { opcode: u8 },

    #[error("invalid Unicode codepoint: {value}")]
    InvalidUnicodeCodepoint { value: u32 },

    #[error(transparent)]
    NumError(#[from] num::bigint::TryFromBigIntError<BigInt>),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Encoding errors.
#[derive(Debug, thiserror::Error)]
pub enum EncodeError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

/// Decodes BEAM instructions.
pub fn decode_instructions(bytecode: &[u8]) -> Result<Vec<Instruction>, DecodeError> {
    let mut reader = bytecode;
    let mut instructions = Vec::new();
    while !reader.is_empty() {
        let instruction = Instruction::decode(&mut reader)?;
        instructions.push(instruction);
    }
    Ok(instructions)
}

/// Encodes BEAM instructions.
pub fn encode_instructions(instructions: &[Instruction]) -> Result<Vec<u8>, EncodeError> {
    let mut buf = Vec::new();
    for instruction in instructions {
        instruction.encode(&mut buf)?;
    }
    Ok(buf)
}
