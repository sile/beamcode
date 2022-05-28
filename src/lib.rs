use crate::op::Op;
use crate::term::TermKind;
use beamop_derive::{Decode, Encode};
use byteorder::ReadBytesExt as _;
use num::BigInt;
use std::io::{Read, Write};

pub mod op;
pub mod term;

pub trait Decode: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        Self::decode_with_tag(reader, tag)
    }

    fn decode_with_tag<R: Read>(reader: &mut R, tag: u8) -> Result<Self, DecodeError>;
}

pub trait Encode {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError>;
}

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

#[derive(Debug, thiserror::Error)]
pub enum EncodeError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub fn decode_ops(bytecode: &[u8]) -> Result<Vec<Op>, DecodeError> {
    let mut reader = bytecode;
    let mut ops = Vec::new();
    while !reader.is_empty() {
        let op = Op::decode(&mut reader)?;
        ops.push(op);
    }
    Ok(ops)
}

pub fn encode_ops(ops: &[Op]) -> Result<Vec<u8>, EncodeError> {
    let mut buf = Vec::new();
    for op in ops {
        op.encode(&mut buf)?;
    }
    Ok(buf)
}
