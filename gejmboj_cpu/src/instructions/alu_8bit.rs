use crate::{
    errors::CpuError,
    instruction_group,
    registers::{
        Registers, SingleRegister, MASK_FLAG_CARRY, MASK_FLAG_HALF_CARRY, MASK_FLAG_NEGATIVE,
        MASK_FLAG_ZERO,
    },
};

fn add_is_half_carry(x: u8, y: u8) -> bool {
    let x_lowest_nibble = x & 0xF;
    let y_lowest_nibble = y & 0xF;
    let mask = 0x10;

    (x_lowest_nibble + y_lowest_nibble) & mask == mask
}

fn sub_is_half_carry(x: u8, y: u8) -> bool {
    let x_lowest_nibble = x & 0xF;
    let y_lowest_nibble = y & 0xF;

    y_lowest_nibble > x_lowest_nibble
}

fn wrapping_add(x: u8, y: u8) -> (u8, bool) {
    let is_overflow = x as u16 + y as u16 > u8::MAX as u16;
    let result = x.wrapping_add(y);

    (result, is_overflow)
}

fn wrapping_sub(x: u8, y: u8) -> (u8, bool) {
    let is_overflow = y > x;
    let result = x.wrapping_sub(y);

    (result, is_overflow)
}

fn perform_addition(registers: &mut Registers, operand: u8) {
    let a = registers.get_single(&SingleRegister::A);

    let (result, is_carry) = wrapping_add(a, operand);
    let mut flags = 0b0000_0000;

    if result == 0 {
        flags = flags | MASK_FLAG_ZERO; // Set Z
    }
    if add_is_half_carry(a, operand) {
        flags = flags | MASK_FLAG_HALF_CARRY; // Set H
    }
    if is_carry {
        flags = flags | MASK_FLAG_CARRY; // Set C
    }

    registers.set_single(&SingleRegister::A, result);
    registers.set_single(&SingleRegister::F, flags);
}

fn perform_subtraction(registers: &mut Registers, operand: u8) {
    let a = registers.get_single(&SingleRegister::A);

    let (result, is_carry) = wrapping_sub(a, operand);
    let mut flags = 0b0000_0000 | MASK_FLAG_NEGATIVE;

    if result == 0 {
        flags = flags | MASK_FLAG_ZERO; // Set Z
    }
    if sub_is_half_carry(a, operand) {
        flags = flags | MASK_FLAG_HALF_CARRY; // Set H
    }
    if is_carry {
        flags = flags | MASK_FLAG_CARRY; // Set C
    }

    registers.set_single(&SingleRegister::A, result);
    registers.set_single(&SingleRegister::F, flags);
}

instruction_group! {
    /// 8-bit ALU (math) instructions
    ALU8Bit (registers, _memory, _cpu_flags) {

        /// Add value of `SingleRegister` to `A`
        Add(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            perform_addition(registers, registers.get_single(&r).into());

            Ok(1)
        }

        /// Add value of `operand` to `A`
        AddN(operand: u8) [2] => {
            perform_addition(registers, (*operand).into());

            Ok(2)
        }

        /// Add value of `HL` to `A`
        AddHL() [1] => {
            perform_addition(registers, registers.get_double(&crate::registers::DoubleRegister::HL) as u8);

            Ok(2)
        }

        /// Add value of `SingleRegister` and the Carry flag to `A`
        Adc(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let mut operand = registers.get_single(&r);

            if registers.is_carry() {
                operand = wrapping_add(operand, 1).0;
            }

            perform_addition(registers, operand);

            Ok(1)
        }

        /// Add value of `operand` and Carry to `A`
        AdcN(operand: u8) [2] => {
            let mut operand = *operand;

            if registers.is_carry() {
                operand = wrapping_add(operand, 1).0;
            }

            perform_addition(registers, operand);

            Ok(2)
        }

        /// Add value of `HL` and Carry to `A`
        AdcHL() [1] => {
            let mut operand = registers.get_double(&crate::registers::DoubleRegister::HL) as u8;

            if registers.is_carry() {
                operand = wrapping_add(operand, 1).0;
            }

            perform_addition(registers, operand);

            Ok(2)
        }

        /// Subtract value of `SingleRegister` from A
        Sub(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let operand = registers.get_single(&r);

            perform_subtraction(registers, operand);

            Ok(1)
        }

        /// Subtract value of `operand` from A
        SubN(operand: u8) [2] => {
            perform_subtraction(registers, *operand);

            Ok(2)
        }

        /// Subtract value of `HL` from A
        SubHL() [1] => {
            let operand = registers.get_double(&crate::registers::DoubleRegister::HL);

            perform_subtraction(registers, operand as u8);

            Ok(2)
        }

        /// Subtract value of `SingleRegister` and Carry from A
        Sbc(_r: SingleRegister) [1] => {
            unimplemented!()
        }

        /// Subtract value of `operand` and Carry from A
        SbcN(_operand: u8) [2] => {
            unimplemented!()
        }

        /// Subtract value of `HL` and Carry from A
        SbcHL() [1] => {
            unimplemented!()
        }

        /// Logical AND between register and `A`
        And(_r: SingleRegister) [1] => {
            unimplemented!()
        }

        /// Logical AND between `operand` and `A`
        AndN(_operand: u8) [2] => {
            unimplemented!()
        }

        /// Logical AND between `HL` and `A`
        AndHL() [1] => {
            unimplemented!()
        }

        /// Logical OR between register and `A`
        Or(_r: SingleRegister) [1] => {
            unimplemented!()
        }

        /// Logical OR between `operand` and `A`
        OrN(_operand: u8) [2] => {
            unimplemented!()
        }

        /// Logical OR between `HL` and `A`
        OrHL() [1] => {
            unimplemented!()
        }

        /// Logical XOR between register and `A`
        Xor(_r: SingleRegister) [1] => {
            unimplemented!()
        }

        /// Logical XOR between `operand` and `A`
        XorN(_operand: u8) [2] => {
            unimplemented!()
        }

        /// Logical XOR between `HL` and `A`
        XorHL() [1] => {
            unimplemented!()
        }

        /// Compare register and `A`
        Cp(_r: SingleRegister) [1] => {
            unimplemented!()
        }

        /// Compare `operand` and `A`
        CpN(_operand: u8) [2] => {
            unimplemented!()
        }

        /// Compare `HL` and `A`
        CpHL() [1] => {
            unimplemented!()
        }

        /// Increment `SingleRegister` by 1
        Inc(_r: SingleRegister) [1] => {
            unimplemented!()
        }

        /// Increment `HL` by 1
        IncHL() [1] => {
            unimplemented!()
        }

        /// Decrement `SingleRegister` by 1
        Dec(_r: SingleRegister) [1] => {
            unimplemented!()
        }

        /// Decrement `HL` by 1
        DecHL() [1] => {
            unimplemented!()
        }
    }
}

#[cfg(test)]
crate::instruction_tests! {
    add_takes_one_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::Add(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    add_adds_value_of_register_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 1);
        registers.set_single(&SingleRegister::B, 2);

        ALU8Bit::Add(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(3, registers.get_single(&SingleRegister::A));
    }

    add_sets_z_flag_if_result_is_zero(registers, memory, cpu_flags) => {
        assert_eq!(0b0000_0000, registers.get_single(&SingleRegister::F));

        registers.set_single(&SingleRegister::A, 0);
        registers.set_single(&SingleRegister::B, 0);

        ALU8Bit::Add(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1000_0000, registers.get_single(&SingleRegister::F));
    }

    add_sets_h_flag_if_carry_from_bit_3(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b0000_0111);
        registers.set_single(&SingleRegister::B, 0b0000_1001);

        ALU8Bit::Add(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0010_0000, registers.get_single(&SingleRegister::F));
    }

    add_sets_c_flag_if_carry_from_bit_7(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b1111_0000);
        registers.set_single(&SingleRegister::B, 0b0001_0001);

        ALU8Bit::Add(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0001_0000, registers.get_single(&SingleRegister::F));
    }

    add_handles_overflow(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 252);
        registers.set_single(&SingleRegister::B, 8);

        ALU8Bit::Add(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(4, registers.get_single(&SingleRegister::A));
    }

    add_does_support_the_f_register(registers, memory, cpu_flags) => {
        let result = ALU8Bit::Add(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(SingleRegister::F));

        assert_eq!(expected, result);
    }

    addn_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::AddN(0xAB).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles);
    }

    addn_adds_operand_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);

        ALU8Bit::AddN(2).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    addhl_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::AddHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles);
    }

    addhl_adds_hl_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);
        registers.set_double(&DoubleRegister::HL, 2);

        ALU8Bit::AddHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    add_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0x3A);
        registers.set_single(&SingleRegister::B, 0xC6);

        ALU8Bit::Add(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x00, registers.get_single(&SingleRegister::A), "Wrong result");
        assert_eq!(0b1011_0000, registers.get_single(&SingleRegister::F), "Incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3C);

        ALU8Bit::AddN(0xFF).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x3B, registers.get_single(&SingleRegister::A), "Wrong result");
        assert_eq!(0b0011_0000, registers.get_single(&SingleRegister::F), "Incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3C);
        registers.set_double(&DoubleRegister::HL, 0x12);

        ALU8Bit::AddHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x4E, registers.get_single(&SingleRegister::A), "Wrong result");
        assert_eq!(0b0000_0000, registers.get_single(&SingleRegister::F), "Incorrect flags");
    }

    adc_takes_1_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::Adc(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    adc_adds_register_plus_carry_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);
        registers.set_single(&SingleRegister::B, 2);
        registers.set_single(&SingleRegister::F, MASK_FLAG_CARRY);

        ALU8Bit::Adc(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(43, registers.get_single(&SingleRegister::A));

        registers.set_single(&SingleRegister::A, 40);
        registers.set_single(&SingleRegister::F, 0);

        ALU8Bit::Adc(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    adcn_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::AdcN(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles);
    }

    adcn_adds_register_plus_carry_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);
        registers.set_single(&SingleRegister::F, MASK_FLAG_CARRY);

        ALU8Bit::AdcN(2).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(43, registers.get_single(&SingleRegister::A));

        registers.set_single(&SingleRegister::A, 40);
        registers.set_single(&SingleRegister::F, 0);

        ALU8Bit::AdcN(2).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    adchl_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::AdcHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles);
    }

    adchl_adds_register_plus_carry_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);
        registers.set_double(&DoubleRegister::HL, 2);
        registers.set_single(&SingleRegister::F, MASK_FLAG_CARRY);

        ALU8Bit::AdcHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(43, registers.get_single(&SingleRegister::A));

        registers.set_single(&SingleRegister::A, 40);
        registers.set_single(&SingleRegister::F, 0);

        ALU8Bit::AdcHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    sub_takes_1_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::Sub(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    sub_subtracts_register_from_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 45);
        registers.set_single(&SingleRegister::B, 3);

        ALU8Bit::Sub(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    sub_sets_the_negative_flag(registers, memory, cpu_flags) => {
        assert_eq!(false, registers.is_negative());

        ALU8Bit::Sub(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(true, registers.is_negative());
    }

    sub_sets_the_zero_flag_if_result_is_zero(registers, memory, cpu_flags) => {
        assert_eq!(false, registers.is_zero());

        ALU8Bit::Sub(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(true, registers.is_zero());
    }

    sub_resets_the_zero_flag_if_result_is_non_zero(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 45);
        registers.set_single(&SingleRegister::B, 3);

        assert_eq!(false, registers.is_zero());

        ALU8Bit::Sub(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(false, registers.is_zero());
    }

    sub_handles_overflow(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 10);
        registers.set_single(&SingleRegister::B, 15);

        ALU8Bit::Sub(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(251, registers.get_single(&SingleRegister::A));
    }

    sub_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0x3E);
        registers.set_single(&SingleRegister::E, 0x3E);
        registers.set_double(&DoubleRegister::HL, 0x40);

        ALU8Bit::Sub(SingleRegister::E).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x00, registers.get_single(&SingleRegister::A), "Sub has wrong result");
        assert_eq!(0b1100_0000, registers.get_single(&SingleRegister::F), "Sub sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3E);

        ALU8Bit::SubN(0x0F).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x2F, registers.get_single(&SingleRegister::A), "SubN has wrong result");
        assert_eq!(0b0110_0000, registers.get_single(&SingleRegister::F), "SubN sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3E);

        ALU8Bit::SubHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0xFE, registers.get_single(&SingleRegister::A), "SubHL has wrong result");
        assert_eq!(0b0101_0000, registers.get_single(&SingleRegister::F), "SubN sets incorrect flags");
    }
}
