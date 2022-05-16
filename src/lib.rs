use beam_file::chunk::CodeChunk;
use beamop_derive::{Decode, DecodeOperands, Opcode};
use byteorder::ReadBytesExt as _;
use std::io::Read;

pub mod terms;

pub const INSTRUCTION_SET_VERSION: u32 = 0;

pub trait Decode: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>;
}

pub trait DecodeOperands: Sized {
    fn decode_operands<R: Read>(reader: &mut R) -> Result<Self, DecodeError>;
}

pub trait Opcode {
    const CODE: u8;
}

const USIZE_BYTES: u32 = usize::BITS / 8;

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

    #[error("unknown opcode: {opcode}")]
    UnknownOpcode { opcode: u8 },

    #[error("expected a usize value ({USIZE_BYTES} bytes), but got a {byte_size} bytes value")]
    TooLargeUsizeValue { byte_size: usize },

    #[error(transparent)]
    ConvertTermError(#[from] crate::terms::ConvertTermError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl From<std::convert::Infallible> for ParseError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
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
                if (tag & 0b1_000) == 0 {
                    let index = (tag >> 4) as usize;
                    Ok(Self::Literal(Literal { index }))
                } else if (tag & 0b10_000) == 0 {
                    let v = reader.read_u8()? as usize;
                    let index = (usize::from(tag & 0b111_00_000) << 3) | v;
                    Ok(Self::Literal(Literal { index }))
                } else {
                    todo!();
                }
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
                if (tag & 0b1_000) == 0 {
                    let index = (tag >> 4) as usize;
                    Ok(Self::Label(Label { index }))
                } else if (tag & 0b10_000) == 0 {
                    let v = reader.read_u8()? as usize;
                    let index = (usize::from(tag & 0b111_00_000) << 3) | v;
                    Ok(Self::Label(Label { index }))
                } else {
                    todo!();
                }
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

impl TryFrom<CompactTerm> for Register {
    type Error = DecodeError;

    fn try_from(x: CompactTerm) -> Result<Self, Self::Error> {
        match x {
            CompactTerm::XRegister(x) => Ok(Register::X(x)),
            CompactTerm::YRegister(x) => Ok(Register::Y(x)),
            x => Err(DecodeError::unexpected_arg("x-register or y-register", x)),
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

impl TryFrom<CompactTerm> for Vec<YRegister> {
    type Error = DecodeError;

    fn try_from(term: CompactTerm) -> Result<Self, Self::Error> {
        let list: List = term.try_into()?;
        list.elements
            .into_iter()
            .map(|x| x.try_into())
            .collect::<Result<_, _>>()
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
pub enum Register {
    X(XRegister),
    Y(YRegister),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label {
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct List {
    pub elements: Vec<CompactTerm>,
}

// TODO: check https://blog.stenmans.org/theBeamBook/#_list_of_all_beam_instructions
#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(1)]
pub struct LabelOp {
    pub literal: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(2)]
pub struct FuncInfoOp {
    pub module: Atom,
    pub function: Atom,
    pub arity: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(4)]
pub struct CallOp {
    pub arity: Literal,
    pub label: Label,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(6)]
pub struct CallOnlyOp {
    pub arity: Literal,
    pub label: Label,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(7)]
pub struct CallExtOp {
    pub arity: Literal,
    pub destination: Literal, // TODO: s/Literal/ImportTableIndex/
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(8)]
pub struct CallExtLastOp {
    pub arity: Literal,
    pub destination: Literal, // TODO: s/Literal/ImportTableIndex/
    pub deallocate: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(12)]
pub struct AllocateOp {
    pub stack_need: Literal,
    pub live: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(13)]
pub struct AllocateHeapOp {
    pub stack_need: Literal,
    pub heap_need: Literal,
    pub live: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(15)]
pub struct AllocateHeapZeroOp {
    pub stack_need: Literal,
    pub heap_need: Literal,
    pub live: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(16)]
pub struct TestHeapOp {
    pub heap_need: Literal,
    pub live: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(18)]
pub struct DeallocateOp {
    pub n: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(19)]
pub struct ReturnOp {}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(43)]
pub struct IsEqExactOp {
    pub label: Label,
    pub arg1: CompactTerm,
    pub arg2: CompactTerm,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(52)]
pub struct IsNilOp {
    pub label: Label,
    pub arg1: CompactTerm,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(56)]
pub struct IsNonemptyListOp {
    pub label: Label,
    pub arg1: CompactTerm,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(57)]
pub struct IsTupleOp {
    pub label: Label,
    pub arg1: CompactTerm,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(58)]
pub struct TestArityOp {
    pub label: Label,
    pub arg1: CompactTerm,
    pub arity: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(64)]
pub struct MoveOp {
    pub src: CompactTerm,
    pub dst: XRegister,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(65)]
pub struct GetListOp {
    pub source: CompactTerm,
    pub head: Register,
    pub tail: Register,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(66)]
pub struct GetTupleElementOp {
    pub source: Register,
    pub element: Literal,
    pub destination: Register,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(69)]
pub struct PutListOp {
    pub head: CompactTerm,
    pub tail: CompactTerm,
    pub destination: Register,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(72)]
pub struct BadmatchOp {
    pub arg1: CompactTerm, // TODO
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(78)]
pub struct CallExtOnlyOp {
    pub arity: Literal,
    pub destination: Literal, // TODO: s/Literal/ImportTableIndex/
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(104)]
pub struct TryOp {
    pub register: YRegister,
    pub label: Label,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(105)]
pub struct TryEndOp {
    pub register: YRegister,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(106)]
pub struct TryCaseOp {
    pub register: YRegister,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(108)]
pub struct RaiseOp {
    pub stacktrace: CompactTerm,
    pub exc_value: CompactTerm,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(153)]
pub struct LineOp {
    pub literal: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(159)]
pub struct IsTaggedTupleOp {
    pub label: Label,
    pub register: XRegister,
    pub arity: Literal,
    pub atom: Atom,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(160)]
pub struct BuildStacktraceOp {}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(172)]
pub struct InitYregsOp {
    pub registers: Vec<YRegister>,
}

#[derive(Debug, Clone, Decode)]
pub enum Op {
    Allocate(AllocateOp),
    AllocateHeap(AllocateHeapOp),
    AllocateHeapZero(AllocateHeapZeroOp),
    Badmatch(BadmatchOp),
    BuildStacktrace(BuildStacktraceOp),
    Call(CallOp),
    CallExt(CallExtOp),
    CallExtLast(CallExtLastOp),
    CallExtOnly(CallExtOnlyOp),
    CallOnly(CallOnlyOp),
    Deallocate(DeallocateOp),
    FuncInfo(FuncInfoOp),
    GetList(GetListOp),
    GetTupleElement(GetTupleElementOp),
    InitYregs(InitYregsOp),
    IsEqExact(IsEqExactOp),
    IsNil(IsNilOp),
    IsNonemptyList(IsNonemptyListOp),
    IsTaggedTuple(IsTaggedTupleOp),
    IsTuple(IsTupleOp),
    Label(LabelOp),
    Line(LineOp),
    Move(MoveOp),
    PutList(PutListOp),
    Raise(RaiseOp),
    Return(ReturnOp),
    TestArity(TestArityOp),
    TestHeap(TestHeapOp),
    Try(TryOp),
    TryCase(TryCaseOp),
    TryEnd(TryEndOp),
}
