use crate::instruction_group;
use crate::registers::{DoubleRegister, SingleRegister};

instruction_group! {
    /// 8 Bit load instructions.
    Load8Bit (registers, memory, _cpu_flags) {

        /// Loads data from register `r2` into `r1`.
        Ld(r1: SingleRegister, r2: SingleRegister) [1] => {
            let value = registers.get_single(r2);
            registers.set_single(r1, value);
            Ok(1)
        }

        /// Loads data pointed to by HL into `r`.
        LdFromHL(r: SingleRegister) [1] => {
            let value = memory.get(registers.get_double(&DoubleRegister::HL).into());
            registers.set_single(r, value);
            Ok(2)
        }

        /// Loads data in `r` into location pointed to by HL.
        LdToHL(r: SingleRegister) [1] => {
            let value = registers.get_single(r);
            memory.set(registers.get_double(&DoubleRegister::HL).into(), value);
            Ok(2)
        }

        /// Loads `operand` into register `r`.
        LdByte(r: SingleRegister, operand: u8) [2] => {
            registers.set_single(r, *operand);
            Ok(2)
        }

        /// Load the value of `operand` into the location pointed to by `HL`
        LdByteToHL(operand: u8) [2] => {
            memory.set(registers.get_double(&DoubleRegister::HL).into(), *operand);
            Ok(3)
        }

        /// Load data at address pointed to by BC into A
        LdBCToA() [1] => {
            let value = memory.get(registers.get_double(&DoubleRegister::BC).into());
            registers.set_single(&SingleRegister::A, value);
            Ok(2)
        }

        /// Load data at address pointed to by DE into A
        LdDEToA() [1] => {
            let value = memory.get(registers.get_double(&DoubleRegister::DE).into());
            registers.set_single(&SingleRegister::A, value);
            Ok(2)
        }

        /// Load A into into address pointed to by BC
        LdAToBC() [1] => {
            memory.set(
                registers.get_double(&DoubleRegister::BC).into(),
                registers.get_single(&SingleRegister::A)
            );
            Ok(2)
        }

        /// Load A into into address pointed to by DE
        LdAToDE() [1] => {
            memory.set(
                registers.get_double(&DoubleRegister::DE).into(),
                registers.get_single(&SingleRegister::A)
            );
            Ok(2)
        }

        /// Load data at `address` into A
        LdToA(address: u16) [3] => {
            let value = memory.get((*address).into());
            registers.set_single(&SingleRegister::A, value);
            Ok(4)
        }

        /// Load data in A into address at `address`
        LdFromA(address: u16) [3] => {
            memory.set((*address).into(), registers.get_single(&SingleRegister::A));
            Ok(4)
        }

        /// Load data to A from the address at `0xFF00` + register C
        LdhCToA() [1] => {
            let lo = registers.get_single(&SingleRegister::C);
            let address = u16::from_le_bytes([lo, 0xFF]);
            let value = memory.get(address.into());
            registers.set_single(&SingleRegister::A, value);
            Ok(2)
        }

        /// Load data from A into the address at `0xFF00` + register C
        LdhAToC() [1] => {
            let value = registers.get_single(&SingleRegister::A);
            let lo = registers.get_single(&SingleRegister::C);
            let address = u16::from_le_bytes([lo, 0xFF]);
            memory.set(address.into(), value);
            Ok(2)
        }

        /// Load data to A from the address at `0xFF00` + `operand`
        LdhToA(operand: u8) [2] => {
            let address = u16::from_le_bytes([*operand, 0xFF]);
            let value = memory.get(address.into());
            registers.set_single(&SingleRegister::A, value);
            Ok(3)
        }

        /// Load data from A into the address at `0xFF00` + `operand`
        LdhFromA(operand: u8) [2] => {
            let address = u16::from_le_bytes([*operand, 0xFF]);
            let value = registers.get_single(&SingleRegister::A);
            memory.set(address.into(), value);
            Ok(3)
        }

        /// Load data to A from the address at HL, value at HL is decremented.
        LdAFromHLDec() [1] => {
            let address = registers.get_double(&DoubleRegister::HL);
            let value = memory.get(address.into());
            registers.set_double(&DoubleRegister::HL, address - 1);
            registers.set_single(&SingleRegister::A, value);
            Ok(2)
        }

        /// Load data to address at HL from A, HL is decremented after write.
        LdAToHLDec() [1] => {
            let address = registers.get_double(&DoubleRegister::HL);
            let value = registers.get_single(&SingleRegister::A);
            memory.set(address.into(), value);
            registers.set_double(&DoubleRegister::HL, address - 1);
            Ok(2)
        }

        /// Load data to A from the address at HL, value at HL is incremented.
        LdAFromHLInc() [1] => {
            let address = registers.get_double(&DoubleRegister::HL);
            let value = memory.get(address.into());
            registers.set_double(&DoubleRegister::HL, address + 1);
            registers.set_single(&SingleRegister::A, value);
            Ok(2)
        }

        /// Load data to address at HL from A, HL is incremented after write.
        LdAToHLInc() [1] => {
            let address = registers.get_double(&DoubleRegister::HL);
            let value = registers.get_single(&SingleRegister::A);
            memory.set(address.into(), value);
            registers.set_double(&DoubleRegister::HL, address + 1);
            Ok(2)
        }
    }
}

#[cfg(test)]
crate::instruction_tests! {
    load_data_from_register_r2_into_register_r1(registers, memory, cpu_flags) => {
        let instruction = Load8Bit::Ld(SingleRegister::B, SingleRegister::E);
        registers.set_single(&SingleRegister::E, 42);

        assert_eq!(0, registers.get_single(&SingleRegister::B));

        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::B));
        assert_eq!(1, cycles);
    }

    loads_data_pointed_to_by_hl_into_register(registers, memory, cpu_flags) => {
        let instruction = Load8Bit::LdFromHL(SingleRegister::B);

        memory.set(0x9000, 42);
        registers.set_double(&DoubleRegister::HL, 0x9000);

        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::B));
        assert_eq!(2, cycles);
    }

    loads_data_in_register_into_location_at_hl(registers, memory, cpu_flags) => {
        let instruction = Load8Bit::LdToHL(SingleRegister::B);

        registers.set_single(&SingleRegister::B, 42);
        registers.set_double(&DoubleRegister::HL, 0x9000);

        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, memory.get(0x9000));
        assert_eq!(2, cycles);
    }

    loads_operand_into_register(registers, memory, cpu_flags) => {
        let instruction = Load8Bit::LdByte(SingleRegister::B, 0x42);
        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles);
        assert_eq!(0x42, registers.get_single(&SingleRegister::B));
    }

    load_value_into_hl_location(registers, memory, cpu_flags) => {
        let instruction = Load8Bit::LdByteToHL(0x42);
        registers.set_double(&DoubleRegister::HL, 0x9000);

        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(3, cycles);
        assert_eq!(0x42, memory.get(0x9000));
    }
}
