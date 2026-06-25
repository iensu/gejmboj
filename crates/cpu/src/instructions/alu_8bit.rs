use crate::{
    cycles::MachineCycles,
    errors::CpuError,
    instruction_group,
    registers::{
        DoubleRegister, MASK_FLAG_CARRY, MASK_FLAG_HALF_CARRY, MASK_FLAG_NEGATIVE, MASK_FLAG_ZERO,
        Registers, SingleRegister,
    },
};

instruction_group! {
    /// 8-bit ALU (math) instructions
    ALU8Bit (registers, memory, _cpu_flags) {

        /// Add value of `SingleRegister` to `A`
        ADD(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            perform_calculation(&AluOp::Add, registers, registers.get_single(r), false);

            Ok(MachineCycles::new(1))
        }

        /// Add value of `operand` to `A`
        ADD_N(operand: u8) [2] => {
            perform_calculation(&AluOp::Add, registers, *operand , false);

            Ok(MachineCycles::new(2))
        }

        /// Add value of `(HL)` to `A`
        ADD_HL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL));
            perform_calculation(&AluOp::Add, registers, operand, false);

            Ok(MachineCycles::new(2))
        }

        /// Add value of `SingleRegister` and the Carry flag to `A`
        ADC(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            perform_calculation(&AluOp::Add, registers, registers.get_single(r), true);

            Ok(MachineCycles::new(1))
        }

        /// Add value of `operand` and Carry to `A`
        ADC_N(operand: u8) [2] => {
            perform_calculation(&AluOp::Add, registers, *operand, true);

            Ok(MachineCycles::new(2))
        }

        /// Add value of `(HL)` and Carry to `A`
        ADC_HL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL));
            perform_calculation(&AluOp::Add, registers, operand, true);

            Ok(MachineCycles::new(2))
        }

        /// Subtract value of `SingleRegister` from A
        SUB(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let operand = registers.get_single(r);

            perform_calculation(&AluOp::Sub, registers, operand, false);

            Ok(MachineCycles::new(1))
        }

        /// Subtract value of `operand` from A
        SUB_N(operand: u8) [2] => {
            perform_calculation(&AluOp::Sub, registers, *operand, false);

            Ok(MachineCycles::new(2))
        }

        /// Subtract value of `(HL)` from A
        SUB_HL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL));

            perform_calculation(&AluOp::Sub, registers, operand, false);

            Ok(MachineCycles::new(2))
        }

        /// Subtract value of `SingleRegister` and Carry from A
        SBC(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let operand = registers.get_single(r);

            perform_calculation(&AluOp::Sub, registers, operand, true);

            Ok(MachineCycles::new(1))
        }

        /// Subtract value of `operand` and Carry from A
        SBC_N(operand: u8) [2] => {
            perform_calculation(&AluOp::Sub, registers, *operand, true);

            Ok(MachineCycles::new(2))
        }

        /// Subtract value of `(HL)` and Carry from A
        SBC_HL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL));
            perform_calculation(&AluOp::Sub, registers, operand, true);

            Ok(MachineCycles::new(2))
        }

        /// Logical AND between register and `A`
        AND(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r))
            }

            perform_calculation(&AluOp::And, registers, registers.get_single(r), false);

            Ok(MachineCycles::new(1))
        }

        /// Logical AND between `operand` and `A`
        AND_N(operand: u8) [2] => {
            perform_calculation(&AluOp::And, registers, *operand, false);

            Ok(MachineCycles::new(2))
        }

        /// Logical AND between `(HL)` and `A`
        AND_HL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL));
            perform_calculation(&AluOp::And, registers, operand, false);

            Ok(MachineCycles::new(2))
        }

        /// Logical OR between register and `A`
        OR(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r))
            }

            let operand = registers.get_single(r);
            perform_calculation(&AluOp::Or, registers, operand, false);

            Ok(MachineCycles::new(1))
        }

        /// Logical OR between `operand` and `A`
        OR_N(operand: u8) [2] => {
            perform_calculation(&AluOp::Or, registers, *operand, false);

            Ok(MachineCycles::new(2))
        }

        /// Logical OR between `(HL)` and `A`
        OR_HL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL));
            perform_calculation(&AluOp::Or, registers, operand, false);

            Ok(MachineCycles::new(2))
        }

        /// Logical XOR between register and `A`
        XOR(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let operand = registers.get_single(r);
            perform_calculation(&AluOp::Xor, registers, operand, false);

            Ok(MachineCycles::new(1))
        }

        /// Logical XOR between `operand` and `A`
        XOR_N(operand: u8) [2] => {
            perform_calculation(&AluOp::Xor, registers, *operand, false);

            Ok(MachineCycles::new(2))
        }

        /// Logical XOR between `(HL)` and `A`
        XOR_HL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL));
            perform_calculation(&AluOp::Xor, registers, operand, false);

            Ok(MachineCycles::new(2))
        }

        /// Compare register and `A`
        ///
        /// Basically an A - n subtraction but with the result being thrown away,
        /// so the same flag rules as `Sub` apply.
        CP(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }
            let operand = registers.get_single(r);
            let a = registers.get_single(&SingleRegister::A);

            let (_, flags) = AluOp::Cp.calculate(a, operand, false);

            registers.set_flags(flags);
            Ok(MachineCycles::new(1))
        }

        /// Compare `operand` and `A`
        CP_N(operand: u8) [2] => {
            let a = registers.get_single(&SingleRegister::A);

            let (_, flags) = AluOp::Cp.calculate(a, *operand, false);

            registers.set_flags(flags);

            Ok(MachineCycles::new(2))
        }

        /// Compare `(HL)` and `A`
        CP_HL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL));
            let a = registers.get_single(&SingleRegister::A);

            let (_, flags) = AluOp::Cp.calculate(a, operand, false);

            registers.set_flags(flags);

            Ok(MachineCycles::new(2))
        }

        /// Increment `SingleRegister` by 1
        ///
        /// The Carry flag is unaffected by this instruction.
        INC(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let operand = registers.get_single(r);
            let (result, flags) = AluOp::Add.calculate(operand, 1, false);
            // Set Carry if already set, otherwise reset
            let flags = if registers.is_carry() { flags | MASK_FLAG_CARRY } else { flags & 0b1110_0000 };

            registers.set_single(r, result);
            registers.set_flags(flags);

            Ok(MachineCycles::new(1))
        }

        /// Increment `HL` by 1
        ///
        /// The Carry flag is unaffected by this instruction.
        INC_HL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL));
            let (result, flags) = AluOp::Add.calculate(operand, 1, false);
            // Set Carry if already set, otherwise reset
            let flags = if registers.is_carry() { flags | MASK_FLAG_CARRY } else { flags & 0b1110_0000 };

            memory.set(registers.get_double(&DoubleRegister::HL), result);
            registers.set_flags(flags);

            Ok(MachineCycles::new(3))
        }

        /// Decrement `SingleRegister` by 1
        ///
        /// The Carry flag is unaffected by this instruction.
        DEC(r: SingleRegister) [1] => {
            if *r == SingleRegister::F {
                return Err(CpuError::UnsupportedSingleRegister(*r));
            }

            let operand = registers.get_single(r);
            let (result, flags) = AluOp::Sub.calculate(operand, 1, false);
            // Set Carry if already set, otherwise reset
            let flags = if registers.is_carry() { flags | MASK_FLAG_CARRY } else { flags & 0b1110_0000 };

            registers.set_single(r, result);
            registers.set_flags(flags);

            Ok(MachineCycles::new(1))
        }

        /// Decrement `HL` by 1
        ///
        /// The Carry flag is unaffected by this instruction.
        DEC_HL() [1] => {
            let operand = memory.get(registers.get_double(&DoubleRegister::HL));
            let (result, flags) = AluOp::Sub.calculate(operand, 1, false);
            // Set Carry if already set, otherwise reset
            let flags = if registers.is_carry() { flags | MASK_FLAG_CARRY } else { flags & 0b1110_0000 };

            memory.set(registers.get_double(&DoubleRegister::HL), result);
            registers.set_flags(flags);

            Ok(MachineCycles::new(3))
        }
    }
}

fn perform_calculation(op: &AluOp, registers: &mut Registers, operand: u8, add_carry: bool) {
    let a = registers.get_single(&SingleRegister::A);

    let (result, flags) = op.calculate(a, operand, add_carry && registers.is_carry());

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
    pub fn calculate(&self, a: u8, operand: u8, include_carry: bool) -> (u8, u8) {
        match &self {
            Self::Sub | Self::Cp => {
                let a: u16 = a.into();
                let a_expanded: u16 = 0xFF00 | a;
                let operand: u16 = operand.into();
                let carry: u16 = include_carry.into();
                let result = a_expanded - operand - carry;

                let mut flags = MASK_FLAG_NEGATIVE;

                // Check if borrow is necessary in the lowest nibble
                if (a & 0xF) < ((operand & 0xF) + carry) {
                    flags |= MASK_FLAG_HALF_CARRY; // Set H
                }
                if result & 0xF00 != 0xF00 {
                    flags |= MASK_FLAG_CARRY; // Set C
                }

                #[allow(clippy::cast_possible_truncation)]
                let result = result as u8;
                if result == 0 {
                    flags |= MASK_FLAG_ZERO; // Set Z
                }

                (result, flags)
            }
            Self::Add => {
                let carry: u16 = include_carry.into();
                let a = u16::from(a);
                let operand = u16::from(operand);
                let result = a + operand + carry;
                let mut flags = 0b0000_0000;

                if ((a & 0xF) + (operand & 0xF) + carry) & 0x10 > 0 {
                    flags |= MASK_FLAG_HALF_CARRY; // Set H
                }
                if result > 0xFF {
                    flags |= MASK_FLAG_CARRY; // Set C
                }

                let result = (result & 0xFF) as u8;
                if result == 0 {
                    flags |= MASK_FLAG_ZERO; // Set Z
                }

                (result, flags)
            }
            Self::And => {
                let result = a & operand;
                let mut flags = 0b0010_0000;

                if result == 0 {
                    flags |= MASK_FLAG_ZERO;
                }

                (result, flags)
            }
            Self::Or => {
                let result = a | operand;

                let flags = if result == 0 { MASK_FLAG_ZERO } else { 0 };

                (result, flags)
            }
            Self::Xor => {
                let result = a ^ operand;
                let flags = if result == 0 { MASK_FLAG_ZERO } else { 0 };

                (result, flags)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use crate::registers::*;
    use crate::test_utils::setup;

    #[test]
    fn add_takes_one_machine_cycle() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::ADD(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(1, cycles.value());
    }

    #[test]
    fn add_adds_value_of_register_to_a() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 1);
        registers.set_single(&SingleRegister::B, 2);

        ALU8Bit::ADD(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(3, registers.get_single(&SingleRegister::A));
    }

    #[test]
    fn add_sets_z_flag_if_result_is_zero() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        assert_eq!(0b0000_0000, registers.get_flags());

        registers.set_single(&SingleRegister::A, 0);
        registers.set_single(&SingleRegister::B, 0);

        ALU8Bit::ADD(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(0b1000_0000, registers.get_flags());
    }

    #[test]
    fn add_sets_h_flag_if_carry_from_bit_3() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 0b0000_0111);
        registers.set_single(&SingleRegister::B, 0b0000_1001);

        ALU8Bit::ADD(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(0b0010_0000, registers.get_flags());
    }

    #[test]
    fn add_sets_c_flag_if_carry_from_bit_7() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 0b1111_0000);
        registers.set_single(&SingleRegister::B, 0b0001_0001);

        ALU8Bit::ADD(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(0b0001_0000, registers.get_flags());
    }

    #[test]
    fn add_handles_overflow() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 252);
        registers.set_single(&SingleRegister::B, 8);

        ALU8Bit::ADD(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(4, registers.get_single(&SingleRegister::A));
    }

    #[test]
    fn add_does_not_support_the_f_register() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let result =
            ALU8Bit::ADD(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(
            SingleRegister::F,
        ));

        assert_eq!(expected, result);
    }

    #[test]
    fn addn_takes_2_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::ADD_N(0xAB)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value());
    }

    #[test]
    fn addn_adds_operand_to_a() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 40);

        ALU8Bit::ADD_N(2)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    #[test]
    fn addhl_takes_2_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::ADD_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value());
    }

    #[test]
    fn addhl_adds_hl_to_a() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 40);
        memory.set(registers.get_double(&DoubleRegister::HL), 2);

        ALU8Bit::ADD_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    #[test]
    fn add_handles_flags_correctly() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 0x3A);
        registers.set_single(&SingleRegister::B, 0xC6);

        ALU8Bit::ADD(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x00,
            registers.get_single(&SingleRegister::A),
            "Wrong result"
        );
        assert_eq!(0b1011_0000, registers.get_flags(), "Incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3C);

        ALU8Bit::ADD_N(0xFF)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x3B,
            registers.get_single(&SingleRegister::A),
            "Wrong result"
        );
        assert_eq!(0b0011_0000, registers.get_flags(), "Incorrect flags");

        registers.set_single(&SingleRegister::A, 0x3C);
        memory.set(registers.get_double(&DoubleRegister::HL), 0x12);

        ALU8Bit::ADD_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x4E,
            registers.get_single(&SingleRegister::A),
            "Wrong result"
        );
        assert_eq!(0b0000_0000, registers.get_flags(), "Incorrect flags");
    }

    #[test]
    fn adc_takes_1_machine_cycle() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::ADC(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(1, cycles.value());
    }

    #[test]
    fn adc_adds_register_plus_carry_to_a() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 40);
        registers.set_single(&SingleRegister::B, 2);
        registers.set_flags(MASK_FLAG_CARRY);

        ALU8Bit::ADC(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(43, registers.get_single(&SingleRegister::A));

        registers.set_single(&SingleRegister::A, 40);
        registers.set_flags(0);

        ALU8Bit::ADC(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    #[test]
    fn adcn_takes_2_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::ADC_N(0)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value());
    }

    #[test]
    fn adcn_adds_register_plus_carry_to_a() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 40);
        registers.set_flags(MASK_FLAG_CARRY);

        ALU8Bit::ADC_N(2)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(43, registers.get_single(&SingleRegister::A));

        registers.set_single(&SingleRegister::A, 40);
        registers.set_flags(0);

        ALU8Bit::ADC_N(2)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    #[test]
    fn adchl_takes_2_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::ADC_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value());
    }

    #[test]
    fn adchl_adds_register_plus_carry_to_a() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 40);
        memory.set(registers.get_double(&DoubleRegister::HL), 2);
        registers.set_flags(MASK_FLAG_CARRY);

        ALU8Bit::ADC_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(43, registers.get_single(&SingleRegister::A));

        registers.set_single(&SingleRegister::A, 40);
        registers.set_flags(0);

        ALU8Bit::ADC_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    #[test]
    fn sub_takes_1_machine_cycle() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::SUB(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(1, cycles.value());
    }

    #[test]
    fn sub_subtracts_register_from_a() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 45);
        registers.set_single(&SingleRegister::B, 3);

        ALU8Bit::SUB(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(42, registers.get_single(&SingleRegister::A));
    }

    #[test]
    fn sub_sets_the_negative_flag() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        assert!(!registers.is_negative());

        ALU8Bit::SUB(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert!(registers.is_negative());
    }

    #[test]
    fn sub_sets_the_zero_flag_if_result_is_zero() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        assert!(!registers.is_zero());

        ALU8Bit::SUB(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert!(registers.is_zero());
    }

    #[test]
    fn sub_resets_the_zero_flag_if_result_is_non_zero() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 45);
        registers.set_single(&SingleRegister::B, 3);

        assert!(!registers.is_zero());

        ALU8Bit::SUB(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert!(!registers.is_zero());
    }

    #[test]
    fn sub_handles_overflow() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 10);
        registers.set_single(&SingleRegister::B, 15);

        ALU8Bit::SUB(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(251, registers.get_single(&SingleRegister::A));
    }

    #[test]
    fn sub_handles_flags_correctly() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::E, 0x3E);
        memory.set(registers.get_double(&DoubleRegister::HL), 0x40);
        registers.set_single(&SingleRegister::A, 0x3E);

        ALU8Bit::SUB(SingleRegister::E)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x00,
            registers.get_single(&SingleRegister::A),
            "Sub has wrong result"
        );
        assert_eq!(
            0b1100_0000,
            registers.get_flags(),
            "Sub sets incorrect flags"
        );

        registers.set_single(&SingleRegister::A, 0x3E);

        ALU8Bit::SUB_N(0x0F)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x2F,
            registers.get_single(&SingleRegister::A),
            "SubN has wrong result"
        );
        assert_eq!(
            0b0110_0000,
            registers.get_flags(),
            "SubN sets incorrect flags"
        );

        registers.set_single(&SingleRegister::A, 0x3E);

        ALU8Bit::SUB_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0xFE,
            registers.get_single(&SingleRegister::A),
            "SubHL has wrong result"
        );
        assert_eq!(
            0b0101_0000,
            registers.get_flags(),
            "SubN sets incorrect flags"
        );
    }

    #[test]
    fn sbc_takes_the_correct_amount_of_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::SBC(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(1, cycles.value(), "Incorrect machine cycle count for Sbc");

        let cycles = ALU8Bit::SBC_N(42)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value(), "Incorrect machine cycle count for SbcN");

        let cycles = ALU8Bit::SBC_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value(), "Incorrect machine cycle count for SbcHL");
    }

    #[test]
    fn sbc_computes_and_handles_flags_correctly() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::H, 0x2A);
        memory.set(registers.get_double(&DoubleRegister::HL), 0x4F);
        registers.set_single(&SingleRegister::A, 0x3B);
        registers.set_flags(0b0001_0000);

        ALU8Bit::SBC(SingleRegister::H)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x10,
            registers.get_single(&SingleRegister::A),
            "Sbc has wrong result"
        );
        assert_eq!(
            0b0100_0000,
            registers.get_flags(),
            "Sbc sets incorrect flags"
        );

        registers.set_single(&SingleRegister::A, 0x3B);
        registers.set_flags(0b0001_0000);

        ALU8Bit::SBC_N(0x3A)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x00,
            registers.get_single(&SingleRegister::A),
            "SbcN has wrong result"
        );
        assert_eq!(
            0b1100_0000,
            registers.get_flags(),
            "SbcN sets incorrect flags"
        );

        registers.set_single(&SingleRegister::A, 0x3B);

        registers.set_flags(0b0001_0000);

        ALU8Bit::SBC_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0xEB,
            registers.get_single(&SingleRegister::A),
            "SbcHL has wrong result"
        );
        assert_eq!(
            0b0111_0000,
            registers.get_flags(),
            "SbcHL sets incorrect flags: {:08b}",
            registers.get_flags()
        );
    }

    #[test]
    fn and_takes_the_correct_amount_of_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::AND(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(1, cycles.value(), "Incorrect machine cycle count for And");

        let cycles = ALU8Bit::AND_N(42)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value(), "Incorrect machine cycle count for AndN");

        let cycles = ALU8Bit::AND_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value(), "Incorrect machine cycle count for AndHL");
    }

    #[test]
    fn and_does_not_support_the_f_register() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let result =
            ALU8Bit::AND(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(
            SingleRegister::F,
        ));

        assert_eq!(expected, result);
    }

    #[test]
    fn and_computes_and_handles_flags_correctly() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_single(&SingleRegister::A, 0x5A);
        registers.set_single(&SingleRegister::L, 0x3F);

        ALU8Bit::AND(SingleRegister::L)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x1A,
            registers.get_single(&SingleRegister::A),
            "And has wrong result"
        );
        assert_eq!(
            0b0010_0000,
            registers.get_flags(),
            "And sets incorrect flags"
        );

        registers.set_single(&SingleRegister::A, 0x5A);

        ALU8Bit::AND_N(0x38)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x18,
            registers.get_single(&SingleRegister::A),
            "AndN has wrong result"
        );
        assert_eq!(
            0b0010_0000,
            registers.get_flags(),
            "AndN sets incorrect flags"
        );

        registers.set_single(&SingleRegister::A, 0x5A);
        registers.set_double(&DoubleRegister::HL, 0x00);

        ALU8Bit::AND_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x00,
            registers.get_single(&SingleRegister::A),
            "AndHL has wrong result"
        );
        assert_eq!(
            0b1010_0000,
            registers.get_flags(),
            "AndHL sets incorrect flags"
        );
    }

    #[test]
    fn or_takes_the_correct_amount_of_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::OR(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(1, cycles.value(), "Incorrect machine cycle count for Or");

        let cycles = ALU8Bit::OR_N(42)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value(), "Incorrect machine cycle count for OrN");

        let cycles = ALU8Bit::OR_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value(), "Incorrect machine cycle count for OrHL");
    }

    #[test]
    fn or_does_not_support_the_f_register() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let result =
            ALU8Bit::OR(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(
            SingleRegister::F,
        ));

        assert_eq!(expected, result);
    }

    #[test]
    fn or_computes_and_handles_flags_correctly() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        memory.set(registers.get_double(&DoubleRegister::HL), 0x0F);
        registers.set_single(&SingleRegister::A, 0x5A);

        ALU8Bit::OR(SingleRegister::A)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x5A,
            registers.get_single(&SingleRegister::A),
            "Or has wrong result"
        );
        assert_eq!(
            0b0000_0000,
            registers.get_flags(),
            "Or sets incorrect flags"
        );

        registers.set_single(&SingleRegister::A, 0x5A);

        ALU8Bit::OR_N(0x03)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x5B,
            registers.get_single(&SingleRegister::A),
            "OrN has wrong result"
        );
        assert_eq!(
            0b0000_0000,
            registers.get_flags(),
            "OrN sets incorrect flags"
        );

        registers.set_single(&SingleRegister::A, 0x5A);

        ALU8Bit::OR_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x5F,
            registers.get_single(&SingleRegister::A),
            "OrHL has wrong result"
        );
        assert_eq!(
            0b0000_0000,
            registers.get_flags(),
            "OrHL sets incorrect flags"
        );
    }

    #[test]
    fn xor_takes_the_correct_amount_of_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::XOR(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(1, cycles.value(), "Incorrect machine cycle count for Xor");

        let cycles = ALU8Bit::XOR_N(42)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value(), "Incorrect machine cycle count for XorN");

        let cycles = ALU8Bit::XOR_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value(), "Incorrect machine cycle count for XorHL");
    }

    #[test]
    fn xor_does_not_support_the_f_register() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let result =
            ALU8Bit::XOR(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(
            SingleRegister::F,
        ));

        assert_eq!(expected, result);
    }

    #[test]
    fn xor_computes_and_handles_flags_correctly() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        memory.set(registers.get_double(&DoubleRegister::HL), 0x8A);
        registers.set_single(&SingleRegister::A, 0xFF);

        ALU8Bit::XOR(SingleRegister::A)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x00,
            registers.get_single(&SingleRegister::A),
            "Xor has wrong result"
        );
        assert_eq!(
            0b1000_0000,
            registers.get_flags(),
            "Xor sets incorrect flags"
        );

        registers.set_single(&SingleRegister::A, 0xFF);

        ALU8Bit::XOR_N(0x0F)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0xF0,
            registers.get_single(&SingleRegister::A),
            "XorN has wrong result"
        );
        assert_eq!(
            0b0000_0000,
            registers.get_flags(),
            "XorN sets incorrect flags"
        );

        registers.set_single(&SingleRegister::A, 0xFF);

        ALU8Bit::XOR_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x75,
            registers.get_single(&SingleRegister::A),
            "XorHL has wrong result"
        );
        assert_eq!(
            0b0000_0000,
            registers.get_flags(),
            "XorHL sets incorrect flags"
        );
    }

    #[test]
    fn cp_takes_the_correct_amount_of_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::CP(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(1, cycles.value(), "Incorrect machine cycle count for Cp");

        let cycles = ALU8Bit::CP_N(42)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value(), "Incorrect machine cycle count for CpN");

        let cycles = ALU8Bit::CP_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(2, cycles.value(), "Incorrect machine cycle count for CpHL");
    }

    #[test]
    fn cp_does_not_support_the_f_register() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let result =
            ALU8Bit::CP(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(
            SingleRegister::F,
        ));

        assert_eq!(expected, result);
    }

    #[test]
    fn cp_handles_flags_correctly() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        memory.set(registers.get_double(&DoubleRegister::HL), 0x40);
        registers.set_single(&SingleRegister::B, 0x2F);
        registers.set_single(&SingleRegister::A, 0x3C);

        ALU8Bit::CP(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0b0110_0000,
            registers.get_flags(),
            "Cp sets incorrect flags"
        );

        ALU8Bit::CP_N(0x3C)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0b1100_0000,
            registers.get_flags(),
            "CpN sets incorrect flags"
        );

        registers.set_single(&SingleRegister::A, 0x3C);

        ALU8Bit::CP_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0b0101_0000,
            registers.get_flags(),
            "CpHL sets incorrect flags"
        );
    }

    #[test]
    fn inc_takes_the_correct_amount_of_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::INC(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(1, cycles.value(), "Incorrect machine cycle count for Inc");

        let cycles = ALU8Bit::INC_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(3, cycles.value(), "Incorrect machine cycle count for IncHL");
    }

    #[test]
    fn inc_does_not_support_the_f_register() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let result =
            ALU8Bit::INC(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(
            SingleRegister::F,
        ));

        assert_eq!(expected, result);
    }

    #[test]
    fn inc_handles_flags_correctly() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        memory.set(registers.get_double(&DoubleRegister::HL), 0x50);
        registers.set_single(&SingleRegister::A, 0xFF);

        ALU8Bit::INC(SingleRegister::A)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0,
            registers.get_single(&SingleRegister::A),
            "Inc sets wrong result"
        );
        assert_eq!(
            0b1010_0000,
            registers.get_flags(),
            "Inc sets incorrect flags"
        );

        ALU8Bit::INC_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0x51,
            memory.get(registers.get_double(&DoubleRegister::HL)),
            "IncHL sets wrong result"
        );
        assert_eq!(
            0b0000_0000,
            registers.get_flags(),
            "IncHL sets incorrect flags"
        );

        registers.set_flags(MASK_FLAG_CARRY);
        ALU8Bit::INC(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0b0001_0000,
            registers.get_flags(),
            "Inc did not maintain Carry flag"
        );

        registers.set_flags(MASK_FLAG_CARRY);
        ALU8Bit::INC_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0b0001_0000,
            registers.get_flags(),
            "IncHL did not maintain Carry flag"
        );
    }

    #[test]
    fn dec_takes_the_correct_amount_of_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU8Bit::DEC(SingleRegister::B)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(1, cycles.value(), "Incorrect machine cycle count for Dec");

        let cycles = ALU8Bit::DEC_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(3, cycles.value(), "Incorrect machine cycle count for DecHL");
    }

    #[test]
    fn dec_does_not_support_the_f_register() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let result =
            ALU8Bit::DEC(SingleRegister::F).execute(&mut registers, &mut memory, &mut cpu_flags);
        let expected = Err(crate::errors::CpuError::UnsupportedSingleRegister(
            SingleRegister::F,
        ));

        assert_eq!(expected, result);
    }

    #[test]
    fn dec_handles_flags_correctly() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        memory.set(registers.get_double(&DoubleRegister::HL), 0x00);
        registers.set_single(&SingleRegister::A, 0x01);
        registers.set_single(&SingleRegister::C, 0x02);

        ALU8Bit::DEC(SingleRegister::A)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0,
            registers.get_single(&SingleRegister::A),
            "Dec sets wrong result"
        );
        assert_eq!(
            0b1100_0000,
            registers.get_flags(),
            "Dec sets incorrect flags"
        );

        ALU8Bit::DEC_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0xFF,
            memory.get(registers.get_double(&DoubleRegister::HL)),
            "DecHL sets wrong result"
        );
        assert_eq!(
            0b0110_0000,
            registers.get_flags(),
            "DecHL sets incorrect flags"
        );

        registers.set_flags(MASK_FLAG_CARRY);
        ALU8Bit::DEC(SingleRegister::C)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0b0101_0000,
            registers.get_flags(),
            "Dec did not maintain Carry flag"
        );

        registers.set_flags(MASK_FLAG_CARRY);
        ALU8Bit::DEC_HL()
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(
            0b0101_0000,
            registers.get_flags(),
            "DecHL did not maintain Carry flag"
        );
    }
}
