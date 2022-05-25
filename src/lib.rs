use crate::op::Op;
use crate::terms::Term;
use beamop_derive::{Decode, Opcode};
use std::io::Read;

// TODO: s/terms/term/
pub mod op;
pub mod terms;

pub trait Opcode {
    const CODE: u8;
}

pub trait Decode: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>;
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
    ConvertTermError(#[from] crate::terms::ConvertTermError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl From<std::convert::Infallible> for DecodeError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}

pub fn decode_ops(bytecode: &[u8]) -> Result<Vec<Op>, DecodeError> {
    let mut reader = bytecode;
    let mut ops = Vec::new();
    while !reader.is_empty() {
        ops.push(Op::decode(&mut reader)?);
    }
    Ok(ops)
}
