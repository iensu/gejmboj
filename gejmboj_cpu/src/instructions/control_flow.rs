use super::Instruction;
use crate::registers::DoubleRegister;
use std::fmt::Display;

/// Unconditional jump to location specified by 16-bit operand.
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let instruction = Jp { operand: 0xBADA };
///
/// assert_eq!(0, registers.pc);
/// instruction.execute(&mut registers, &mut memory).unwrap();
/// assert_eq!(0xBADA, registers.pc);
/// ```
pub struct Jp {
    pub operand: u16,
}

impl Instruction for Jp {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        _memory: &mut crate::memory::Memory,
    ) -> Result<(), crate::errors::CpuError> {
        registers.pc = self.operand;
        Ok(())
    }
    fn duration(&self) -> u16 {
        4
    }
    fn length(&self) -> u16 {
        3
    }
}

impl Display for Jp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JP({:04x})", self.operand)
    }
}

/// Unconditional jump to location specified by register HL
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let instruction = JpToHL { };
///
/// registers.set_double(DoubleRegister::HL, 0xBADA);
/// instruction.execute(&mut registers, &mut memory).unwrap();
/// assert_eq!(0xBADA, registers.pc);
/// ```
pub struct JpToHL {}

impl Instruction for JpToHL {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        _memory: &mut crate::memory::Memory,
    ) -> Result<(), crate::errors::CpuError> {
        registers.pc = registers.get_double(DoubleRegister::HL);
        Ok(())
    }
    fn duration(&self) -> u16 {
        1
    }
    fn length(&self) -> u16 {
        1
    }
}

impl Display for JpToHL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JP(HL)")
    }
}

/// Unconditional jump to location at current + offset
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// registers.pc = 0x0200;
/// let instruction = JpToOffset { operand: 0x42 };
///
/// instruction.execute(&mut registers, &mut memory).unwrap();
/// assert_eq!(0x0242, registers.pc);
/// ```
pub struct JpToOffset {
    pub operand: u8,
}

impl Instruction for JpToOffset {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        _memory: &mut crate::memory::Memory,
    ) -> Result<(), crate::errors::CpuError> {
        registers.pc += self.operand as u16;
        Ok(())
    }
    fn duration(&self) -> u16 {
        3
    }
    fn length(&self) -> u16 {
        2
    }
}

impl Display for JpToOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JP(PC + {:02x})", self.operand)
    }
}

/// Unconditional call of the function at operand address.
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let instruction = Call { operand: 0xABCD };
/// registers.pc = 0xAAAA;
/// instruction.execute(&mut registers, &mut memory).unwrap();
///
/// assert_eq!(0xABCD, registers.pc);
/// assert_eq!(0xFFFC, registers.sp);
/// assert_eq!(0xAAAA, memory.get_u16(registers.sp.into()));
/// ```
pub struct Call {
    pub operand: u16,
}

impl Instruction for Call {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        memory: &mut crate::memory::Memory,
    ) -> Result<(), crate::errors::CpuError> {
        let [lo, hi] = registers.pc.to_le_bytes();
        memory.set((registers.sp - 1).into(), hi);
        memory.set((registers.sp - 2).into(), lo);
        registers.sp -= 2;
        registers.pc = self.operand;
        Ok(())
    }
    fn duration(&self) -> u16 {
        6
    }
    fn length(&self) -> u16 {
        3
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CALL({:04x})", self.operand)
    }
}

/// Unconditional return from function.
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let function_call = Call { operand: 0xABCD };
/// let return_call = Ret { };
///
/// registers.pc = 0xAAAA;
/// function_call.execute(&mut registers, &mut memory).unwrap();
/// return_call.execute(&mut registers, &mut memory).unwrap();
///
/// assert_eq!(0xAAAA, registers.pc);
/// assert_eq!(0xFFFE, registers.sp);
/// ```
pub struct Ret {}

impl Instruction for Ret {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        memory: &mut crate::memory::Memory,
    ) -> Result<(), crate::errors::CpuError> {
        registers.pc = memory.get_u16(registers.sp.into());
        registers.sp += 2;
        Ok(())
    }
    fn duration(&self) -> u16 {
        6
    }
    fn length(&self) -> u16 {
        3
    }
}

impl Display for Ret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RET")
    }
}
