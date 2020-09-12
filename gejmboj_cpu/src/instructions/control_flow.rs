use super::{Condition, Instruction, InstructionResult};
use crate::memory::Memory;
use crate::registers::Registers;
use crate::{cpu::CpuFlags, define_instruction, registers::DoubleRegister};
use std::fmt::Display;

define_instruction! {
    /// Unconditional jump to location specified by 16-bit operand.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let instruction = Jp { operand: 0xBADA };
    ///
    /// assert_eq!(0, registers.PC);
    /// instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    /// assert_eq!(0xBADA, registers.PC);
    /// ```
    Jp { "JP({:04x})", operand: u16; 3 }

    (self, registers) => {
        registers.PC = self.operand;
        Ok(4)
    }
}

define_instruction! {
    /// Conditional jump to location specified by 16-bit operand.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let instruction = JpIf { operand: 0xBADA, condition: Condition::Carry };
    ///
    /// let mut cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    /// assert_eq!(0, registers.PC);
    /// assert_eq!(3, cycles);
    ///
    /// registers.set_single(SingleRegister::F, 0b0001_0000);
    /// cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    /// assert_eq!(0xBADA, registers.PC);
    /// assert_eq!(4, cycles);
    /// ```
    JpIf { "JP({:04x}) {:?}", operand: u16, condition: Condition; 3 }

    (self, registers) => {
            if self.condition.is_fulfilled(registers) {
            registers.PC = self.operand;
            Ok(4)
        } else {
            Ok(3)
        }

    }
}

define_instruction! {
    /// Unconditional jump to location specified by register HL
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let instruction = JpToHL { };
    ///
    /// registers.set_double(DoubleRegister::HL, 0xBADA);
    /// instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    /// assert_eq!(0xBADA, registers.PC);
    /// ```
    JpToHL { "JP(HL)"; 1 }

    (self, registers) => {
        registers.PC = registers.get_double(DoubleRegister::HL);
        Ok(1)
    }
}

define_instruction! {
    /// Unconditional jump to location at current + offset
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// registers.PC = 0x0200;
    /// let instruction = JpToOffset { operand: 0x42 };
    ///
    /// instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    /// assert_eq!(0x0242, registers.PC);
    /// ```
    JpToOffset { "JP(PC + {:02x})", operand: u8; 2 }

    (self, registers) => {
        registers.PC += self.operand as u16;
        Ok(3)
    }
}

define_instruction! {
    /// Conditional jump to relative address specified by offset operand.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let instruction = JpToOffsetIf { operand: 0x42, condition: Condition::Zero };
    /// let mut cycles = 0;
    /// registers.PC = 0x0200;
    ///
    /// cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    /// assert_eq!(0x0200, registers.PC);
    /// assert_eq!(2, cycles);
    ///
    /// registers.set_single(SingleRegister::F, 0b1000_0000);
    ///
    /// cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    /// assert_eq!(0x0242, registers.PC);
    /// assert_eq!(3, cycles);
    /// ```
    JpToOffsetIf { "JP(PC + {:02x}) {:?}", operand: u8, condition: Condition; 2 }

    (self, registers) => {
        if self.condition.is_fulfilled(registers) {
            registers.PC += self.operand as u16;
            Ok(3)
        } else {
            Ok(2)
        }
    }
}

define_instruction! {
    /// Unconditional call of the function at operand address.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let instruction = Call { operand: 0xABCD };
    /// registers.PC = 0xAAAA;
    /// instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    ///
    /// assert_eq!(0xABCD, registers.PC);
    /// assert_eq!(0xFFFC, registers.SP);
    /// assert_eq!(0xAAAA, memory.get_u16(registers.SP.into()));
    /// ```
    Call { "CALL({:04x})", operand: u16; 3 }

    (self, registers, memory) => {
        let [lo, hi] = registers.PC.to_le_bytes();
        memory.set((registers.SP - 1).into(), hi);
        memory.set((registers.SP - 2).into(), lo);
        registers.SP -= 2;
        registers.PC = self.operand;
        Ok(6)
    }
}

define_instruction! {
    /// Conditional function call.
    ///
    /// ## Examples
    ///
    /// Function is not called if the condition is not fulfilled:
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let instruction = CallIf { operand: 0xABCD, condition: Condition::Carry };
    /// registers.PC = 0xAAAA;
    ///
    /// let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    ///
    /// assert_eq!(registers.PC, 0xAAAA);
    /// assert_eq!(registers.SP, 0xFFFE);
    /// assert_eq!(3, cycles);
    /// ```
    ///
    /// If the condition is fulfilled then the function is called:
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let instruction = CallIf { operand: 0xABCD, condition: Condition::Carry };
    /// registers.PC = 0xAAAA;
    /// registers.set_single(SingleRegister::F, 0b0001_0000);
    ///
    /// let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    ///
    /// assert_eq!(registers.PC, 0xABCD);
    /// assert_eq!(registers.SP, 0xFFFC);
    /// assert_eq!(6, cycles);
    /// ```
    CallIf { "CALL({:04x}) {:?}", operand: u16, condition: Condition; 3 }

    (self, registers, memory) => {
        if self.condition.is_fulfilled(registers) {
            let [lo, hi] = registers.PC.to_le_bytes();
            memory.set((registers.SP - 1).into(), hi);
            memory.set((registers.SP - 2).into(), lo);
            registers.SP -= 2;
            registers.PC = self.operand;
            Ok(6)
        } else {
            Ok(3)
        }
    }
}

define_instruction! {
    /// Unconditional return from function.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let function_call = Call { operand: 0xABCD };
    /// let return_call = Ret { };
    ///
    /// registers.PC = 0xAAAA;
    /// function_call.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    /// return_call.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    ///
    /// assert_eq!(0xAAAA, registers.PC);
    /// assert_eq!(0xFFFE, registers.SP);
    /// ```
    Ret { "RET"; 1 }

    (self, registers, memory) => {
        registers.PC = memory.get_u16(registers.SP.into());
        registers.SP += 2;
        Ok(4)
    }
}

define_instruction! {
    /// Conditionally return from function.
    ///
    /// ## Examples
    ///
    /// Return from function call only if condition is fulfilled:
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// registers.PC = 0xAAAA;
    ///
    /// let call = Call { operand: 0xABCD };
    /// call.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    ///
    /// let ret = RetIf { condition: Condition::Carry };
    /// let mut cycles = 0;
    /// cycles = ret.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    ///
    /// assert_eq!(0xABCD, registers.PC);
    /// assert_eq!(0xFFFC, registers.SP);
    /// assert_eq!(2, cycles);
    ///
    /// registers.set_single(SingleRegister::F, 0b0001_0000);
    /// cycles = ret.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    ///
    /// assert_eq!(0xAAAA, registers.PC);
    /// assert_eq!(0xFFFE, registers.SP);
    /// assert_eq!(5, cycles);
    /// ```
    RetIf { "RET {:?}", condition: Condition; 1 }

    (self, registers, memory) => {
        if self.condition.is_fulfilled(registers) {
            registers.PC = memory.get_u16(registers.SP.into());
            registers.SP += 2;
            Ok(5)
        } else {
            Ok(2)
        }
    }
}

define_instruction! {
    /// Unconditional return from a function which enables interrupts
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let call = Call { operand: 0xABCD };
    /// let reti = RetI { };
    ///
    /// registers.PC = 0xAAAA;
    /// call.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    /// reti.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    ///
    /// assert_eq!(0xAAAA, registers.PC);
    /// assert_eq!(0xFFFE, registers.SP);
    /// assert_eq!(1, cpu_flags.IME);
    /// ```
    RetI { "RET IME=1"; 1 }

    (self, registers, memory, cpu_flags) => {
        registers.PC = memory.get_u16(registers.SP.into());
        registers.SP += 2;
        cpu_flags.IME = 1;
        Ok(4)
    }
}

define_instruction! {
    /// Unconditional function call to the RESET address defined by bits 3-5
    ///
    /// Possible RESET addresses are:
    ///
    /// * `0x00`
    /// * `0x08`
    /// * `0x10`
    /// * `0x18`
    /// * `0x20`
    /// * `0x28`
    /// * `0x30`
    /// * `0x38`
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let instruction = Rst { opcode: 0b1101_0111};
    ///
    /// instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    ///
    /// assert_eq!(0x10, registers.PC);
    /// ```
    Rst { "RST({:08b})", opcode: u8; 1 }

    (self, registers) => {
        registers.PC = get_reset_address(self.opcode);
        Ok(4)
    }
}

fn get_reset_address(opcode: u8) -> u16 {
    (opcode & 0b00111000) as u16
}
