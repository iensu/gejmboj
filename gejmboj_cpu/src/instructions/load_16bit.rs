use crate::instruction_group;
use crate::registers::DoubleRegister;

instruction_group! {
    /// 16-bit load instructions.
    Load16Bit (registers, memory, _cpu_flags) {

        /// Loads 16-bit data into 16-bit register
        Ld(r: DoubleRegister, operand: u16) [3] => {
            registers.set_double(&r, *operand);
            Ok(3)
        }

        /// Loads value from SP into address
        LdFromSP(address: u16) [3] => {
            let value = registers.get_double(&DoubleRegister::SP);
            memory.set_u16((*address).into(), value);
            Ok(5)
        }

        /// Loads data from HL into SP
        LdHLToSP() [1] => {
            let value = registers.get_double(&DoubleRegister::HL);
            registers.set_double(&DoubleRegister::SP, value);
            Ok(2)
        }
    }
}

#[cfg(test)]
crate::instruction_tests! {
    load_16_bit_data_to_registers(registers, memory, cpu_flags) => {
        let instruction = Load16Bit::Ld(DoubleRegister::BC, 0x1234);
        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(3, cycles);
        assert_eq!(0x1234, registers.get_double(&DoubleRegister::BC), "Register BC was not set correctly");

        let instruction = Load16Bit::Ld(DoubleRegister::DE, 0x1234);
        let _cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x1234, registers.get_double(&DoubleRegister::DE), "Register DE was not set correctly");

        let instruction = Load16Bit::Ld(DoubleRegister::HL, 0x1234);
        let _cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x1234, registers.get_double(&DoubleRegister::HL), "Register HL was not set correctly");

        let instruction = Load16Bit::Ld(DoubleRegister::SP, 0x1234);
        let _cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x1234, registers.get_double(&DoubleRegister::SP), "Register SP was not set correctly");
    }

    load_16_bit_data_from_sp_into_address(registers, memory, cpu_flags) => {
        registers.set_double(&DoubleRegister::SP, 0x1234);
        let instruction = Load16Bit::LdFromSP(0xABCD);
        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(5, cycles);
        assert_eq!(0x1234, memory.get_u16(0xABCD));
    }

    load_hl_to_sp(registers, memory, cpu_flags) => {
        registers.set_double(&DoubleRegister::HL, 0x1234);
        let stack_pointer_start_address = 0xFFFE;
        assert_eq!(stack_pointer_start_address, registers.get_double(&DoubleRegister::SP));

        let instruction = Load16Bit::LdHLToSP();
        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(2, cycles);
        assert_eq!(0x1234, registers.get_double(&DoubleRegister::SP));
    }
}
