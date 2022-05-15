use beam_file::chunk::CodeChunk;
use beamop_derive::DecodeOperands;
use byteorder::ReadBytesExt as _;
use std::io::Read;

pub const INSTRUCTION_SET_VERSION: u32 = 0;

pub trait DecodeOperands: Sized {
    fn decode_operands<R: Read>(reader: &mut R) -> Result<Self, DecodeError>;
}

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

#[derive(Debug, Clone, DecodeOperands)]
pub struct LabelOp {
    pub literal: Literal,
}

impl LabelOp {
    pub const CODE: u8 = 1;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct FuncInfoOp {
    pub module: Atom,
    pub function: Atom,
    pub arity: Literal,
}

impl FuncInfoOp {
    pub const CODE: u8 = 2;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct CallOp {
    pub arity: Literal,
    pub label: Label,
}

impl CallOp {
    pub const CODE: u8 = 4;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct CallOnlyOp {
    pub arity: Literal,
    pub label: Label,
}

impl CallOnlyOp {
    pub const CODE: u8 = 6;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct CallExtOp {
    pub arity: Literal,
    pub destination: Literal, // TODO: s/Literal/ImportTableIndex/
}

impl CallExtOp {
    pub const CODE: u8 = 7;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct AllocateOp {
    pub stack_need: Literal,
    pub live: Literal,
}

impl AllocateOp {
    pub const CODE: u8 = 12;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct DeallocateOp {
    pub n: Literal,
}

impl DeallocateOp {
    pub const CODE: u8 = 18;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct ReturnOp {}

impl ReturnOp {
    pub const CODE: u8 = 19;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct IsEqExactOp {
    pub label: Label,
    pub arg1: CompactTerm,
    pub arg2: CompactTerm,
}

impl IsEqExactOp {
    pub const CODE: u8 = 43;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct IsNonemptyListOp {
    pub label: Label,
    pub arg1: CompactTerm,
}

impl IsNonemptyListOp {
    pub const CODE: u8 = 56;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct IsTupleOp {
    pub label: Label,
    pub arg1: CompactTerm,
}

impl IsTupleOp {
    pub const CODE: u8 = 57;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct TestArityOp {
    pub label: Label,
    pub arg1: CompactTerm,
    pub arity: Literal,
}

impl TestArityOp {
    pub const CODE: u8 = 58;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct MoveOp {
    pub src: CompactTerm,
    pub dst: XRegister,
}

impl MoveOp {
    pub const CODE: u8 = 64;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct GetListOp {
    pub source: CompactTerm,
    pub head: Register,
    pub tail: Register,
}

impl GetListOp {
    pub const CODE: u8 = 65;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct GetTupleElementOp {
    pub source: Register,
    pub element: Literal,
    pub destination: Register,
}

impl GetTupleElementOp {
    pub const CODE: u8 = 66;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct TryOp {
    pub register: YRegister,
    pub label: Label,
}

impl TryOp {
    pub const CODE: u8 = 104;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct TryEndOp {
    pub register: YRegister,
}

impl TryEndOp {
    pub const CODE: u8 = 105;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct TryCaseOp {
    pub register: YRegister,
}

impl TryCaseOp {
    pub const CODE: u8 = 106;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct LineOp {
    pub literal: Literal,
}

impl LineOp {
    pub const CODE: u8 = 153;
}

#[derive(Debug, Clone, DecodeOperands)]
pub struct IsTaggedTupleOp {
    pub label: Label,
    pub register: XRegister,
    pub arity: Literal,
    pub atom: Atom,
}

impl IsTaggedTupleOp {
    pub const CODE: u8 = 159;
}

// TODO: move
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

#[derive(Debug, Clone, DecodeOperands)]
pub struct InitYregsOp {
    pub registers: Vec<YRegister>,
}

impl InitYregsOp {
    pub const CODE: u8 = 172;
}

#[derive(Debug, Clone)]
pub enum Op {
    Label(LabelOp),
    FuncInfo(FuncInfoOp),
    Call(CallOp),
    CallOnly(CallOnlyOp),
    CallExt(CallExtOp),
    Allocate(AllocateOp),
    Deallocate(DeallocateOp),
    Return(ReturnOp),
    IsEqExact(IsEqExactOp),
    IsNonemptyList(IsNonemptyListOp),
    IsTuple(IsTupleOp),
    TestArity(TestArityOp),
    Move(MoveOp),
    GetList(GetListOp),
    GetTupleElement(GetTupleElementOp),
    Try(TryOp),
    TryEnd(TryEndOp),
    TryCase(TryCaseOp),
    Line(LineOp),
    IsTaggedTuple(IsTaggedTupleOp),
    InitYregs(InitYregsOp),
}

impl Op {
    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        match reader.read_u8()? {
            LabelOp::CODE => DecodeOperands::decode_operands(reader).map(Self::Label),
            FuncInfoOp::CODE => DecodeOperands::decode_operands(reader).map(Self::FuncInfo),
            CallOp::CODE => DecodeOperands::decode_operands(reader).map(Self::Call),
            CallOnlyOp::CODE => DecodeOperands::decode_operands(reader).map(Self::CallOnly),
            CallExtOp::CODE => DecodeOperands::decode_operands(reader).map(Self::CallExt),
            AllocateOp::CODE => DecodeOperands::decode_operands(reader).map(Self::Allocate),
            DeallocateOp::CODE => DecodeOperands::decode_operands(reader).map(Self::Deallocate),
            ReturnOp::CODE => DecodeOperands::decode_operands(reader).map(Self::Return),
            IsEqExactOp::CODE => DecodeOperands::decode_operands(reader).map(Self::IsEqExact),
            IsNonemptyListOp::CODE => {
                DecodeOperands::decode_operands(reader).map(Self::IsNonemptyList)
            }
            IsTupleOp::CODE => DecodeOperands::decode_operands(reader).map(Self::IsTuple),
            TestArityOp::CODE => DecodeOperands::decode_operands(reader).map(Self::TestArity),
            MoveOp::CODE => DecodeOperands::decode_operands(reader).map(Self::Move),
            GetListOp::CODE => DecodeOperands::decode_operands(reader).map(Self::GetList),
            GetTupleElementOp::CODE => {
                DecodeOperands::decode_operands(reader).map(Self::GetTupleElement)
            }
            TryOp::CODE => DecodeOperands::decode_operands(reader).map(Self::Try),
            TryEndOp::CODE => DecodeOperands::decode_operands(reader).map(Self::TryEnd),
            TryCaseOp::CODE => DecodeOperands::decode_operands(reader).map(Self::TryCase),
            LineOp::CODE => DecodeOperands::decode_operands(reader).map(Self::Line),
            IsTaggedTupleOp::CODE => {
                DecodeOperands::decode_operands(reader).map(Self::IsTaggedTuple)
            }
            InitYregsOp::CODE => DecodeOperands::decode_operands(reader).map(Self::InitYregs),
            op => todo!("{op}"),
        }
    }
}
