//! Sharp SM83 instruction set

use crate::{cpu::CpuFlags, errors::CpuError, memory::Memory, registers::Registers};

mod control_flow;
mod load;
mod misc;

pub use control_flow::*;
pub use load::*;
pub use misc::*;

/// Return either the number of consumed machine cycles, or a `CpuError`.
pub type InstructionResult = Result<u16, CpuError>;

/// Trait for implementing a Sharp SM83 instruction.
pub trait Instruction {
    /// Execute the instruction and return an [InstructionResult](type.InstructionResult.html)
    fn execute(
        &self,
        registers: &mut Registers,
        memory: &mut Memory,
        cpu_flags: &mut CpuFlags,
    ) -> InstructionResult;

    /// Returns the byte length of the operation
    fn length(&self) -> u16;
}

#[derive(Debug)]
pub enum Condition {
    Carry,
    NoCarry,
    Zero,
    NotZero,
}

impl Condition {
    pub fn parse(c1: u8, c2: u8) -> Result<Self, CpuError> {
        match (c1, c2) {
            (0, 0) => Ok(Condition::Carry),
            (0, 1) => Ok(Condition::NoCarry),
            (1, 0) => Ok(Condition::Zero),
            (1, 1) => Ok(Condition::NotZero),
            _ => Err(CpuError::Error(format!(
                "Unknown instruction condition ({}, {})",
                c1, c2
            ))),
        }
    }

    pub fn is_fulfilled(&self, registers: &Registers) -> bool {
        match self {
            Condition::Carry => registers.is_carry(),
            Condition::NoCarry => !registers.is_carry(),
            Condition::Zero => registers.is_zero(),
            Condition::NotZero => !registers.is_zero(),
        }
    }
}

/// Decode an operation code into an `Instruction`.
pub fn decode(opcode: u8, pc: u16, memory: &Memory) -> Result<Box<dyn Instruction>, CpuError> {
    match into_bits(opcode) {
        (0, 0, 0, 0, 0, 0, 0, 0) => Ok(Box::new(misc::Noop {})),
        (1, 1, 0, 0, 0, 0, 1, 1) => Ok(Box::new(control_flow::Jp {
            operand: get_16bit_operand(pc, memory),
        })),
        (1, 1, 0, 0, 1, 0, 0, 1) => Ok(Box::new(control_flow::Ret {})),
        (1, 1, 0, 1, 1, 0, 0, 1) => Ok(Box::new(control_flow::RetI {})),
        (1, 1, 0, 0, 1, 1, 0, 1) => Ok(Box::new(control_flow::Call {
            operand: get_16bit_operand(pc, memory),
        })),
        (1, 1, 1, 0, 1, 0, 0, 1) => Ok(Box::new(control_flow::JpToHL {})),
        (0, 0, 0, 1, 1, 0, 0, 0) => Ok(Box::new(control_flow::JpToOffset {
            operand: get_8bit_operand(pc, memory),
        })),
        (1, 1, 0, c, d, 0, 1, 0) => Ok(Box::new(control_flow::JpIf {
            operand: get_16bit_operand(pc, memory),
            condition: Condition::parse(c, d).unwrap(),
        })),
        (0, 0, 1, c, d, 0, 0, 0) => Ok(Box::new(control_flow::JpToOffsetIf {
            operand: get_8bit_operand(pc, memory),
            condition: Condition::parse(c, d).unwrap(),
        })),
        (1, 1, 0, c, d, 1, 0, 0) => Ok(Box::new(control_flow::CallIf {
            operand: get_16bit_operand(pc, memory),
            condition: Condition::parse(c, d).unwrap(),
        })),
        (1, 1, 0, c, d, 0, 0, 0) => Ok(Box::new(control_flow::RetIf {
            condition: Condition::parse(c, d).unwrap(),
        })),
        (1, 1, _, _, _, 1, 1, 1) => Ok(Box::new(control_flow::Rst { opcode })),
        (0, 1, a, b, c, 1, 1, 0) => Ok(Box::new(load::LdFromHL {
            r: (a, b, c).into(),
        })),
        (0, 1, 1, 1, 0, a, b, c) => Ok(Box::new(load::LdToHL {
            r: (a, b, c).into(),
        })),
        (0, 1, a, b, c, x, y, z) => Ok(Box::new(load::Ld {
            r1: (a, b, c).into(),
            r2: (x, y, z).into(),
        })),
        _ => Err(CpuError::UnknownInstruction(opcode)),
    }
}

fn get_8bit_operand(pc: u16, memory: &Memory) -> u8 {
    memory.get((pc as usize) + 1)
}

fn get_16bit_operand(pc: u16, memory: &Memory) -> u16 {
    memory.get_u16((pc as usize) + 1)
}

fn into_bits(x: u8) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    (
        x & 0b1000_0000,
        x & 0b0100_0000,
        x & 0b0010_0000,
        x & 0b0001_0000,
        x & 0b0000_1000,
        x & 0b0000_0100,
        x & 0b0000_0010,
        x & 0b0000_0001,
    )
}

/// Generates an [Instruction](instructions/trait.Instruction.html).
///
/// The actual instruction implementation definition takes a variable arguments list which
/// expands to underscore for the unspecified arguments in the generated implementation.
/// The possible order of the arguments is fixed to `self`, [Registers](registers/struct.Registers.html),
/// [Memory](memory/struct.Memory.html), [CpuFlags](cpu/struct.CpuFlags.html).
///
/// ## Examples
///
/// In this example the struct will have no additional properties:
///
/// ```
/// # #[macro_use] extern crate gejmboj_cpu;
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// # use std::fmt::Display;
/// define_instruction! {
///    /// My awesome instruction
///    Awesome { "AWESOME"; 1 }
///
///    (self) => {
///        Ok(2)
///    }
///}
///
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
///
/// let instruction = Awesome {};
/// let machine_cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
/// assert_eq!(1, instruction.length());
/// assert_eq!(2, machine_cycles);
/// assert_eq!("AWESOME", format!("{}", instruction));
/// ```
///
/// When you add properties these must be referenced in the format string template argument:
///
/// ```
/// # #[macro_use] extern crate gejmboj_cpu;
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// # use std::fmt::Display;
/// define_instruction! {
///    /// My awesome instruction with properties
///    AwesomeWithProps { "AWESOME({:04x}) {:?}", operand: u16, condition: Condition; 3 }
///
///    (self, registers, memory) => {
///        if self.condition.is_fulfilled(registers) {
///            let [lo, hi] = registers.PC.to_le_bytes();
///            memory.set((registers.SP - 1).into(), hi);
///            memory.set((registers.SP - 2).into(), lo);
///            registers.SP -= 2;
///            registers.PC = self.operand;
///            Ok(6)
///        } else {
///            Ok(3)
///        }
///    }
///}
///
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
///
/// let instruction = AwesomeWithProps { operand: 0x0200, condition: Condition::Zero };
/// let mut machine_cycles = 0;
///
/// machine_cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
///
/// assert_eq!(3, instruction.length());
/// assert_eq!(3, machine_cycles);
/// assert_eq!("AWESOME(0200) Zero", format!("{}", instruction));
///
/// registers.set_single(SingleRegister::F, 0b1000_0000);
///
/// machine_cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
///
/// assert_eq!(6, machine_cycles);
/// ```
#[macro_export]
macro_rules! define_instruction {
    ($(#[$docs:meta])* $name:ident { $template:expr $(, $operand:ident: $t:tt)* ; $length:literal }

     ($($arg:ident),+) => $body:block

     $(@test $testname:ident ($r:ident, $m:ident, $c:ident) => $testbody:block)*) => {
        $(#[$docs])*
        pub struct $name {
            $(pub $operand: $t),*
        }

        impl $crate::instructions::Instruction for $name {
            $crate::instruction_execute!(($($arg),+) => $body);

            fn length(&self) -> u16 {
                $length
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $template $(, self.$operand)*)
            }
        }

        $(
            #[cfg(test)]
            #[test]
            fn $testname() {
                use $crate::registers::*;
                use $crate::instructions::*;
                let mut $r = $crate::registers::Registers::new();
                let mut $m = $crate::memory::Memory::new();
                let mut $c = $crate::cpu::CpuFlags::new();

                $testbody
            }

        )*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! instruction_execute {
    (($self:ident) => $body:block) => {
        fn execute(
            &$self,
            _: &mut $crate::registers::Registers,
            _: &mut $crate::memory::Memory,
            _: &mut $crate::cpu::CpuFlags,
        ) -> $crate::instructions::InstructionResult $body
    };
    (($self:ident, $r:ident) => $body:block) => {
        fn execute(
            &$self,
            $r: &mut $crate::registers::Registers,
            _: &mut $crate::memory::Memory,
            _: &mut $crate::cpu::CpuFlags,
        ) -> $crate::instructions::InstructionResult $body
    };
    (($self:ident, $r:ident, $m:ident) => $body:block) => {
        fn execute(
            &$self,
            $r: &mut $crate::registers::Registers,
            $m: &mut $crate::memory::Memory,
            _: &mut $crate::cpu::CpuFlags,
        ) -> $crate::instructions::InstructionResult $body
    };
    (($self:ident, $r:ident, $m:ident, $c:ident) => $body:block) => {
        fn execute(
            &$self,
            $r: &mut $crate::registers::Registers,
            $m: &mut $crate::memory::Memory,
            $c: &mut $crate::cpu::CpuFlags,
        ) -> $crate::instructions::InstructionResult $body
    };
}
