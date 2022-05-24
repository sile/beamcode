use crate::terms::{Atom, Label, List, Literal, Register, Term, XRegister, YRegister};
use beam_file::chunk::CodeChunk;
use beamop_derive::{Decode, DecodeOperands, Opcode};
use byteorder::ReadBytesExt as _;
use std::io::Read;

// TODO: s/terms/term/
pub mod terms;

// TODO: pub mod op;

const INSTRUCTION_SET_VERSION: u32 = 0;

pub trait Decode: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError>;
}

// TODO: Remove this and use `Decode` instead.
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

// TODO: s/CodeChunk/impl Read/
pub fn parse_code_chunk(chunk: &CodeChunk) -> Result<Vec<Op>, ParseError> {
    if chunk.version != INSTRUCTION_SET_VERSION {
        return Err(ParseError::UnsupportedInstructionSetVersion {
            version: chunk.version,
        });
    }

    let mut ops = Vec::new();
    let mut reader = &mut &chunk.bytecode[..];
    while !reader.is_empty() {
        ops.push(Op::decode(&mut reader)?);
    }
    Ok(ops)
}

pub type DecodeError = ParseError;

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
#[opcode(3)]
pub struct IntCodeEndOp {}

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
#[opcode(59)]
pub struct SelectValOp {
    pub arg: Term,
    pub fail_label: Label,
    pub destinations: List, // TODO: AssocList
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(61)]
pub struct JumpOp {
    pub label: Label,
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
#[opcode(117)]
pub struct BsGetInteger2Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
    pub arg7: Term,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(119)]
pub struct BsGetBinary2Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
    pub arg7: Term,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(121)]
pub struct BsTestTail2Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(131)]
pub struct BsTestUnitOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
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
#[opcode(164)]
pub struct PutTuple2Op {
    pub destination: Register,
    pub elements: List,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(165)]
pub struct BsGetTailOp {
    pub context: Term,
    pub destination: Register,
    pub live: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(166)]
pub struct BsStartMatch3Op {
    pub fail: Label,
    pub bin: Term,
    pub live: Literal,
    pub destination: Register,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(167)]
pub struct BsGetPositionOp {
    pub context: Term,
    pub destination: Register,
    pub live: Literal,
}

#[derive(Debug, Clone, Opcode, DecodeOperands)]
#[opcode(168)]
pub struct BsSetPositionOp {
    pub context: Term,
    pub position: Term,
}

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
    BsGetBinary2(BsGetBinary2Op),
    BsGetInteger2(BsGetInteger2Op),
    BsGetPosition(BsGetPositionOp),
    BsGetTail(BsGetTailOp),
    BsSetPosition(BsSetPositionOp),
    BsStartMatch3(BsStartMatch3Op),
    BsTestTailp(BsTestTail2Op),
    BsTestUnit(BsTestUnitOp),
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
    IntCodeEnd(IntCodeEndOp),
    IsEqExact(IsEqExactOp),
    IsNil(IsNilOp),
    IsNonemptyList(IsNonemptyListOp),
    IsTaggedTuple(IsTaggedTupleOp),
    IsTuple(IsTupleOp),
    Jump(JumpOp),
    Label(LabelOp),
    Line(LineOp),
    Move(MoveOp),
    PutList(PutListOp),
    PutTuple2(PutTuple2Op),
    Raise(RaiseOp),
    Return(ReturnOp),
    SelectVal(SelectValOp),
    TestArity(TestArityOp),
    TestHeap(TestHeapOp),
    Try(TryOp),
    TryCase(TryCaseOp),
    TryEnd(TryEndOp),
}
