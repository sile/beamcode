//! BEAM operations (a.k.a. instructions).
//!
//! # References
//!
//! - [The BEAM Book - Generic Instructions](https://blog.stenmans.org/theBeamBook/#_generic_instructions)
//! - [erlang/otp/lib/compiler/src/genop.tab](https://github.com/erlang/otp/blob/master/lib/compiler/src/genop.tab)
//! - erlang/otp/lib/compiler/src/beam_opcodes.erl (generated file)
use crate::term::{Allocation, Atom, Label, List, Register, Term, YRegister};
use crate::{Decode, Encode};
use beamop_derive::Opcode;

pub trait Opcode {
    const CODE: u8;
}

#[derive(Debug, Clone, Decode, Encode)]
pub enum Op {
    Allocate(AllocateOp),
    AllocateHeap(AllocateHeapOp),
    AllocateHeapZero(AllocateHeapZeroOp),
    AllocateZero(AllocateZeroOp),
    Apply(ApplyOp),
    ApplyLast(ApplyLastOp),
    Badmatch(BadmatchOp),
    Badrecord(BadrecordOp),
    Bif0(Bif0Op),
    Bif1(Bif1Op),
    Bif2(Bif2Op),
    BsAdd(BsAddOp),
    BsAppend(BsAppendOp),
    /// Deprecated.
    BsBitsToBytes(BsBitsToBytesOp),
    /// Deprecated.
    BsBitsToBytes2(BsBitsToBytes2Op),
    BsContextToBinary(BsContextToBinaryOp),
    BsCreateBin(BsCreateBinOp),
    /// Deprecated.
    BsFinal(BsFinalOp),
    /// Deprecated.
    BsFinal2(BsFinal2Op),
    /// Deprecated.
    BsGetBinary(BsGetBinaryOp),
    BsGetBinary2(BsGetBinary2Op),
    /// Deprecated.
    BsGetFloat(BsGetFloatOp),
    BsGetFloat2(BsGetFloat2Op),
    /// Deprecated.
    BsGetInteger(BsGetIntegerOp),
    BsGetInteger2(BsGetInteger2Op),
    BsGetPosition(BsGetPositionOp),
    BsGetTail(BsGetTailOp),
    BsGetUtf16(BsGetUtf16Op),
    BsGetUtf32(BsGetUtf32Op),
    BsGetUtf8(BsGetUtf8Op),
    /// Deprecated.
    BsInit(BsInitOp),
    BsInit2(BsInit2Op),
    BsInitBits(BsInitBitsOp),
    BsInitWritable(BsInitWritableOp),
    BsMatchString(BsMatchStringOp),
    BsNeedBuf(BsNeedBufOp),
    BsPrivateAppend(BsPrivateAppendOp),
    BsPutBinary(BsPutBinaryOp),
    BsPutFloat(BsPutFloatOp),
    BsPutInteger(BsPutIntegerOp),
    BsPutString(BsPutStringOp),
    BsPutUtf32(BsPutUtf32Op),
    BsPutUtf16(BsPutUtf16Op),
    BsPutUtf8(BsPutUtf8Op),
    /// Deprecated.
    BsRestore(BsRestoreOp),
    BsRestore2(BsRestore2Op),
    BsSave(BsSaveOp),
    BsSave2(BsSave2Op),
    BsSetPosition(BsSetPositionOp),
    BsSkipBits(BsSkipBitsOp),
    BsSkipBits2(BsSkipBits2Op),
    BsSkipUtf32(BsSkipUtf32Op),
    BsSkipUtf16(BsSkipUtf16Op),
    BsSkipUtf8(BsSkipUtf8Op),
    /// Deprecated.
    BsStartMatch(BsStartMatchOp),
    BsStartMatch2(BsStartMatch2Op),
    BsStartMatch3(BsStartMatch3Op),
    BsStartMatch4(BsStartMatch4Op),
    /// Deprecated.
    BsTestTail(BsTestTailOp),
    BsTestTail2(BsTestTail2Op),
    BsTestUnit(BsTestUnitOp),
    BsUtf16Size(BsUtf16SizeOp),
    BsUtf8Size(BsUtf8SizeOp),
    BuildStacktrace(BuildStacktraceOp),
    Call(CallOp),
    CallExt(CallExtOp),
    CallExtLast(CallExtLastOp),
    CallExtOnly(CallExtOnlyOp),
    CallFun(CallFunOp),
    CallFun2(CallFun2Op),
    CallLast(CallLastOp),
    CallOnly(CallOnlyOp),
    CaseEnd(CaseEndOp),
    Catch(CatchOp),
    CatchEnd(CatchEndOp),
    Deallocate(DeallocateOp),
    Fadd(FaddOp),
    Fcheckerror(FcheckerrorOp),
    Fclearerror(FclearerrorOp),
    Fconv(FconvOp),
    Fdiv(FdivOp),
    Fmove(FmoveOp),
    Fmul(FmulOp),
    Fnegate(FnegateOp),
    Fsub(FsubOp),
    FuncInfo(FuncInfoOp),
    GcBif1(GcBif1Op),
    GcBif2(GcBif2Op),
    GcBif3(GcBif3Op),
    GetHd(GetHdOp),
    GetList(GetListOp),
    GetMapElement(GetMapElementOp),
    GetTl(GetTlOp),
    GetTupleElement(GetTupleElementOp),
    HasMapFields(HasMapFieldsOp),
    IfEnd(IfEndOp),
    Init(InitOp),
    InitYregs(InitYregsOp),
    /// Deprecated.
    IntBand(IntBandOp),
    /// Deprecated.
    IntBnot(IntBnotOp),
    /// Deprecated.
    IntBor(IntBorOp),
    /// Deprecated.
    IntBsl(IntBslOp),
    /// Deprecated.
    IntBsr(IntBsrOp),
    /// Deprecated.
    IntBxor(IntBxorOp),
    IntCodeEnd(IntCodeEndOp),
    /// Deprecated.
    IntDiv(IntDivOp),
    /// Deprecated.
    IntRem(IntRemOp),
    IsAtom(IsAtomOp),
    IsBinary(IsBinaryOp),
    IsBitstr(IsBitstrOp),
    IsBoolean(IsBooleanOp),
    /// Deprecated.
    IsConstant(IsConstantOp),
    IsEq(IsEqOp),
    IsEqExact(IsEqExactOp),
    IsFloat(IsFloatOp),
    IsFunction(IsFunctionOp),
    IsFunction2(IsFunction2Op),
    IsGe(IsGeOp),
    IsInteger(IsIntegerOp),
    IsList(IsListOp),
    IsLt(IsLtOp),
    IsMap(IsMapOp),
    IsNe(IsNeOp),
    IsNeExact(IsNeExactOp),
    IsNil(IsNilOp),
    IsNonemptyList(IsNonemptyListOp),
    IsNumber(IsNumberOp),
    IsPid(IsPidOp),
    IsPort(IsPortOp),
    IsReference(IsReferenceOp),
    IsTaggedTuple(IsTaggedTupleOp),
    IsTuple(IsTupleOp),
    Jump(JumpOp),
    Label(LabelOp),
    Line(LineOp),
    LoopRec(LoopRecOp),
    LoopRecEnd(LoopRecEndOp),
    /// Deprecated.
    MakeFun(MakeFunOp),
    MakeFun2(MakeFun2Op),
    MakeFun3(MakeFun3Op),
    /// Deprecated.
    MDiv(MDivOp),
    Move(MoveOp),
    /// Deprecated.
    MPlus(MPlusOp),
    /// Deprecated.
    MMinus(MMinusOp),
    /// Deprecated.
    MTimes(MTimesOp),
    NifStart(NifStartOp),
    OnLoad(OnLoadOp),
    Put(PutOp),
    PutList(PutListOp),
    /// Deprecated.
    PutLiteral(PutLiteralOp),
    PutMapAssoc(PutMapAssocOp),
    PutMapExact(PutMapExactOp),
    /// Deprecated.
    PutString(PutStringOp),
    PutTuple(PutTupleOp),
    PutTuple2(PutTuple2Op),
    Raise(RaiseOp),
    RawRaise(RawRaiseOp),
    RecvMark(RecvMarkOp),
    RecvMarkerBind(RecvMarkerBindOp),
    RecvMarkerClear(RecvMarkerClearOp),
    RecvMarkerReserve(RecvMarkerReserveOp),
    RecvMarkerUse(RecvMarkerUseOp),
    RecvSet(RecvSetOp),
    Return(ReturnOp),
    RemoveMessage(RemoveMessageOp),
    SelectTupleArity(SelectTupleArityOp),
    SelectVal(SelectValOp),
    Send(SendOp),
    SetTupleElement(SetTupleElementOp),
    Swap(SwapOp),
    TestArity(TestArityOp),
    TestHeap(TestHeapOp),
    Timeout(TimeoutOp),
    Trim(TrimOp),
    Try(TryOp),
    TryCase(TryCaseOp),
    TryCaseEnd(TryCaseEndOp),
    TryEnd(TryEndOp),
    Wait(WaitOp),
    WaitTimeout(WaitTimeoutOp),
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(1)]
pub struct LabelOp {
    pub literal: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(2)]
pub struct FuncInfoOp {
    pub module: Atom,
    pub function: Atom,
    pub arity: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(3)]
pub struct IntCodeEndOp {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(4)]
pub struct CallOp {
    pub arity: usize,
    pub label: Label,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(5)]
pub struct CallLastOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(6)]
pub struct CallOnlyOp {
    pub arity: usize,
    pub label: Label,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(7)]
pub struct CallExtOp {
    pub arity: usize,
    pub destination: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(8)]
pub struct CallExtLastOp {
    pub arity: usize,
    pub destination: usize,
    pub deallocate: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(9)]
pub struct Bif0Op {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(10)]
pub struct Bif1Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(11)]
pub struct Bif2Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(12)]
pub struct AllocateOp {
    pub stack_need: Allocation,
    pub live: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(13)]
pub struct AllocateHeapOp {
    pub stack_need: Allocation,
    pub heap_need: Allocation,
    pub live: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(14)]
pub struct AllocateZeroOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(15)]
pub struct AllocateHeapZeroOp {
    pub stack_need: Allocation,
    pub heap_need: Allocation,
    pub live: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(16)]
pub struct TestHeapOp {
    pub heap_need: Allocation,
    pub live: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(17)]
pub struct InitOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(18)]
pub struct DeallocateOp {
    pub n: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(19)]
pub struct ReturnOp {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(20)]
pub struct SendOp {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(21)]
pub struct RemoveMessageOp {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(22)]
pub struct TimeoutOp {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(23)]
pub struct LoopRecOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(24)]
pub struct LoopRecEndOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(25)]
pub struct WaitOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(26)]
pub struct WaitTimeoutOp {
    pub arg1: Term,
    pub arg2: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(27)]
pub struct MPlusOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(28)]
pub struct MMinusOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(29)]
pub struct MTimesOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(30)]
pub struct MDivOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(31)]
pub struct IntDivOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(32)]
pub struct IntRemOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(33)]
pub struct IntBandOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(34)]
pub struct IntBorOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(35)]
pub struct IntBxorOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(36)]
pub struct IntBslOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(37)]
pub struct IntBsrOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(38)]
pub struct IntBnotOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(39)]
pub struct IsLtOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(40)]
pub struct IsGeOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(41)]
pub struct IsEqOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(42)]
pub struct IsNeOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(43)]
pub struct IsEqExactOp {
    pub label: Label,
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(44)]
pub struct IsNeExactOp {
    pub label: Label,
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(45)]
pub struct IsIntegerOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(46)]
pub struct IsFloatOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(47)]
pub struct IsNumberOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(48)]
pub struct IsAtomOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(49)]
pub struct IsPidOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(50)]
pub struct IsReferenceOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(51)]
pub struct IsPortOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(52)]
pub struct IsNilOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(53)]
pub struct IsBinaryOp {
    pub label: Label,
    pub arg1: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(54)]
pub struct IsConstantOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(55)]
pub struct IsListOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(56)]
pub struct IsNonemptyListOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(57)]
pub struct IsTupleOp {
    pub label: Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(58)]
pub struct TestArityOp {
    pub label: Label,
    pub arg1: Term,
    pub arity: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(59)]
pub struct SelectValOp {
    pub arg: Term,
    pub fail_label: Label,
    pub destinations: List,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(60)]
pub struct SelectTupleArityOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(61)]
pub struct JumpOp {
    pub label: Label,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(62)]
pub struct CatchOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(63)]
pub struct CatchEndOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(64)]
pub struct MoveOp {
    pub src: Term,
    pub dst: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(65)]
pub struct GetListOp {
    pub source: Term,
    pub head: Register,
    pub tail: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(66)]
pub struct GetTupleElementOp {
    pub source: Register,
    pub element: usize,
    pub destination: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(67)]
pub struct SetTupleElementOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(68)]
pub struct PutStringOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(69)]
pub struct PutListOp {
    pub head: Term,
    pub tail: Term,
    pub destination: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(70)]
pub struct PutTupleOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(71)]
pub struct PutOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(72)]
pub struct BadmatchOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(73)]
pub struct IfEndOp {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(74)]
pub struct CaseEndOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(75)]
pub struct CallFunOp {
    pub arg1: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(76)]
pub struct MakeFunOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(77)]
pub struct IsFunctionOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(78)]
pub struct CallExtOnlyOp {
    pub arity: usize,
    pub destination: usize,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(79)]
pub struct BsStartMatchOp {
    pub arg1: Term,
    pub arg2: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(80)]
pub struct BsGetIntegerOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(81)]
pub struct BsGetFloatOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(82)]
pub struct BsGetBinaryOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(83)]
pub struct BsSkipBitsOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(84)]
pub struct BsTestTailOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(85)]
pub struct BsSaveOp {
    pub arg1: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(86)]
pub struct BsRestoreOp {
    pub arg1: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(87)]
pub struct BsInitOp {
    pub arg1: Term,
    pub arg2: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(88)]
pub struct BsFinalOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(89)]
pub struct BsPutIntegerOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(90)]
pub struct BsPutBinaryOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(91)]
pub struct BsPutFloatOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(92)]
pub struct BsPutStringOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(93)]
pub struct BsNeedBufOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(94)]
pub struct FclearerrorOp {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(95)]
pub struct FcheckerrorOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(96)]
pub struct FmoveOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(97)]
pub struct FconvOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(98)]
pub struct FaddOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(99)]
pub struct FsubOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(100)]
pub struct FmulOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(101)]
pub struct FdivOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(102)]
pub struct FnegateOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(103)]
pub struct MakeFun2Op {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(104)]
pub struct TryOp {
    pub register: Register,
    pub label: Label,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(105)]
pub struct TryEndOp {
    pub register: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(106)]
pub struct TryCaseOp {
    pub register: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(107)]
pub struct TryCaseEndOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(108)]
pub struct RaiseOp {
    pub stacktrace: Term,
    pub exc_value: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(109)]
pub struct BsInit2Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(110)]
pub struct BsBitsToBytesOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(111)]
pub struct BsAddOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(112)]
pub struct ApplyOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(113)]
pub struct ApplyLastOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(114)]
pub struct IsBooleanOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(115)]
pub struct IsFunction2Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(116)]
pub struct BsStartMatch2Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
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

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(118)]
pub struct BsGetFloat2Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
    pub arg7: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
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

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(120)]
pub struct BsSkipBits2Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(121)]
pub struct BsTestTail2Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(122)]
pub struct BsSave2Op {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(123)]
pub struct BsRestore2Op {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(124)]
pub struct GcBif1Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(125)]
pub struct GcBif2Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(126)]
pub struct BsFinal2Op {
    pub arg1: Term,
    pub arg2: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(127)]
pub struct BsBitsToBytes2Op {
    pub arg1: Term,
    pub arg2: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(128)]
pub struct PutLiteralOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(129)]
pub struct IsBitstrOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(130)]
pub struct BsContextToBinaryOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(131)]
pub struct BsTestUnitOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(132)]
pub struct BsMatchStringOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(133)]
pub struct BsInitWritableOp {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(134)]
pub struct BsAppendOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
    pub arg7: Term,
    pub arg8: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(135)]
pub struct BsPrivateAppendOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(136)]
pub struct TrimOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(137)]
pub struct BsInitBitsOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(138)]
pub struct BsGetUtf8Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(139)]
pub struct BsSkipUtf8Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(140)]
pub struct BsGetUtf16Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(141)]
pub struct BsSkipUtf16Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(142)]
pub struct BsGetUtf32Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(143)]
pub struct BsSkipUtf32Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(144)]
pub struct BsUtf8SizeOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(145)]
pub struct BsPutUtf8Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(146)]
pub struct BsUtf16SizeOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(147)]
pub struct BsPutUtf16Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(148)]
pub struct BsPutUtf32Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(149)]
pub struct OnLoadOp {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(150)]
pub struct RecvMarkOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(151)]
pub struct RecvSetOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(152)]
pub struct GcBif3Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
    pub arg7: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(153)]
pub struct LineOp {
    pub literal: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(154)]
pub struct PutMapAssocOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(155)]
pub struct PutMapExactOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(156)]
pub struct IsMapOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(157)]
pub struct HasMapFieldsOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(158)]
pub struct GetMapElementOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(159)]
pub struct IsTaggedTupleOp {
    pub label: Label,
    pub register: Register,
    pub arity: usize,
    pub atom: Atom,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(160)]
pub struct BuildStacktraceOp {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(161)]
pub struct RawRaiseOp {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(162)]
pub struct GetHdOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(163)]
pub struct GetTlOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(164)]
pub struct PutTuple2Op {
    pub destination: Register,
    pub elements: List,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(165)]
pub struct BsGetTailOp {
    pub context: Term,
    pub destination: Register,
    pub live: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(166)]
pub struct BsStartMatch3Op {
    pub fail: Label,
    pub bin: Term,
    pub live: usize,
    pub destination: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(167)]
pub struct BsGetPositionOp {
    pub context: Term,
    pub destination: Register,
    pub live: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(168)]
pub struct BsSetPositionOp {
    pub context: Term,
    pub position: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(169)]
pub struct SwapOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(170)]
pub struct BsStartMatch4Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(171)]
pub struct MakeFun3Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(172)]
pub struct InitYregsOp {
    pub registers: Vec<YRegister>,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(173)]
pub struct RecvMarkerBindOp {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(174)]
pub struct RecvMarkerClearOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(175)]
pub struct RecvMarkerReserveOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(176)]
pub struct RecvMarkerUseOp {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(177)]
pub struct BsCreateBinOp {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(178)]
pub struct CallFun2Op {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(179)]
pub struct NifStartOp {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(180)]
pub struct BadrecordOp {
    pub arg1: Term,
}
