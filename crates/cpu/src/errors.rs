//! Gejmboj CPU related errors

use std::{error::Error, fmt::Display};

use crate::registers::SingleRegister;

#[derive(Debug, PartialEq, Eq)]
pub enum CpuError {
    Error(String),
    UnsupportedSingleRegister(SingleRegister),
    UnknownInstruction(u8),
    SingleRegisterParseError(u8),
    MemoryExceeded { size: usize, max: usize },
}

impl Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error(msg) => write!(f, "Something went wrong: {msg}"),
            Self::UnknownInstruction(opcode) => write!(f, "Unknown opcode: {opcode:08b}"),
            Self::UnsupportedSingleRegister(register) => {
                write!(f, "Instruction does not support register {register:?}")
            }
            Self::SingleRegisterParseError(x) => {
                write!(f, "No single register matching {x:08b}")
            }
            Self::MemoryExceeded { size, max } => {
                write!(
                    f,
                    "Memory exceeded, actual size: {size:04X}, max: {max:04X}"
                )
            }
        }
    }
}

impl Error for CpuError {}
