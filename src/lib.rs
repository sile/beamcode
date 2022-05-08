use beam_file::chunk::CodeChunk;
use byteorder::ReadBytesExt as _;
use std::io::Read;

pub const INSTRUCTION_SET_VERSION: u32 = 0;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("supported instruction set version is {INSTRUCTION_SET_VERSION}, but got {version}")]
    UnsupportedInstructionSetVersion { version: u32 },

    #[error("expected a {expected:?} arg, but got {actual:?}")]
    UnexpectedArg {
        expected: &'static str,
        actual: CompactTerm,
    },

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl ParseError {
    fn unexpected_arg(expected: &'static str, actual: CompactTerm) -> Self {
        Self::UnexpectedArg { expected, actual }
    }
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
        let op = Op::decode(&mut reader)?;
        dbg!(op);
    }
    todo!()
}

// https://blog.stenmans.org/theBeamBook/#SEC-BeamModulesCTE
pub fn decode_compact_term() {}

pub type DecodeError = ParseError;

#[derive(Debug, Clone)]
pub enum CompactTerm {
    Literal(Literal),
    Todo,
}

impl CompactTerm {
    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        match tag & 0b111 {
            0b000 => {
                let index = (tag >> 4) as usize;
                Ok(Self::Literal(Literal { index }))
            }
            0b001 => {
                todo!();
            }
            0b010 => {
                todo!();
            }
            0b011 => {
                todo!();
            }
            0b100 => {
                todo!();
            }
            0b101 => {
                todo!()
            }
            0b110 => {
                todo!();
            }
            _ => {
                todo!();
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Literal {
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct LabelOp {
    pub literal: Literal,
}

impl LabelOp {
    pub const CODE: u8 = 1;
    pub const ARITY: usize = 1;

    pub fn decode_args<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        match CompactTerm::decode(reader)? {
            CompactTerm::Literal(literal) => Ok(Self { literal }),
            term => Err(DecodeError::unexpected_arg("literal", term)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Label(LabelOp),
}

impl Op {
    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        match reader.read_u8()? {
            LabelOp::CODE => LabelOp::decode_args(reader).map(Self::Label),
            op => todo!("{op}"),
        }
    }

    pub fn opcode(&self) -> u8 {
        match self {
            Self::Label { .. } => LabelOp::CODE,
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Self::Label { .. } => LabelOp::ARITY,
        }
    }
}
