use crate::{
    define_instruction,
    registers::{DoubleRegister, SingleRegister},
};

define_instruction! {
    /// Loads data from register `r2` into `r1`.
    Ld { "LD({:?}, {:?})", r1: SingleRegister, r2: SingleRegister; 1 }

    (self, registers) => {
        let value = registers.get_single(self.r2);
        registers.set_single(self.r1, value);
        Ok(1)
    }

    @test load_data_from_register_r2_into_register_r1(registers, memory, cpu_flags) => {
        let instruction = Ld { r1: SingleRegister::B, r2: SingleRegister::E };
        registers.set_single(SingleRegister::E, 42);

        assert_eq!(0, registers.get_single(SingleRegister::B));

        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(SingleRegister::B));
        assert_eq!(1, cycles);
        assert_eq!("LD(B, E)", format!("{}", instruction));
    }
}

define_instruction! {
    /// Loads data pointed to by HL into `r`.
    LdFromHL { "LD({:?}, (HL))", r: SingleRegister; 1 }

    (self, registers, memory) => {
        let value = memory.get(registers.get_double(DoubleRegister::HL).into());
        registers.set_single(self.r, value);
        Ok(2)
    }

    @test loads_data_pointed_to_by_hl_into_register(registers, memory, cpu_flags) => {
        let instruction = LdFromHL { r: SingleRegister::B };

        memory.set(0x9000, 42);
        registers.set_double(DoubleRegister::HL, 0x9000);

        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(SingleRegister::B));
        assert_eq!(2, cycles);
        assert_eq!("LD(B, (HL))", format!("{}", instruction));
    }
}

define_instruction! {
    /// Loads data in `r` into location pointed to by HL.
    LdToHL { "LD((HL), {:?})", r: SingleRegister; 1 }

    (self, registers, memory) => {
        let value = registers.get_single(self.r);
        memory.set(registers.get_double(DoubleRegister::HL).into(), value);
        Ok(2)
    }

    @test loads_data_in_register_into_location_at_hl(registers, memory, cpu_flags) => {
        let instruction = LdToHL { r: SingleRegister::B };

        registers.set_single(SingleRegister::B, 42);
        registers.set_double(DoubleRegister::HL, 0x9000);

        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, memory.get(0x9000));
        assert_eq!(2, cycles);
        assert_eq!("LD((HL), B)", format!("{}", instruction));
    }
}

define_instruction! {
    /// Loads `operand` into register `r`.
    LdByte { "LD({:?}, {:02x})", r: SingleRegister, operand: u8; 2 }

    (self, registers) => {
        registers.set_single(self.r, self.operand);
        Ok(2)
    }

    @test loads_operand_into_register(registers, memory, cpu_flags) => {
        let instruction = LdByte { r: SingleRegister::B, operand: 0x42 };
        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles);
        assert_eq!(0x42, registers.get_single(SingleRegister::B));
        assert_eq!("LD(B, 42)", format!("{}", instruction));
    }
}

define_instruction! {
    /// Load the value of `operand` into the location pointed to by `HL`
    LdByteToHL { "LD((HL), {:02x})", operand: u8; 2 }

    (self, registers, memory) => {
        memory.set(registers.get_double(DoubleRegister::HL).into(), self.operand);
        Ok(3)
    }

    @test load_value_into_hl_location(registers, memory, cpu_flags) => {
        let instruction = LdByteToHL { operand: 0x42 };
        registers.set_double(DoubleRegister::HL, 0x9000);

        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(3, cycles);
        assert_eq!(0x42, memory.get(0x9000));
        assert_eq!("LD((HL), 42)", format!("{}", instruction));
    }
}
