use crate::instruction_group;
use crate::registers::DoubleRegister;

instruction_group! {
    /// 16-bit load instructions.
    Load16Bit (registers, _memory, _cpu_flags) {

        /// Loads 16-bit data into 16-bit register
        Ld(r: DoubleRegister, operand: u16) [3] => {
            registers.set_double(&r, *operand);
            Ok(3)
        }
    }
}

#[cfg(test)]
crate::instruction_tests! {
    load_16_bit_data_to_registers(registers, memory, cpu_flags) => {
        let instruction = Load16Bit::Ld(DoubleRegister::BC, 0x1234);
        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x1234, registers.get_double(&DoubleRegister::BC), "Register BC was not set correctly");
        assert_eq!(3, cycles);

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
}
