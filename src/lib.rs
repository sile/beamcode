use crate::terms::{Atom, Label, Literal, Register, Term, XRegister, YRegister};
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

impl From<std::convert::Infallible> for ParseError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
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

pub type DecodeError = ParseError;

#[derive(Debug, Clone)]
pub struct ValueAndLabel {
    pub value: Term,
    pub label: Label,
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
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(52)]
pub struct IsNilOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(56)]
pub struct IsNonemptyListOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(57)]
pub struct IsTupleOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(58)]
pub struct TestArityOp {
    pub label: Label,
    pub arg1: Term,
    pub arity: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(64)]
pub struct MoveOp {
    pub src: Term,
    pub dst: XRegister,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(65)]
pub struct GetListOp {
    pub source: Term,
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
    pub head: Term,
    pub tail: Term,
    pub destination: Register,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(72)]
pub struct BadmatchOp {
    pub arg1: Term, // TODO
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
    pub stacktrace: Term,
    pub exc_value: Term,
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
