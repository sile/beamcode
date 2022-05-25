use crate::op::Op;
use crate::term::Term;
use beamop_derive::{Decode, Encode, Opcode};
use std::io::{Read, Write};

pub mod op;
pub mod term;

pub trait Opcode {
    const CODE: u8;
}

pub trait Decode: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>;
}

pub trait Encode {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<(), EncodeError>;
}

const USIZE_BYTES: u32 = usize::BITS / 8;

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("unknown compact term tag: {tag}")]
    UnknownTermTag { tag: u8 },

    #[error("unknown opcode: {opcode}")]
    UnknownOpcode { opcode: u8 },

    #[error("expected a usize value ({USIZE_BYTES} bytes), but got a {byte_size} bytes value")]
    TooLargeUsizeValue { byte_size: usize },

    #[error(transparent)]
    ConvertTermError(#[from] crate::term::ConvertTermError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl From<std::convert::Infallible> for DecodeError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
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
        ops.push(Op::decode(&mut reader)?);
    }
    Ok(ops)
}

pub fn encode_ops(ops: &[Op]) -> Result<Vec<u8>, EncodeError> {
    let mut buf = Vec::new();
    let mut writer = &mut buf[..];
    for op in ops {
        op.encode(&mut writer)?;
    }
    Ok(buf)
}
