use crate::{
    define_instruction,
    registers::{DoubleRegister, SingleRegister},
};

define_instruction! {
    /// Loads data from register `r2` into `r1`.
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let instruction = Ld { r1: SingleRegister::B, r2: SingleRegister::E };
    /// registers.set_single(SingleRegister::E, 42);
    ///
    /// assert_eq!(0, registers.get_single(SingleRegister::B));
    ///
    /// let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    ///
    /// assert_eq!(42, registers.get_single(SingleRegister::B));
    /// assert_eq!(1, cycles);
    /// assert_eq!("LD(B, E)", format!("{}", instruction));
    /// ```
    Ld { "LD({:?}, {:?})", r1: SingleRegister, r2: SingleRegister; 1 }

    (self, registers) => {
        let value = registers.get_single(self.r2);
        registers.set_single(self.r1, value);
        Ok(1)
    }
}

define_instruction! {
    /// Loads data pointed to by HL into `r`.
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let instruction = LdFromHL { r: SingleRegister::B };
    ///
    /// memory.set(0x9000, 42);
    /// registers.set_double(DoubleRegister::HL, 0x9000);
    ///
    /// let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    ///
    /// assert_eq!(42, registers.get_single(SingleRegister::B));
    /// assert_eq!(1, cycles);
    /// assert_eq!("LD(B, (HL))", format!("{}", instruction));
    /// ```
    LdFromHL { "LD({:?}, (HL))", r: SingleRegister; 1 }

    (self, registers, memory) => {
        let value = memory.get(registers.get_double(DoubleRegister::HL).into());
        registers.set_single(self.r, value);
        Ok(1)
    }
}

define_instruction! {
    /// Loads data in `r` into location pointed to by HL.
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # use gejmboj_cpu::memory::*;
    /// # use gejmboj_cpu::instructions::*;
    /// # use gejmboj_cpu::cpu::*;
    /// let mut registers = Registers::new();
    /// let mut memory = Memory::new();
    /// let mut cpu_flags = CpuFlags::new();
    /// let instruction = LdToHL { r: SingleRegister::B };
    ///
    /// registers.set_single(SingleRegister::B, 42);
    /// registers.set_double(DoubleRegister::HL, 0x9000);
    ///
    /// let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
    ///
    /// assert_eq!(42, memory.get(0x9000));
    /// assert_eq!(1, cycles);
    /// assert_eq!("LD((HL), B)", format!("{}", instruction));
    /// ```
    LdToHL { "LD((HL), {:?})", r: SingleRegister; 1 }

    (self, registers, memory) => {
        let value = registers.get_single(self.r);
        memory.set(registers.get_double(DoubleRegister::HL).into(), value);
        Ok(1)
    }
}
