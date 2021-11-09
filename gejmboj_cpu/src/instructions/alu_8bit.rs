use crate::{
    errors::CpuError,
    instruction_group,
    registers::{Registers, SingleRegister, MASK_FLAG_CARRY},
};

fn is_half_carry(x: u16, y: u16) -> bool {
    let x_lowest_nibble = x & 0xF;
    let y_lowest_nibble = y & 0xF;
    let mask = 0x10;

    (x_lowest_nibble + y_lowest_nibble) & mask == mask
}

fn calculate_flags(result: u16, x: u16, y: u16, is_subtraction: bool) -> u8 {
    let mut flags = 0b0000_0000;

    if result == 0 {
        flags = flags | 0b1000_0000; // Set Z
    }
    if is_subtraction {
        flags = flags | 0b0100_0000; // Set N
    }
    if is_half_carry(x, y) {
        flags = flags | 0b0010_0000; // Set H
    }
    if result > 0xFF {
        flags = flags | 0b0001_0000; // Set C
    }

    flags
}

fn do_adda(registers: &mut Registers, operand: u16) {
    let a = registers.get_single(&SingleRegister::A);

    let mut result: u16 = a as u16 + operand;
    let flags = calculate_flags(result, a.into(), operand, false);

    if flags & MASK_FLAG_CARRY > 0 {
        result = result >> 8;
    }

    registers.set_single(&SingleRegister::A, result as u8);
    registers.set_single(&SingleRegister::F, flags);
}

instruction_group! {
    /// 8-bit ALU (math) instructions
    ALU8Bit (registers, _memory, _cpu_flags) {

        /// Add value of `SingleRegister` to `A`
        AddA(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            do_adda(registers, registers.get_single(&r).into());

            Ok(1)
        }

        /// Add value of `operand` to `A`
        AddAN(operand: u8) [2] => {
            do_adda(registers, (*operand).into());

            Ok(2)
        }

        /// Add value of `HL` to `A`
        AddAHL() [1] => {
            do_adda(registers, registers.get_double(&crate::registers::DoubleRegister::HL));

            Ok(2)
        }

        /// Add value of `SingleRegister` and the Carry flag to `A`
        AdcA(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let mut operand: u16 = registers.get_single(&r).into();

            if registers.is_carry() {
                operand +=  1;
            }

            do_adda(registers, operand);

            Ok(1)
        }

        /// Add value of `operand` and Carry to `A`
        AdcAN(operand: u8) [2] => {
            let mut operand: u16 = (*operand).into();

            if registers.is_carry() {
                operand += 1
            }

            do_adda(registers, operand);

            Ok(2)
        }

        /// Add value of `HL` and Carry to `A`
        AdcAHL() [1] => {
            let mut operand = registers.get_double(&crate::registers::DoubleRegister::HL);

            if registers.is_carry() {
                operand += 1;
            }

            do_adda(registers, operand);

            Ok(2)
        }
    }
}

#[cfg(test)]
crate::instruction_tests! {
    adda_takes_one_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::AddA(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    adda_adds_value_of_register_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 1);
        registers.set_single(&SingleRegister::B, 2);

        ALU8Bit::AddA(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(3, registers.get_single(&SingleRegister::A));
    }

    adda_sets_z_flag_if_result_is_zero(registers, memory, cpu_flags) => {
        assert_eq!(0b0000_0000, registers.get_single(&SingleRegister::F));

        registers.set_single(&SingleRegister::A, 0);
        registers.set_single(&SingleRegister::B, 0);

        ALU8Bit::AddA(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1000_0000, registers.get_single(&SingleRegister::F));
    }

    adda_sets_h_flag_if_carry_from_bit_3(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b0000_0111);
        registers.set_single(&SingleRegister::B, 0b0000_1001);

        ALU8Bit::AddA(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0010_0000, registers.get_single(&SingleRegister::F));
    }

    adda_sets_c_flag_if_carry_from_bit_7(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b1111_0000);
        registers.set_single(&SingleRegister::B, 0b0001_0000);

        ALU8Bit::AddA(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0001_0000, registers.get_single(&SingleRegister::F));
    }

    adda_handles_overflow(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b1111_1111);
        registers.set_single(&SingleRegister::B, 0b0000_0001);

        ALU8Bit::AddA(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0000_0001, registers.get_single(&SingleRegister::A));
    }

    adda_does_support_the_f_register(registers, memory, cpu_flags) => {
        let result = ALU8Bit::AddA(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(SingleRegister::F));

        assert_eq!(expected, result);
    }

    addan_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::AddAN(0xAB).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles);
    }

    addan_adds_operand_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);

        ALU8Bit::AddAN(2).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    addahl_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::AddAHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles);
    }

    addahl_adds_hl_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);
        registers.set_double(&DoubleRegister::HL, 2);

        ALU8Bit::AddAHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    adca_takes_1_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::AdcA(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    adca_adds_register_plus_carry_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);
        registers.set_single(&SingleRegister::B, 2);
        registers.set_single(&SingleRegister::F, MASK_FLAG_CARRY);

        ALU8Bit::AdcA(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(43, registers.get_single(&SingleRegister::A));

        registers.set_single(&SingleRegister::A, 40);
        registers.set_single(&SingleRegister::F, 0);

        ALU8Bit::AdcA(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    adcan_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::AdcAN(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles);
    }

    adcan_adds_register_plus_carry_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);
        registers.set_single(&SingleRegister::F, MASK_FLAG_CARRY);

        ALU8Bit::AdcAN(2).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(43, registers.get_single(&SingleRegister::A));

        registers.set_single(&SingleRegister::A, 40);
        registers.set_single(&SingleRegister::F, 0);

        ALU8Bit::AdcAN(2).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    adcahl_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::AdcAHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles);
    }

    adcahl_adds_register_plus_carry_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);
        registers.set_double(&DoubleRegister::HL, 2);
        registers.set_single(&SingleRegister::F, MASK_FLAG_CARRY);

        ALU8Bit::AdcAHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(43, registers.get_single(&SingleRegister::A));

        registers.set_single(&SingleRegister::A, 40);
        registers.set_single(&SingleRegister::F, 0);

        ALU8Bit::AdcAHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }
}
