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

    #[error("unknown compact term tag: {tag}")]
    UnknownCompactTermTag { tag: u8 },

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
pub struct ValueAndLabel {
    pub value: CompactTerm,
    pub label: Label,
}

#[derive(Debug, Clone)]
pub enum CompactTerm {
    Literal(Literal),
    Atom(Atom),
    XRegister(XRegister),
    YRegister(YRegister),
    Label(Label),
    List(List),
    Todo,
}

impl CompactTerm {
    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        match tag & 0b111 {
            0b000 => {
                if (tag & 0b1000) != 0 {
                    todo!();
                }
                let index = (tag >> 4) as usize;
                Ok(Self::Literal(Literal { index }))
            }
            0b001 => {
                todo!();
            }
            0b010 => {
                if (tag & 0b1000) != 0 {
                    todo!();
                }
                let index = (tag >> 4) as usize;
                Ok(Self::Atom(Atom { index }))
            }
            0b011 => {
                if (tag & 0b1000) != 0 {
                    todo!();
                }
                let index = (tag >> 4) as usize;
                Ok(Self::XRegister(XRegister { index }))
            }
            0b100 => {
                if (tag & 0b1000) != 0 {
                    todo!();
                }
                let index = (tag >> 4) as usize;
                Ok(Self::YRegister(YRegister { index }))
            }
            0b101 => {
                if (tag & 0b1000) != 0 {
                    todo!();
                }
                let index = (tag >> 4) as usize;
                Ok(Self::Label(Label { index }))
            }
            0b110 => {
                todo!();
            }
            _ => match tag >> 3 {
                0b00010 => {
                    let size: Literal = Self::decode(reader)?.try_into()?;
                    let mut elements = Vec::with_capacity(size.index);
                    for _ in 0..size.index {
                        elements.push(Self::decode(reader)?);
                    }
                    Ok(Self::List(List { elements }))
                }
                0b00100 => {
                    todo!("floating point register");
                }
                0b00110 => {
                    todo!("allocation list");
                }
                0b01000 => {
                    todo!("literal");
                }
                _ => Err(DecodeError::UnknownCompactTermTag { tag }),
            },
        }
    }
}

impl TryFrom<CompactTerm> for Literal {
    type Error = DecodeError;

    fn try_from(x: CompactTerm) -> Result<Self, Self::Error> {
        match x {
            CompactTerm::Literal(x) => Ok(x),
            x => Err(DecodeError::unexpected_arg("literal", x)),
        }
    }
}

impl TryFrom<CompactTerm> for Atom {
    type Error = DecodeError;

    fn try_from(x: CompactTerm) -> Result<Self, Self::Error> {
        match x {
            CompactTerm::Atom(x) => Ok(x),
            x => Err(DecodeError::unexpected_arg("atom", x)),
        }
    }
}

impl TryFrom<CompactTerm> for XRegister {
    type Error = DecodeError;

    fn try_from(x: CompactTerm) -> Result<Self, Self::Error> {
        match x {
            CompactTerm::XRegister(x) => Ok(x),
            x => Err(DecodeError::unexpected_arg("x-register", x)),
        }
    }
}

impl TryFrom<CompactTerm> for YRegister {
    type Error = DecodeError;

    fn try_from(x: CompactTerm) -> Result<Self, Self::Error> {
        match x {
            CompactTerm::YRegister(x) => Ok(x),
            x => Err(DecodeError::unexpected_arg("y-register", x)),
        }
    }
}

impl TryFrom<CompactTerm> for Label {
    type Error = DecodeError;

    fn try_from(x: CompactTerm) -> Result<Self, Self::Error> {
        match x {
            CompactTerm::Label(x) => Ok(x),
            x => Err(DecodeError::unexpected_arg("label", x)),
        }
    }
}

impl TryFrom<CompactTerm> for List {
    type Error = DecodeError;

    fn try_from(x: CompactTerm) -> Result<Self, Self::Error> {
        match x {
            CompactTerm::List(x) => Ok(x),
            x => Err(DecodeError::unexpected_arg("list", x)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Literal {
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Atom {
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct XRegister {
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct YRegister {
    pub index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label {
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct List {
    pub elements: Vec<CompactTerm>,
}

#[derive(Debug, Clone)]
pub struct LabelOp {
    pub literal: Literal,
}

impl LabelOp {
    pub const CODE: u8 = 1;

    pub fn decode_args<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let literal = CompactTerm::decode(reader)?.try_into()?;
        Ok(Self { literal })
    }
}

#[derive(Debug, Clone)]
pub struct FuncInfoOp {
    pub module: Atom,
    pub function: Atom,
    pub arity: Literal,
}

impl FuncInfoOp {
    pub const CODE: u8 = 2;

    pub fn decode_args<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let module = CompactTerm::decode(reader)?.try_into()?;
        let function = CompactTerm::decode(reader)?.try_into()?;
        let arity = CompactTerm::decode(reader)?.try_into()?;
        Ok(Self {
            module,
            function,
            arity,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CallOnlyOp {
    pub arity: Literal,
    pub label: Label,
}

impl CallOnlyOp {
    pub const CODE: u8 = 6;

    pub fn decode_args<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let arity = CompactTerm::decode(reader)?.try_into()?;
        let label = CompactTerm::decode(reader)?.try_into()?;
        Ok(Self { arity, label })
    }
}

#[derive(Debug, Clone)]
pub struct AllocateOp {
    pub stack_need: Literal,
    pub live: Literal,
}

impl AllocateOp {
    pub const CODE: u8 = 12;

    pub fn decode_args<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let stack_need = CompactTerm::decode(reader)?.try_into()?;
        let live = CompactTerm::decode(reader)?.try_into()?;
        Ok(Self { stack_need, live })
    }
}

#[derive(Debug, Clone)]
pub struct MoveOp {
    pub src: Atom,
    pub dst: XRegister,
}

impl MoveOp {
    pub const CODE: u8 = 64;

    pub fn decode_args<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let src = CompactTerm::decode(reader)?.try_into()?;
        let dst = CompactTerm::decode(reader)?.try_into()?;
        Ok(Self { src, dst })
    }
}

#[derive(Debug, Clone)]
pub struct TryOp {
    pub register: YRegister,
    pub label: Label,
}

impl TryOp {
    pub const CODE: u8 = 104;

    pub fn decode_args<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let register = CompactTerm::decode(reader)?.try_into()?;
        let label = CompactTerm::decode(reader)?.try_into()?;
        Ok(Self { register, label })
    }
}

#[derive(Debug, Clone)]
pub struct LineOp {
    pub literal: Literal,
}

impl LineOp {
    pub const CODE: u8 = 153;

    pub fn decode_args<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let literal = CompactTerm::decode(reader)?.try_into()?;
        Ok(Self { literal })
    }
}

#[derive(Debug, Clone)]
pub struct InitYregsOp {
    pub registers: Vec<YRegister>,
}

impl InitYregsOp {
    pub const CODE: u8 = 172;

    pub fn decode_args<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let list: List = CompactTerm::decode(reader)?.try_into()?;
        let registers = list
            .elements
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<_, _>>()?;
        Ok(Self { registers })
    }
}

#[derive(Debug, Clone)]
pub enum Op {
    Label(LabelOp),
    FuncInfo(FuncInfoOp),
    CallOnly(CallOnlyOp),
    Allocate(AllocateOp),
    Move(MoveOp),
    Try(TryOp),
    Line(LineOp),
    InitYregs(InitYregsOp),
}

impl Op {
    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        match reader.read_u8()? {
            LabelOp::CODE => LabelOp::decode_args(reader).map(Self::Label),
            FuncInfoOp::CODE => FuncInfoOp::decode_args(reader).map(Self::FuncInfo),
            CallOnlyOp::CODE => CallOnlyOp::decode_args(reader).map(Self::CallOnly),
            AllocateOp::CODE => AllocateOp::decode_args(reader).map(Self::Allocate),
            MoveOp::CODE => MoveOp::decode_args(reader).map(Self::Move),
            TryOp::CODE => TryOp::decode_args(reader).map(Self::Try),
            LineOp::CODE => LineOp::decode_args(reader).map(Self::Line),
            InitYregsOp::CODE => InitYregsOp::decode_args(reader).map(Self::InitYregs),
            op => todo!("{op}"),
        }
    }

    pub fn opcode(&self) -> u8 {
        match self {
            Self::Label { .. } => LabelOp::CODE,
            Self::FuncInfo { .. } => FuncInfoOp::CODE,
            Self::CallOnly { .. } => CallOnlyOp::CODE,
            Self::Allocate { .. } => AllocateOp::CODE,
            Self::Try { .. } => TryOp::CODE,
            Self::Move { .. } => MoveOp::CODE,
            Self::Line { .. } => LineOp::CODE,
            Self::InitYregs { .. } => InitYregsOp::CODE,
        }
    }
}
