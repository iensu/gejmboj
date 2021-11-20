use crate::{
    errors::CpuError,
    instruction_group,
    registers::{
        DoubleRegister, Registers, SingleRegister, MASK_FLAG_CARRY, MASK_FLAG_HALF_CARRY,
        MASK_FLAG_NEGATIVE, MASK_FLAG_ZERO,
    },
};

instruction_group! {
    /// 8-bit ALU (math) instructions
    ALU8Bit (registers, memory, _cpu_flags) {

        /// Add value of `SingleRegister` to `A`
        Add(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            perform_calculation(AluOp::Add, registers, registers.get_single(&r).into(), false);

            Ok(1)
        }

        /// Add value of `operand` to `A`
        AddN(operand: u8) [2] => {
            perform_calculation(AluOp::Add, registers, (*operand).into(), false);

            Ok(2)
        }

        /// Add value of `(HL)` to `A`
        AddHL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL).into());
            perform_calculation(AluOp::Add, registers, operand, false);

            Ok(2)
        }

        /// Add value of `SingleRegister` and the Carry flag to `A`
        Adc(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            perform_calculation(AluOp::Add, registers, registers.get_single(&r), true);

            Ok(1)
        }

        /// Add value of `operand` and Carry to `A`
        AdcN(operand: u8) [2] => {
            perform_calculation(AluOp::Add, registers, *operand, true);

            Ok(2)
        }

        /// Add value of `(HL)` and Carry to `A`
        AdcHL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL).into());
            perform_calculation(AluOp::Add, registers, operand, true);

            Ok(2)
        }

        /// Subtract value of `SingleRegister` from A
        Sub(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let operand = registers.get_single(&r);

            perform_calculation(AluOp::Sub, registers, operand, false);

            Ok(1)
        }

        /// Subtract value of `operand` from A
        SubN(operand: u8) [2] => {
            perform_calculation(AluOp::Sub, registers, *operand, false);

            Ok(2)
        }

        /// Subtract value of `(HL)` from A
        SubHL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL).into());

            perform_calculation(AluOp::Sub, registers, operand, false);

            Ok(2)
        }

        /// Subtract value of `SingleRegister` and Carry from A
        Sbc(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let operand = registers.get_single(r);

            perform_calculation(AluOp::Sub, registers, operand, true);

            Ok(1)
        }

        /// Subtract value of `operand` and Carry from A
        SbcN(operand: u8) [2] => {
            perform_calculation(AluOp::Sub, registers, *operand, true);

            Ok(2)
        }

        /// Subtract value of `(HL)` and Carry from A
        SbcHL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL).into());
            perform_calculation(AluOp::Sub, registers, operand, true);

            Ok(2)
        }

        /// Logical AND between register and `A`
        And(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r))
            }

            perform_calculation(AluOp::And, registers, registers.get_single(r), false);

            Ok(1)
        }

        /// Logical AND between `operand` and `A`
        AndN(operand: u8) [2] => {
            perform_calculation(AluOp::And, registers, *operand, false);

            Ok(2)
        }

        /// Logical AND between `(HL)` and `A`
        AndHL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL).into());
            perform_calculation(AluOp::And, registers, operand, false);

            Ok(2)
        }

        /// Logical OR between register and `A`
        Or(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r))
            }

            let operand = registers.get_single(r);
            perform_calculation(AluOp::Or, registers, operand, false);

            Ok(1)
        }

        /// Logical OR between `operand` and `A`
        OrN(operand: u8) [2] => {
            perform_calculation(AluOp::Or, registers, *operand, false);

            Ok(2)
        }

        /// Logical OR between `(HL)` and `A`
        OrHL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL).into());
            perform_calculation(AluOp::Or, registers, operand, false);

            Ok(2)
        }

        /// Logical XOR between register and `A`
        Xor(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let operand = registers.get_single(r);
            perform_calculation(AluOp::Xor, registers, operand, false);

            Ok(1)
        }

        /// Logical XOR between `operand` and `A`
        XorN(operand: u8) [2] => {
            perform_calculation(AluOp::Xor, registers, *operand, false);

            Ok(2)
        }

        /// Logical XOR between `(HL)` and `A`
        XorHL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL).into());
            perform_calculation(AluOp::Xor, registers, operand, false);

            Ok(2)
        }

        /// Compare register and `A`
        ///
        /// Basically an A - n subtraction but with the result being thrown away,
        /// so the same flag rules as `Sub` apply.
        Cp(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }
            let operand = registers.get_single(r);
            let a = registers.get_single(&SingleRegister::A);

            let (_, flags) = AluOp::Cp.calculate(a, operand);

            registers.set_flags(flags);
            Ok(1)
        }

        /// Compare `operand` and `A`
        CpN(operand: u8) [2] => {
            let a = registers.get_single(&SingleRegister::A);

            let (_, flags) = AluOp::Cp.calculate(a, *operand);

            registers.set_flags(flags);

            Ok(2)
        }

        /// Compare `(HL)` and `A`
        CpHL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL).into());
            let a = registers.get_single(&SingleRegister::A);

            let (_, flags) = AluOp::Cp.calculate(a, operand);

            registers.set_flags(flags);

            Ok(2)
        }

        /// Increment `SingleRegister` by 1
        ///
        /// The Carry flag is unaffected by this instruction.
        Inc(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let operand = registers.get_single(r);
            let (result, flags) = AluOp::Add.calculate(operand, 1);
            // Set Carry if already set, otherwise reset
            let flags = if registers.is_carry() { flags | MASK_FLAG_CARRY } else { flags & 0b1110_0000 };

            registers.set_single(r, result);
            registers.set_flags(flags);

            Ok(1)
        }

        /// Increment `HL` by 1
        ///
        /// The Carry flag is unaffected by this instruction.
        IncHL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL).into());
            let (result, flags) = AluOp::Add.calculate(operand, 1);
            // Set Carry if already set, otherwise reset
            let flags = if registers.is_carry() { flags | MASK_FLAG_CARRY } else { flags & 0b1110_0000 };

            memory.set(registers.get_double(&DoubleRegister::HL).into(), result);
            registers.set_flags(flags);

            Ok(3)
        }

        /// Decrement `SingleRegister` by 1
        ///
        /// The Carry flag is unaffected by this instruction.
        Dec(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let operand = registers.get_single(r);
            let (result, flags) = AluOp::Sub.calculate(operand, 1);
            // Set Carry if already set, otherwise reset
            let flags = if registers.is_carry() { flags | MASK_FLAG_CARRY } else { flags & 0b1110_0000 };

            registers.set_single(r, result);
            registers.set_flags(flags);

            Ok(1)
        }

        /// Decrement `HL` by 1
        ///
        /// The Carry flag is unaffected by this instruction.
        DecHL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL).into());
            let (result, flags) = AluOp::Sub.calculate(operand, 1);
            // Set Carry if already set, otherwise reset
            let flags = if registers.is_carry() { flags | MASK_FLAG_CARRY } else { flags & 0b1110_0000 };

            memory.set(registers.get_double(&DoubleRegister::HL).into(), result);
            registers.set_flags(flags);

            Ok(3)
        }
    }
}

fn perform_calculation(op: AluOp, registers: &mut Registers, operand: u8, add_carry: bool) {
    let a = registers.get_single(&SingleRegister::A);
    let operand = if add_carry && registers.is_carry() {
        operand.wrapping_add(1)
    } else {
        operand
    };

    let (result, flags) = op.calculate(a, operand);

    registers.set_single(&SingleRegister::A, result);
    registers.set_flags(flags);
}

enum AluOp {
    Sub,
    Add,
    And,
    Or,
    Xor,
    Cp,
}

impl AluOp {
    pub fn calculate(&self, a: u8, operand: u8) -> (u8, u8) {
        match &self {
            AluOp::Sub | AluOp::Cp => {
                let (result, is_carry) = a.overflowing_sub(operand);

                let mut flags = 0b0000_0000 | MASK_FLAG_NEGATIVE;

                if result == 0 {
                    flags = flags | MASK_FLAG_ZERO; // Set Z
                }
                // Check if the 5th bit has changed in the result
                if result != 0 && (result & 0x10) != (a & 0x10) {
                    flags = flags | MASK_FLAG_HALF_CARRY; // Set H
                }
                if is_carry {
                    flags = flags | MASK_FLAG_CARRY; // Set C
                }

                (result, flags)
            }
            AluOp::Add => {
                let (result, is_carry) = a.overflowing_add(operand);
                let mut flags = 0b0000_0000;

                if result == 0 {
                    flags = flags | MASK_FLAG_ZERO; // Set Z
                }
                if (a ^ operand ^ result) & 0x10 > 0 {
                    flags = flags | MASK_FLAG_HALF_CARRY; // Set H
                }
                if is_carry {
                    flags = flags | MASK_FLAG_CARRY; // Set C
                }

                (result, flags)
            }
            AluOp::And => {
                let result = a & operand;
                let mut flags = 0b0010_0000;

                if result == 0 {
                    flags |= MASK_FLAG_ZERO;
                }

                (result, flags)
            }
            AluOp::Or => {
                let result = a | operand;

                let flags = if result == 0 { MASK_FLAG_ZERO } else { 0 };

                (result, flags)
            }
            AluOp::Xor => {
                let result = a ^ operand;
                let flags = if result == 0 { MASK_FLAG_ZERO } else { 0 };

                (result, flags)
            }
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
        assert_eq!(0b0000_0000, registers.get_flags());

        registers.set_single(&SingleRegister::A, 0);
        registers.set_single(&SingleRegister::B, 0);

        ALU8Bit::Add(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1000_0000, registers.get_flags());
    }

    add_sets_h_flag_if_carry_from_bit_3(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b0000_0111);
        registers.set_single(&SingleRegister::B, 0b0000_1001);

        ALU8Bit::Add(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0010_0000, registers.get_flags());
    }

    add_sets_c_flag_if_carry_from_bit_7(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b1111_0000);
        registers.set_single(&SingleRegister::B, 0b0001_0001);

        ALU8Bit::Add(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0001_0000, registers.get_flags());
    }

    add_handles_overflow(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 252);
        registers.set_single(&SingleRegister::B, 8);

        ALU8Bit::Add(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(4, registers.get_single(&SingleRegister::A));
    }

    add_does_not_support_the_f_register(registers, memory, cpu_flags) => {
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
        memory.set(registers.get_double(&DoubleRegister::HL).into(), 2);

        ALU8Bit::AddHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    add_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0x3A);
        registers.set_single(&SingleRegister::B, 0xC6);

        ALU8Bit::Add(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x00, registers.get_single(&SingleRegister::A), "Wrong result");
        assert_eq!(0b1011_0000, registers.get_flags(), "Incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3C);

        ALU8Bit::AddN(0xFF).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x3B, registers.get_single(&SingleRegister::A), "Wrong result");
        assert_eq!(0b0011_0000, registers.get_flags(), "Incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3C);
        memory.set(registers.get_double(&DoubleRegister::HL).into(), 0x12);

        ALU8Bit::AddHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x4E, registers.get_single(&SingleRegister::A), "Wrong result");
        assert_eq!(0b0000_0000, registers.get_flags(), "Incorrect flags");
    }

    adc_takes_1_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::Adc(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    adc_adds_register_plus_carry_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);
        registers.set_single(&SingleRegister::B, 2);
        registers.set_flags(MASK_FLAG_CARRY);

        ALU8Bit::Adc(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(43, registers.get_single(&SingleRegister::A));

        registers.set_single(&SingleRegister::A, 40);
        registers.set_flags(0);

        ALU8Bit::Adc(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    adcn_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::AdcN(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles);
    }

    adcn_adds_register_plus_carry_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);
        registers.set_flags(MASK_FLAG_CARRY);

        ALU8Bit::AdcN(2).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(43, registers.get_single(&SingleRegister::A));

        registers.set_single(&SingleRegister::A, 40);
        registers.set_flags(0);

        ALU8Bit::AdcN(2).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    adchl_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::AdcHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles);
    }

    adchl_adds_register_plus_carry_to_a(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 40);
        memory.set(registers.get_double(&DoubleRegister::HL).into(), 2);
        registers.set_flags(MASK_FLAG_CARRY);

        ALU8Bit::AdcHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(43, registers.get_single(&SingleRegister::A));

        registers.set_single(&SingleRegister::A, 40);
        registers.set_flags(0);

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
        registers.set_single(&SingleRegister::E, 0x3E);
        memory.set(registers.get_double(&DoubleRegister::HL).into(), 0x40);
        registers.set_single(&SingleRegister::A, 0x3E);

        ALU8Bit::Sub(SingleRegister::E).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x00, registers.get_single(&SingleRegister::A), "Sub has wrong result");
        assert_eq!(0b1100_0000, registers.get_flags(), "Sub sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3E);

        ALU8Bit::SubN(0x0F).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x2F, registers.get_single(&SingleRegister::A), "SubN has wrong result");
        assert_eq!(0b0110_0000, registers.get_flags(), "SubN sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3E);

        ALU8Bit::SubHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0xFE, registers.get_single(&SingleRegister::A), "SubHL has wrong result");
        assert_eq!(0b0101_0000, registers.get_flags(), "SubN sets incorrect flags");
    }

    sbc_takes_the_correct_amount_of_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::Sbc(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles, "Incorrect machine cycle count for Sbc");

        let cycles = ALU8Bit::SbcN(42).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles, "Incorrect machine cycle count for SbcN");

        let cycles = ALU8Bit::SbcHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles, "Incorrect machine cycle count for SbcHL");
    }

    sbc_computes_and_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::H, 0x2A);
        memory.set(registers.get_double(&DoubleRegister::HL).into(), 0x4F);
        registers.set_single(&SingleRegister::A, 0x3B);
        registers.set_flags(0b0001_0000);

        ALU8Bit::Sbc(SingleRegister::H).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x10, registers.get_single(&SingleRegister::A), "Sbc has wrong result");
        assert_eq!(0b0100_0000, registers.get_flags(), "Sbc sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3B);
        registers.set_flags(0b0001_0000);

        ALU8Bit::SbcN(0x3A).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x00, registers.get_single(&SingleRegister::A), "SbcN has wrong result");
        assert_eq!(0b1100_0000, registers.get_flags(), "SbcN sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3B);

        registers.set_flags(0b0001_0000);

        ALU8Bit::SbcHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0xEB, registers.get_single(&SingleRegister::A), "SbcHL has wrong result");
        assert_eq!(0b0111_0000, registers.get_flags(), "SbcHL sets incorrect flags");
    }

    and_takes_the_correct_amount_of_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::And(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles, "Incorrect machine cycle count for And");

        let cycles = ALU8Bit::AndN(42).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles, "Incorrect machine cycle count for AndN");

        let cycles = ALU8Bit::AndHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles, "Incorrect machine cycle count for AndHL");
    }

    and_does_not_support_the_f_register(registers, memory, cpu_flags) => {
        let result = ALU8Bit::And(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(SingleRegister::F));

        assert_eq!(expected, result);
    }

    and_computes_and_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0x5A);
        registers.set_single(&SingleRegister::L, 0x3F);

        ALU8Bit::And(SingleRegister::L).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x1A, registers.get_single(&SingleRegister::A), "And has wrong result");
        assert_eq!(0b0010_0000, registers.get_flags(), "And sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0x5A);

        ALU8Bit::AndN(0x38).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x18, registers.get_single(&SingleRegister::A), "AndN has wrong result");
        assert_eq!(0b0010_0000, registers.get_flags(), "AndN sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0x5A);
        registers.set_double(&DoubleRegister::HL, 0x00);

        ALU8Bit::AndHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x00, registers.get_single(&SingleRegister::A), "AndHL has wrong result");
        assert_eq!(0b1010_0000, registers.get_flags(), "AndHL sets incorrect flags");
    }

    or_takes_the_correct_amount_of_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::Or(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles, "Incorrect machine cycle count for Or");

        let cycles = ALU8Bit::OrN(42).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles, "Incorrect machine cycle count for OrN");

        let cycles = ALU8Bit::OrHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles, "Incorrect machine cycle count for OrHL");
    }

    or_does_not_support_the_f_register(registers, memory, cpu_flags) => {
        let result = ALU8Bit::Or(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(SingleRegister::F));

        assert_eq!(expected, result);
    }

    or_computes_and_handles_flags_correctly(registers, memory, cpu_flags) => {
        memory.set(registers.get_double(&DoubleRegister::HL).into(), 0x0F);
        registers.set_single(&SingleRegister::A, 0x5A);

        ALU8Bit::Or(SingleRegister::A).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x5A, registers.get_single(&SingleRegister::A), "Or has wrong result");
        assert_eq!(0b0000_0000, registers.get_flags(), "Or sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0x5A);

        ALU8Bit::OrN(0x03).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x5B, registers.get_single(&SingleRegister::A), "OrN has wrong result");
        assert_eq!(0b0000_0000, registers.get_flags(), "OrN sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0x5A);

        ALU8Bit::OrHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x5F, registers.get_single(&SingleRegister::A), "OrHL has wrong result");
        assert_eq!(0b0000_0000, registers.get_flags(), "OrHL sets incorrect flags");
    }

    xor_takes_the_correct_amount_of_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::Xor(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles, "Incorrect machine cycle count for Xor");

        let cycles = ALU8Bit::XorN(42).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles, "Incorrect machine cycle count for XorN");

        let cycles = ALU8Bit::XorHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles, "Incorrect machine cycle count for XorHL");
    }

    xor_does_not_support_the_f_register(registers, memory, cpu_flags) => {
        let result = ALU8Bit::Xor(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(SingleRegister::F));

        assert_eq!(expected, result);
    }

    xor_computes_and_handles_flags_correctly(registers, memory, cpu_flags) => {
        memory.set(registers.get_double(&DoubleRegister::HL).into(), 0x8A);
        registers.set_single(&SingleRegister::A, 0xFF);

        ALU8Bit::Xor(SingleRegister::A).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x00, registers.get_single(&SingleRegister::A), "Xor has wrong result");
        assert_eq!(0b1000_0000, registers.get_flags(), "Xor sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0xFF);

        ALU8Bit::XorN(0x0F).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0xF0, registers.get_single(&SingleRegister::A), "XorN has wrong result");
        assert_eq!(0b0000_0000, registers.get_flags(), "XorN sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0xFF);

        ALU8Bit::XorHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x75, registers.get_single(&SingleRegister::A), "XorHL has wrong result");
        assert_eq!(0b0000_0000, registers.get_flags(), "XorHL sets incorrect flags");
    }

    cp_takes_the_correct_amount_of_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::Cp(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles, "Incorrect machine cycle count for Cp");

        let cycles = ALU8Bit::CpN(42).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles, "Incorrect machine cycle count for CpN");

        let cycles = ALU8Bit::CpHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(2, cycles, "Incorrect machine cycle count for CpHL");
    }

    cp_does_not_support_the_f_register(registers, memory, cpu_flags) => {
        let result = ALU8Bit::Cp(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(SingleRegister::F));

        assert_eq!(expected, result);
    }

    cp_handles_flags_correctly(registers, memory, cpu_flags) => {
        memory.set(registers.get_double(&DoubleRegister::HL).into(), 0x40);
        registers.set_single(&SingleRegister::B, 0x2F);
        registers.set_single(&SingleRegister::A, 0x3C);

        ALU8Bit::Cp(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0110_0000, registers.get_flags(), "Cp sets incorrect flags");

        ALU8Bit::CpN(0x3C).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1100_0000, registers.get_flags(), "CpN sets incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3C);

        ALU8Bit::CpHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0101_0000, registers.get_flags(), "CpHL sets incorrect flags");
    }

    inc_takes_the_correct_amount_of_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::Inc(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles, "Incorrect machine cycle count for Inc");

        let cycles = ALU8Bit::IncHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(3, cycles, "Incorrect machine cycle count for IncHL");
    }

    inc_does_not_support_the_f_register(registers, memory, cpu_flags) => {
        let result = ALU8Bit::Inc(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(SingleRegister::F));

        assert_eq!(expected, result);
    }

    inc_handles_flags_correctly(registers, memory, cpu_flags) => {
        memory.set(registers.get_double(&DoubleRegister::HL).into(), 0x50);
        registers.set_single(&SingleRegister::A, 0xFF);

        ALU8Bit::Inc(SingleRegister::A).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0, registers.get_single(&SingleRegister::A), "Inc sets wrong result");
        assert_eq!(0b1010_0000, registers.get_flags(), "Inc sets incorrect flags");

        ALU8Bit::IncHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x51, memory.get(registers.get_double(&DoubleRegister::HL).into()), "IncHL sets wrong result");
        assert_eq!(0b0000_0000, registers.get_flags(), "IncHL sets incorrect flags");

        registers.set_flags(MASK_FLAG_CARRY);
        ALU8Bit::Inc(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "Inc did not maintain Carry flag");

        registers.set_flags(MASK_FLAG_CARRY);
        ALU8Bit::IncHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "IncHL did not maintain Carry flag");
    }

    dec_takes_the_correct_amount_of_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU8Bit::Dec(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles, "Incorrect machine cycle count for Dec");

        let cycles = ALU8Bit::DecHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(3, cycles, "Incorrect machine cycle count for DecHL");
    }

    dec_does_not_support_the_f_register(registers, memory, cpu_flags) => {
        let result = ALU8Bit::Dec(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(SingleRegister::F));

        assert_eq!(expected, result);
    }

    dec_handles_flags_correctly(registers, memory, cpu_flags) => {
        memory.set(registers.get_double(&DoubleRegister::HL).into(), 0x00);
        registers.set_single(&SingleRegister::A, 0x01);
        registers.set_single(&SingleRegister::C, 0x02);

        ALU8Bit::Dec(SingleRegister::A).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0, registers.get_single(&SingleRegister::A), "Dec sets wrong result");
        assert_eq!(0b1100_0000, registers.get_flags(), "Dec sets incorrect flags");

        ALU8Bit::DecHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0xFF, memory.get(registers.get_double(&DoubleRegister::HL).into()), "DecHL sets wrong result");
        assert_eq!(0b0110_0000, registers.get_flags(), "DecHL sets incorrect flags");

        registers.set_flags(MASK_FLAG_CARRY);
        ALU8Bit::Dec(SingleRegister::C).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0101_0000, registers.get_flags(), "Dec did not maintain Carry flag");

        registers.set_flags(MASK_FLAG_CARRY);
        ALU8Bit::DecHL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0101_0000, registers.get_flags(), "DecHL did not maintain Carry flag");
    }
}
