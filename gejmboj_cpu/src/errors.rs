//! Gejmboj CPU related errors

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CpuError {
    Error(String),
    UnknownInstruction(u8),
}

impl Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpuError::Error(msg) => write!(f, "Something went wrong: {}", msg),
            CpuError::UnknownInstruction(opcode) => write!(f, "Unknown opcode: {:08b}", opcode),
        }
    }
}

impl Error for CpuError {}
