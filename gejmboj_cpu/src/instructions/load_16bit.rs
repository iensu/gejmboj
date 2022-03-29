use crate::instruction_group;
use crate::registers::DoubleRegister;

instruction_group! {
    /// 16-bit load instructions.
    Load16Bit (registers, memory, _cpu_flags) {

        /// Loads 16-bit data into 16-bit register
        LD(r: DoubleRegister, operand: u16) [3] => {
            registers.set_double(&r, *operand);
            Ok(3)
        }

        /// Loads value from SP into address
        LD_FROM_SP(address: u16) [3] => {
            let value = registers.get_double(&DoubleRegister::SP);
            memory.set_u16((*address).into(), value);
            Ok(5)
        }

        /// Loads data from HL into SP
        LD_HL_TO_SP() [1] => {
            let value = registers.get_double(&DoubleRegister::HL);
            registers.set_double(&DoubleRegister::SP, value);
            Ok(2)
        }

        /// Push data from 16-bit register to stack memory
        PUSH(r: DoubleRegister) [1] => {
            let sp = registers.decrement_sp();
            let value = registers.get_double(r);
            memory.set_u16(sp.into(), value);
            Ok(4)
        }

        /// Pop data from stack memory to 16-bit register
        POP(r: DoubleRegister) [1] => {
            let sp = registers.get_double(&DoubleRegister::SP);
            let value = memory.get_u16(sp.into());
            registers.set_double(&r, value);
            registers.increment_sp();
            Ok(3)
        }
    }
}

#[cfg(test)]
crate::instruction_tests! {
    load_16_bit_data_to_registers(registers, memory, cpu_flags) => {
        let instruction = Load16Bit::LD(DoubleRegister::BC, 0x1234);
        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(3, cycles);
        assert_eq!(0x1234, registers.get_double(&DoubleRegister::BC), "Register BC was not set correctly");

        let instruction = Load16Bit::LD(DoubleRegister::DE, 0x1234);
        let _cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x1234, registers.get_double(&DoubleRegister::DE), "Register DE was not set correctly");

        let instruction = Load16Bit::LD(DoubleRegister::HL, 0x1234);
        let _cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x1234, registers.get_double(&DoubleRegister::HL), "Register HL was not set correctly");

        let instruction = Load16Bit::LD(DoubleRegister::SP, 0x1234);
        let _cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x1234, registers.get_double(&DoubleRegister::SP), "Register SP was not set correctly");
    }

    load_16_bit_data_from_sp_into_address(registers, memory, cpu_flags) => {
        registers.set_double(&DoubleRegister::SP, 0x1234);
        let instruction = Load16Bit::LD_FROM_SP(0xABCD);
        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(5, cycles);
        assert_eq!(0x1234, memory.get_u16(0xABCD));
    }

    load_hl_to_sp(registers, memory, cpu_flags) => {
        registers.set_double(&DoubleRegister::HL, 0x1234);
        let stack_pointer_start_address = 0xFFFE;
        assert_eq!(stack_pointer_start_address, registers.get_double(&DoubleRegister::SP));

        let instruction = Load16Bit::LD_HL_TO_SP();
        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(2, cycles);
        assert_eq!(0x1234, registers.get_double(&DoubleRegister::SP));
    }

    push_register_to_stack_memory(registers, memory, cpu_flags) => {
        let stack_pointer_start_address = 0xFFFE;
        assert_eq!(stack_pointer_start_address, registers.get_double(&DoubleRegister::SP));

        registers.set_double(&DoubleRegister::BC, 0x1122);
        registers.set_double(&DoubleRegister::DE, 0x3344);
        registers.set_double(&DoubleRegister::HL, 0x5566);
        registers.set_double(&DoubleRegister::AF, 0x7780);

        let cycles = Load16Bit::PUSH(DoubleRegister::BC).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        let sp = registers.get_double(&DoubleRegister::SP);
        assert_eq!(4, cycles);
        assert_eq!(stack_pointer_start_address - 2, sp);
        assert_eq!(0x1122, memory.get_u16(sp.into()));

        let cycles = Load16Bit::PUSH(DoubleRegister::DE).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        let sp = registers.get_double(&DoubleRegister::SP);
        assert_eq!(4, cycles);
        assert_eq!(stack_pointer_start_address - 4, sp);
        assert_eq!(0x3344, memory.get_u16(sp.into()));

        let cycles = Load16Bit::PUSH(DoubleRegister::HL).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        let sp = registers.get_double(&DoubleRegister::SP);
        assert_eq!(4, cycles);
        assert_eq!(stack_pointer_start_address - 6, sp);
        assert_eq!(0x5566, memory.get_u16(sp.into()));

        let cycles = Load16Bit::PUSH(DoubleRegister::AF).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        let sp = registers.get_double(&DoubleRegister::SP);
        assert_eq!(4, cycles);
        assert_eq!(stack_pointer_start_address - 8, sp);
        assert_eq!(0x7780, memory.get_u16(sp.into()));
    }

    pop_stack_memory_to_bc_register(registers, memory, cpu_flags) => {
        let sp = registers.decrement_sp();
        memory.set_u16(sp.into(), 0xABCD);
        let cycles = Load16Bit::POP(DoubleRegister::BC).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(cycles, 3);
        assert_eq!(sp + 2, registers.get_double(&DoubleRegister::SP));
        assert_eq!(0xABCD, registers.get_double(&DoubleRegister::BC));
    }

    pop_stack_memory_to_de_register(registers, memory, cpu_flags) => {
        let sp = registers.decrement_sp();
        memory.set_u16(sp.into(), 0xABCD);
        let cycles = Load16Bit::POP(DoubleRegister::DE).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(cycles, 3);
        assert_eq!(sp + 2, registers.get_double(&DoubleRegister::SP));
        assert_eq!(0xABCD, registers.get_double(&DoubleRegister::DE));
    }

    pop_stack_memory_to_hl_register(registers, memory, cpu_flags) => {
        let sp = registers.decrement_sp();
        memory.set_u16(sp.into(), 0xABCD);
        let cycles = Load16Bit::POP(DoubleRegister::HL).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(cycles, 3);
        assert_eq!(sp + 2, registers.get_double(&DoubleRegister::SP));
        assert_eq!(0xABCD, registers.get_double(&DoubleRegister::HL));
    }

    pop_stack_memory_to_af_register(registers, memory, cpu_flags) => {
        let sp = registers.decrement_sp();
        memory.set_u16(sp.into(), 0xABCD);
        let cycles = Load16Bit::POP(DoubleRegister::AF).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(cycles, 3);
        assert_eq!(sp + 2, registers.get_double(&DoubleRegister::SP));
        // Lowest nibble (4 bits) of the AF register are unwriteable.
        assert_eq!(0xABC0, registers.get_double(&DoubleRegister::AF));
    }
}
