use crate::{
    cycles::MachineCycles,
    instruction_group,
    registers::{DoubleRegister, MASK_FLAG_CARRY, MASK_FLAG_HALF_CARRY, MASK_FLAG_ZERO},
};

instruction_group! {
    /// 16-bit ALU instructions
    ALU16Bit (registers, _memory, _cpu_flags) {
        /// Add contents of `DoubleRegister` to `HL`
        ///
        /// **Flags**
        ///
        /// | Flag | Effect                               |
        /// |------|--------------------------------------|
        /// | `Z`  | No change                            |
        /// | `N`  | `0`                                  |
        /// | `H`  | Set if carry from bit 11, else reset |
        /// | `C`  | Set if carry from bit 15, else reset |
        ADD_HL(r: DoubleRegister) [1] => {
            let hl = registers.get_double(&DoubleRegister::HL);
            let operand = registers.get_double(r);
            let (result, carry) = hl.overflowing_add(operand);

            let mut flags = registers.get_flags() & MASK_FLAG_ZERO; // Keep the Z flag unchanged
            if carry {
                flags |= MASK_FLAG_CARRY;
            }
            if ((hl & 0xFFF) + (operand & 0xFFF)) >= 0x1000 {
                flags |= MASK_FLAG_HALF_CARRY;
            }
            registers.set_double(&DoubleRegister::HL, result);
            registers.set_flags(flags);
            Ok(MachineCycles::new(2))
        }

        /// Add contents of `u8` operand to `SP`
        ///
        /// | Flag | Effect                               |
        /// |------|--------------------------------------|
        /// | `Z`  | `0`                                  |
        /// | `N`  | `0`                                  |
        /// | `H`  | Set if carry from bit 11, else reset |
        /// | `C`  | Set if carry from bit 15, else reset |
        ADD_SP(operand: u8) [2] => {
            let extended = if operand.cast_signed() < 0 {
                0xFF00 | u16::from(*operand)
            }  else {
                u16::from(*operand)
            };
            let sp = registers.SP.wrapping_add(extended);

            let mut flags = 0;
            if sp & 0xFF < registers.SP & 0xFF {
                flags |= MASK_FLAG_CARRY;
            }
            if sp & 0xF < registers.SP & 0xF {
                flags |= MASK_FLAG_HALF_CARRY;
            }

            registers.SP = sp;
            registers.set_flags(flags);
            Ok(MachineCycles::new(4))
        }

        /// Increment contents of `DoubleRegister` by 1.
        ///
        /// Flags are unaffected.
        INC(r: DoubleRegister) [1] => {
            let result = registers.get_double(r).wrapping_add(1);
            registers.set_double(r, result);
            Ok(MachineCycles::new(2))
        }

        /// Decrement contents of `DoubleRegister` by 1.
        ///
        /// Flags are unaffected.
        DEC(r: DoubleRegister) [1] => {
            let result = registers.get_double(r).wrapping_sub(1);
            registers.set_double(r, result);
            Ok(MachineCycles::new(2))
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
    fn addhl_takes_2_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU16Bit::ADD_HL(DoubleRegister::BC)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(2, cycles.value());
    }

    #[test]
    fn addhl_adds_register_to_hl() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_double(&DoubleRegister::BC, 0xAABB);
        registers.set_double(&DoubleRegister::HL, 0x1122);
        ALU16Bit::ADD_HL(DoubleRegister::BC)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(0xBBDD, registers.get_double(&DoubleRegister::HL));
    }

    #[test]
    fn addhl_sets_flags_correctly() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        for (hl, bc, flags, expected_flags) in [
            (0x0001, 0x0002, 0b0000_0000, 0b0000_0000),
            (0x0001, 0x0002, 0b0100_0000, 0b0000_0000),
            (0x0001, 0x0002, 0b1000_0000, 0b1000_0000),
            (0xFF00, 0x1100, 0b0000_0000, 0b0011_0000),
            (0x0FFF, 0x0111, 0b0000_0000, 0b0010_0000),
            (0xFFFF, 0x1111, 0b0000_0000, 0b0011_0000),
            (0xFFFF, 0x1111, 0b1000_0000, 0b1011_0000),
        ] {
            registers.set_double(&DoubleRegister::HL, hl);
            registers.set_double(&DoubleRegister::BC, bc);
            registers.set_flags(flags);

            ALU16Bit::ADD_HL(DoubleRegister::BC)
                .execute(&mut registers, &mut memory, &mut cpu_flags)
                .unwrap();

            assert_eq!(
                expected_flags,
                registers.get_flags(),
                "Expected {expected_flags:08b} from {hl:04x} + {bc:04x} (flags: {:08b})",
                registers.get_flags()
            );
        }
    }

    #[test]
    fn addsp_takes_4_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU16Bit::ADD_SP(0)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(4, cycles.value());
    }

    #[test]
    fn addsp_adds_operand_to_sp() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_double(&DoubleRegister::SP, 0x1122);
        ALU16Bit::ADD_SP(0xAB)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(0x10CD, registers.get_double(&DoubleRegister::SP));
    }

    #[test]
    fn addsp_sets_flags_correctly() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        for (sp, operand, flags, expected_flags) in [
            (0x0001, 0x02, 0b0000_0000, 0b0000_0000),
            (0x0003, 0x04, 0b0100_0000, 0b0000_0000),
            (0x0005, 0x06, 0b1000_0000, 0b0000_0000),
            (0x0F11, 0xFF, 0b0000_0000, 0b0011_0000),
            (0xFFFF, 0xFF, 0b0000_0000, 0b0011_0000),
            (0xFFFF, 0xFF, 0b1000_0000, 0b0011_0000),
        ] {
            registers.set_double(&DoubleRegister::SP, sp);
            registers.set_flags(flags);

            ALU16Bit::ADD_SP(operand)
                .execute(&mut registers, &mut memory, &mut cpu_flags)
                .unwrap();

            assert_eq!(
                expected_flags,
                registers.get_flags(),
                "Expected {expected_flags:08b} from {sp:04x} + {operand:04x} (flags: {:08b})",
                registers.get_flags()
            );
        }
    }

    #[test]
    fn inc_takes_2_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU16Bit::INC(DoubleRegister::BC)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(2, cycles.value());
    }

    #[test]
    fn inc_increments_register() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_double(&DoubleRegister::BC, 0xABCD);
        ALU16Bit::INC(DoubleRegister::BC)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(0xABCE, registers.get_double(&DoubleRegister::BC));
    }

    #[test]
    fn inc_flags_are_unaffected() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_double(&DoubleRegister::BC, 0xABCD);
        registers.set_flags(0b1111_0000);
        ALU16Bit::INC(DoubleRegister::BC)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(0b1111_0000, registers.get_flags());
    }

    #[test]
    fn dec_takes_2_machine_cycles() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        let cycles = ALU16Bit::DEC(DoubleRegister::BC)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();
        assert_eq!(2, cycles.value());
    }

    #[test]
    fn dec_decrements_register() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_double(&DoubleRegister::BC, 0xABCD);
        ALU16Bit::DEC(DoubleRegister::BC)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(0xABCC, registers.get_double(&DoubleRegister::BC));
    }

    #[test]
    fn dec_flags_are_unaffected() {
        let (mut registers, mut memory, mut cpu_flags) = setup();

        registers.set_double(&DoubleRegister::BC, 0xABCD);
        registers.set_flags(0b1111_0000);
        ALU16Bit::DEC(DoubleRegister::BC)
            .execute(&mut registers, &mut memory, &mut cpu_flags)
            .unwrap();

        assert_eq!(0b1111_0000, registers.get_flags());
    }
}
