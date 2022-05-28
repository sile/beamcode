//! BEAM instructions.
//!
//! # References
//!
//! - [The BEAM Book - Generic Instructions](https://blog.stenmans.org/theBeamBook/#_generic_instructions)
//! - [erlang/otp/lib/compiler/src/genop.tab](https://github.com/erlang/otp/blob/master/lib/compiler/src/genop.tab)
//! - erlang/otp/lib/compiler/src/beam_opcodes.erl (generated file)
use crate::term::{self, Allocation, Atom, List, Register, Term, YRegister};
use crate::{Decode, Encode};
use beamop_derive::Opcode;

pub trait Opcode {
    const CODE: u8;
}

#[derive(Debug, Clone, Decode, Encode)]
pub enum Instruction {
    Allocate(Allocate),
    AllocateHeap(AllocateHeap),
    AllocateHeapZero(AllocateHeapZero),
    AllocateZero(AllocateZero),
    Apply(Apply),
    ApplyLast(ApplyLast),
    Badmatch(Badmatch),
    Badrecord(Badrecord),
    Bif0(Bif0),
    Bif1(Bif1),
    Bif2(Bif2),
    BsAdd(BsAdd),
    BsAppend(BsAppend),
    /// Deprecated.
    BsBitsToBytes(BsBitsToBytes),
    /// Deprecated.
    BsBitsToBytes2(BsBitsToBytes2),
    BsContextToBinary(BsContextToBinary),
    BsCreateBin(BsCreateBin),
    /// Deprecated.
    BsFinal(BsFinal),
    /// Deprecated.
    BsFinal2(BsFinal2),
    /// Deprecated.
    BsGetBinary(BsGetBinary),
    BsGetBinary2(BsGetBinary2),
    /// Deprecated.
    BsGetFloat(BsGetFloat),
    BsGetFloat2(BsGetFloat2),
    /// Deprecated.
    BsGetInteger(BsGetInteger),
    BsGetInteger2(BsGetInteger2),
    BsGetPosition(BsGetPosition),
    BsGetTail(BsGetTail),
    BsGetUtf16(BsGetUtf16),
    BsGetUtf32(BsGetUtf32),
    BsGetUtf8(BsGetUtf8),
    /// Deprecated.
    BsInit(BsInit),
    BsInit2(BsInit2),
    BsInitBits(BsInitBits),
    BsInitWritable(BsInitWritable),
    BsMatchString(BsMatchString),
    BsNeedBuf(BsNeedBuf),
    BsPrivateAppend(BsPrivateAppend),
    BsPutBinary(BsPutBinary),
    BsPutFloat(BsPutFloat),
    BsPutInteger(BsPutInteger),
    BsPutString(BsPutString),
    BsPutUtf32(BsPutUtf32),
    BsPutUtf16(BsPutUtf16),
    BsPutUtf8(BsPutUtf8),
    /// Deprecated.
    BsRestore(BsRestore),
    BsRestore2(BsRestore2),
    BsSave(BsSave),
    BsSave2(BsSave2),
    BsSetPosition(BsSetPosition),
    BsSkipBits(BsSkipBits),
    BsSkipBits2(BsSkipBits2),
    BsSkipUtf32(BsSkipUtf32),
    BsSkipUtf16(BsSkipUtf16),
    BsSkipUtf8(BsSkipUtf8),
    /// Deprecated.
    BsStartMatch(BsStartMatch),
    BsStartMatch2(BsStartMatch2),
    BsStartMatch3(BsStartMatch3),
    BsStartMatch4(BsStartMatch4),
    /// Deprecated.
    BsTestTail(BsTestTail),
    BsTestTail2(BsTestTail2),
    BsTestUnit(BsTestUnit),
    BsUtf16Size(BsUtf16Size),
    BsUtf8Size(BsUtf8Size),
    BuildStacktrace(BuildStacktrace),
    Call(Call),
    CallExt(CallExt),
    CallExtLast(CallExtLast),
    CallExtOnly(CallExtOnly),
    CallFun(CallFun),
    CallFun2(CallFun2),
    CallLast(CallLast),
    CallOnly(CallOnly),
    CaseEnd(CaseEnd),
    Catch(Catch),
    CatchEnd(CatchEnd),
    Deallocate(Deallocate),
    Fadd(Fadd),
    Fcheckerror(Fcheckerror),
    Fclearerror(Fclearerror),
    Fconv(Fconv),
    Fdiv(Fdiv),
    Fmove(Fmove),
    Fmul(Fmul),
    Fnegate(Fnegate),
    Fsub(Fsub),
    FuncInfo(FuncInfo),
    GcBif1(GcBif1),
    GcBif2(GcBif2),
    GcBif3(GcBif3),
    GetHd(GetHd),
    GetList(GetList),
    GetMapElement(GetMapElement),
    GetTl(GetTl),
    GetTupleElement(GetTupleElement),
    HasMapFields(HasMapFields),
    IfEnd(IfEnd),
    Init(Init),
    InitYregs(InitYregs),
    /// Deprecated.
    IntBand(IntBand),
    /// Deprecated.
    IntBnot(IntBnot),
    /// Deprecated.
    IntBor(IntBor),
    /// Deprecated.
    IntBsl(IntBsl),
    /// Deprecated.
    IntBsr(IntBsr),
    /// Deprecated.
    IntBxor(IntBxor),
    IntCodeEnd(IntCodeEnd),
    /// Deprecated.
    IntDiv(IntDiv),
    /// Deprecated.
    IntRem(IntRem),
    IsAtom(IsAtom),
    IsBinary(IsBinary),
    IsBitstr(IsBitstr),
    IsBoolean(IsBoolean),
    /// Deprecated.
    IsConstant(IsConstant),
    IsEq(IsEq),
    IsEqExact(IsEqExact),
    IsFloat(IsFloat),
    IsFunction(IsFunction),
    IsFunction2(IsFunction2),
    IsGe(IsGe),
    IsInteger(IsInteger),
    IsList(IsList),
    IsLt(IsLt),
    IsMap(IsMap),
    IsNe(IsNe),
    IsNeExact(IsNeExact),
    IsNil(IsNil),
    IsNonemptyList(IsNonemptyList),
    IsNumber(IsNumber),
    IsPid(IsPid),
    IsPort(IsPort),
    IsReference(IsReference),
    IsTaggedTuple(IsTaggedTuple),
    IsTuple(IsTuple),
    Jump(Jump),
    Label(Label),
    Line(Line),
    LoopRec(LoopRec),
    LoopRecEnd(LoopRecEnd),
    /// Deprecated.
    MakeFun(MakeFun),
    MakeFun2(MakeFun2),
    MakeFun3(MakeFun3),
    /// Deprecated.
    MDiv(MDiv),
    Move(Move),
    /// Deprecated.
    MPlus(MPlus),
    /// Deprecated.
    MMinus(MMinus),
    /// Deprecated.
    MTimes(MTimes),
    NifStart(NifStart),
    OnLoad(OnLoad),
    Put(Put),
    PutList(PutList),
    /// Deprecated.
    PutLiteral(PutLiteral),
    PutMapAssoc(PutMapAssoc),
    PutMapExact(PutMapExact),
    /// Deprecated.
    PutString(PutString),
    PutTuple(PutTuple),
    PutTuple2(PutTuple2),
    Raise(Raise),
    RawRaise(RawRaise),
    RecvMark(RecvMark),
    RecvMarkerBind(RecvMarkerBind),
    RecvMarkerClear(RecvMarkerClear),
    RecvMarkerReserve(RecvMarkerReserve),
    RecvMarkerUse(RecvMarkerUse),
    RecvSet(RecvSet),
    Return(Return),
    RemoveMessage(RemoveMessage),
    SelectTupleArity(SelectTupleArity),
    SelectVal(SelectVal),
    Send(Send),
    SetTupleElement(SetTupleElement),
    Swap(Swap),
    TestArity(TestArity),
    TestHeap(TestHeap),
    Timeout(Timeout),
    Trim(Trim),
    Try(Try),
    TryCase(TryCase),
    TryCaseEnd(TryCaseEnd),
    TryEnd(TryEnd),
    Wait(Wait),
    WaitTimeout(WaitTimeout),
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(1)]
pub struct Label {
    pub literal: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(2)]
pub struct FuncInfo {
    pub module: Atom,
    pub function: Atom,
    pub arity: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(3)]
pub struct IntCodeEnd {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(4)]
pub struct Call {
    pub arity: usize,
    pub label: term::Label,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(5)]
pub struct CallLast {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(6)]
pub struct CallOnly {
    pub arity: usize,
    pub label: term::Label,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(7)]
pub struct CallExt {
    pub arity: usize,
    pub destination: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(8)]
pub struct CallExtLast {
    pub arity: usize,
    pub destination: usize,
    pub deallocate: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(9)]
pub struct Bif0 {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(10)]
pub struct Bif1 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(11)]
pub struct Bif2 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(12)]
pub struct Allocate {
    pub stack_need: Allocation,
    pub live: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(13)]
pub struct AllocateHeap {
    pub stack_need: Allocation,
    pub heap_need: Allocation,
    pub live: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(14)]
pub struct AllocateZero {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(15)]
pub struct AllocateHeapZero {
    pub stack_need: Allocation,
    pub heap_need: Allocation,
    pub live: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(16)]
pub struct TestHeap {
    pub heap_need: Allocation,
    pub live: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(17)]
pub struct Init {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(18)]
pub struct Deallocate {
    pub n: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(19)]
pub struct Return {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(20)]
pub struct Send {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(21)]
pub struct RemoveMessage {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(22)]
pub struct Timeout {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(23)]
pub struct LoopRec {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(24)]
pub struct LoopRecEnd {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(25)]
pub struct Wait {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(26)]
pub struct WaitTimeout {
    pub arg1: Term,
    pub arg2: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(27)]
pub struct MPlus {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(28)]
pub struct MMinus {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(29)]
pub struct MTimes {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(30)]
pub struct MDiv {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(31)]
pub struct IntDiv {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(32)]
pub struct IntRem {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(33)]
pub struct IntBand {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(34)]
pub struct IntBor {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(35)]
pub struct IntBxor {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(36)]
pub struct IntBsl {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(37)]
pub struct IntBsr {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(38)]
pub struct IntBnot {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(39)]
pub struct IsLt {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(40)]
pub struct IsGe {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(41)]
pub struct IsEq {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(42)]
pub struct IsNe {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(43)]
pub struct IsEqExact {
    pub label: term::Label,
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(44)]
pub struct IsNeExact {
    pub label: term::Label,
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(45)]
pub struct IsInteger {
    pub label: term::Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(46)]
pub struct IsFloat {
    pub label: term::Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(47)]
pub struct IsNumber {
    pub label: term::Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(48)]
pub struct IsAtom {
    pub label: term::Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(49)]
pub struct IsPid {
    pub label: term::Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(50)]
pub struct IsReference {
    pub label: term::Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(51)]
pub struct IsPort {
    pub label: term::Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(52)]
pub struct IsNil {
    pub label: term::Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(53)]
pub struct IsBinary {
    pub label: term::Label,
    pub arg1: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(54)]
pub struct IsConstant {
    pub label: term::Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(55)]
pub struct IsList {
    pub label: term::Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(56)]
pub struct IsNonemptyList {
    pub label: term::Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(57)]
pub struct IsTuple {
    pub label: term::Label,
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(58)]
pub struct TestArity {
    pub label: term::Label,
    pub arg1: Term,
    pub arity: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(59)]
pub struct SelectVal {
    pub arg: Term,
    pub fail_label: term::Label,
    pub destinations: List,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(60)]
pub struct SelectTupleArity {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(61)]
pub struct Jump {
    pub label: term::Label,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(62)]
pub struct Catch {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(63)]
pub struct CatchEnd {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(64)]
pub struct Move {
    pub src: Term,
    pub dst: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(65)]
pub struct GetList {
    pub source: Term,
    pub head: Register,
    pub tail: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(66)]
pub struct GetTupleElement {
    pub source: Register,
    pub element: usize,
    pub destination: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(67)]
pub struct SetTupleElement {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(68)]
pub struct PutString {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(69)]
pub struct PutList {
    pub head: Term,
    pub tail: Term,
    pub destination: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(70)]
pub struct PutTuple {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(71)]
pub struct Put {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(72)]
pub struct Badmatch {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(73)]
pub struct IfEnd {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(74)]
pub struct CaseEnd {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(75)]
pub struct CallFun {
    pub arg1: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(76)]
pub struct MakeFun {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(77)]
pub struct IsFunction {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(78)]
pub struct CallExtOnly {
    pub arity: usize,
    pub destination: usize,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(79)]
pub struct BsStartMatch {
    pub arg1: Term,
    pub arg2: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(80)]
pub struct BsGetInteger {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(81)]
pub struct BsGetFloat {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(82)]
pub struct BsGetBinary {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(83)]
pub struct BsSkipBits {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(84)]
pub struct BsTestTail {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(85)]
pub struct BsSave {
    pub arg1: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(86)]
pub struct BsRestore {
    pub arg1: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(87)]
pub struct BsInit {
    pub arg1: Term,
    pub arg2: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(88)]
pub struct BsFinal {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(89)]
pub struct BsPutInteger {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(90)]
pub struct BsPutBinary {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(91)]
pub struct BsPutFloat {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(92)]
pub struct BsPutString {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(93)]
pub struct BsNeedBuf {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(94)]
pub struct Fclearerror {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(95)]
pub struct Fcheckerror {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(96)]
pub struct Fmove {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(97)]
pub struct Fconv {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(98)]
pub struct Fadd {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(99)]
pub struct Fsub {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(100)]
pub struct Fmul {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(101)]
pub struct Fdiv {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(102)]
pub struct Fnegate {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(103)]
pub struct MakeFun2 {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(104)]
pub struct Try {
    pub register: Register,
    pub label: term::Label,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(105)]
pub struct TryEnd {
    pub register: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(106)]
pub struct TryCase {
    pub register: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(107)]
pub struct TryCaseEnd {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(108)]
pub struct Raise {
    pub stacktrace: Term,
    pub exc_value: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(109)]
pub struct BsInit2 {
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
pub struct BsBitsToBytes {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(111)]
pub struct BsAdd {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(112)]
pub struct Apply {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(113)]
pub struct ApplyLast {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(114)]
pub struct IsBoolean {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(115)]
pub struct IsFunction2 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(116)]
pub struct BsStartMatch2 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(117)]
pub struct BsGetInteger2 {
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
pub struct BsGetFloat2 {
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
pub struct BsGetBinary2 {
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
pub struct BsSkipBits2 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(121)]
pub struct BsTestTail2 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(122)]
pub struct BsSave2 {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(123)]
pub struct BsRestore2 {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(124)]
pub struct GcBif1 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(125)]
pub struct GcBif2 {
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
pub struct BsFinal2 {
    pub arg1: Term,
    pub arg2: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(127)]
pub struct BsBitsToBytes2 {
    pub arg1: Term,
    pub arg2: Term,
}

/// Deprecated.
#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(128)]
pub struct PutLiteral {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(129)]
pub struct IsBitstr {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(130)]
pub struct BsContextToBinary {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(131)]
pub struct BsTestUnit {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(132)]
pub struct BsMatchString {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(133)]
pub struct BsInitWritable {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(134)]
pub struct BsAppend {
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
pub struct BsPrivateAppend {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(136)]
pub struct Trim {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(137)]
pub struct BsInitBits {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(138)]
pub struct BsGetUtf8 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(139)]
pub struct BsSkipUtf8 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(140)]
pub struct BsGetUtf16 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(141)]
pub struct BsSkipUtf16 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(142)]
pub struct BsGetUtf32 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(143)]
pub struct BsSkipUtf32 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(144)]
pub struct BsUtf8Size {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(145)]
pub struct BsPutUtf8 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(146)]
pub struct BsUtf16Size {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(147)]
pub struct BsPutUtf16 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(148)]
pub struct BsPutUtf32 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(149)]
pub struct OnLoad {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(150)]
pub struct RecvMark {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(151)]
pub struct RecvSet {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(152)]
pub struct GcBif3 {
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
pub struct Line {
    pub literal: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(154)]
pub struct PutMapAssoc {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(155)]
pub struct PutMapExact {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(156)]
pub struct IsMap {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(157)]
pub struct HasMapFields {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(158)]
pub struct GetMapElement {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(159)]
pub struct IsTaggedTuple {
    pub label: term::Label,
    pub register: Register,
    pub arity: usize,
    pub atom: Atom,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(160)]
pub struct BuildStacktrace {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(161)]
pub struct RawRaise {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(162)]
pub struct GetHd {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(163)]
pub struct GetTl {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(164)]
pub struct PutTuple2 {
    pub destination: Register,
    pub elements: List,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(165)]
pub struct BsGetTail {
    pub context: Term,
    pub destination: Register,
    pub live: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(166)]
pub struct BsStartMatch3 {
    pub fail: term::Label,
    pub bin: Term,
    pub live: usize,
    pub destination: Register,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(167)]
pub struct BsGetPosition {
    pub context: Term,
    pub destination: Register,
    pub live: usize,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(168)]
pub struct BsSetPosition {
    pub context: Term,
    pub position: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(169)]
pub struct Swap {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(170)]
pub struct BsStartMatch4 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(171)]
pub struct MakeFun3 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(172)]
pub struct InitYregs {
    pub registers: Vec<YRegister>,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(173)]
pub struct RecvMarkerBind {
    pub arg1: Term,
    pub arg2: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(174)]
pub struct RecvMarkerClear {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(175)]
pub struct RecvMarkerReserve {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(176)]
pub struct RecvMarkerUse {
    pub arg1: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(177)]
pub struct BsCreateBin {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
    pub arg4: Term,
    pub arg5: Term,
    pub arg6: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(178)]
pub struct CallFun2 {
    pub arg1: Term,
    pub arg2: Term,
    pub arg3: Term,
}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(179)]
pub struct NifStart {}

#[derive(Debug, Clone, Opcode, Decode, Encode)]
#[opcode(180)]
pub struct Badrecord {
    pub arg1: Term,
}
