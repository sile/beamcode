use beam_file::chunk::CodeChunk;
use byteorder::ReadBytesExt as _;
use std::io::Read;

pub const INSTRUCTION_SET_VERSION: u32 = 0;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("supported instruction set version is {INSTRUCTION_SET_VERSION}, but got {version}")]
    UnsupportedInstructionSetVersion { version: u32 },

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub fn parse_code_chunk(chunk: &CodeChunk) -> Result<(), ParseError> {
    if chunk.version != INSTRUCTION_SET_VERSION {
        return Err(ParseError::UnsupportedInstructionSetVersion {
            version: chunk.version,
        });
    }

    dbg!(chunk.info_size);
    dbg!(chunk.opcode_max);
    dbg!(chunk.label_count);
    dbg!(chunk.function_count);
    dbg!(chunk.bytecode.len());
    let mut reader = &mut &chunk.bytecode[..];
    while !reader.is_empty() {
        parse_op(&mut reader)?;
    }
    todo!()
}

pub fn parse_op<R: Read>(reader: &mut R) -> Result<(), ParseError> {
    let opcode = reader.read_u8()?;
    todo!("OP: {opcode}")
}

// https://blog.stenmans.org/theBeamBook/#SEC-BeamModulesCTE
pub fn decode_compact_term() {}

#[derive(Debug, Clone)]
pub enum Op {
    Label { label: String },
}

impl Op {
    pub fn opcode(&self) -> u8 {
        match self {
            Self::Label { .. } => 1,
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Self::Label { .. } => 1,
        }
    }
}
