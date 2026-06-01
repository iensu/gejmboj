//! Gejmboj CPU related errors

use std::{error::Error, fmt::Display};

use crate::registers::SingleRegister;

#[derive(Debug, PartialEq)]
pub enum CpuError {
    Error(String),
    UnsupportedSingleRegister(SingleRegister),
    UnknownInstruction(u8),
    SingleRegisterParseError(u8),
}

impl Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuError::Error(msg) => write!(f, "Something went wrong: {msg}"),
            CpuError::UnknownInstruction(opcode) => write!(f, "Unknown opcode: {opcode:08b}"),
            CpuError::UnsupportedSingleRegister(register) => {
                write!(f, "Instruction does not support register {register:?}")
            }
            CpuError::SingleRegisterParseError(x) => {
                write!(f, "No single register matching {x:08b}")
            }
        }
    }
}

impl Error for CpuError {}
