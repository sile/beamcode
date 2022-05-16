use crate::DecodeError;
use byteorder::ReadBytesExt;
use std::io::Read;

// From beam_opcodes.hrl file.
const TAG_U: u8 = 0; // Literal
const TAG_I: u8 = 1; // Integer
const TAG_A: u8 = 2; // Atom
const TAG_X: u8 = 3; // X regsiter
const TAG_Y: u8 = 4; // Y register
const TAG_F: u8 = 5; // Label
const TAG_H: u8 = 6; // Character
const TAG_Z: u8 = 7; // Extended

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Literal(Literal),
    Atom(Atom),
    XRegister(XRegister),
    YRegister(YRegister),
    Label(Label),
    List(List),
    // TODO: Integer (maybe a big-num)
    // TODO: Alloc List, etc
}

impl Term {
    // TODO: pub crate
    pub fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        let tag = reader.read_u8()?;
        match tag & 0b111 {
            TAG_U => Literal::decode(reader).map(Self::Literal),
            TAG_I => todo!(),
            TAG_A => todo!(),
            TAG_X => todo!(),
            TAG_Y => todo!(),
            TAG_F => todo!(),
            TAG_H => todo!(),
            TAG_Z => todo!(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Register {
    X(XRegister),
    Y(YRegister),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    XRegister(XRegister),
    YRegister(YRegister),
    Literal(Literal),
    Atom(Atom),
    // Integer
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Literal {
    pub value: usize,
}

impl Literal {
    fn decode<R: Read>(reader: &mut R) -> Result<Self, DecodeError> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Atom {
    pub value: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct XRegister {
    pub value: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct YRegister {
    pub value: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label {
    pub value: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct List {
    pub elements: Vec<Term>,
}
