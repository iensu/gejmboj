use crate::{
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
            let operand = registers.get_double(&r);
            let (result, carry) = hl.overflowing_add(operand);

            let mut flags = registers.get_flags() & MASK_FLAG_ZERO; // Keep the Z flag unchanged
            if carry {
                flags |= MASK_FLAG_CARRY;
            }
            if ((hl & 0xFFF) + (operand & 0xFFF)) > 0x1000 {
                flags |= MASK_FLAG_HALF_CARRY;
            }
            registers.set_double(&DoubleRegister::HL, result);
            registers.set_flags(flags);
            Ok(2)
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
            let sp = registers.get_double(&DoubleRegister::SP);
            let operand: u16 = *operand as u16;
            let (result, carry) = sp.overflowing_add(operand);

            let mut flags = 0b0000_0000;
            if carry {
                flags |= MASK_FLAG_CARRY;
            }
            if ((sp & 0xFFF) + (operand & 0xFFF)) > 0x1000 {
                flags |= MASK_FLAG_HALF_CARRY;
            }

            registers.set_double(&DoubleRegister::SP, result);
            registers.set_flags(flags);
            Ok(4)
        }

        /// Increment contents of `DoubleRegister` by 1.
        ///
        /// Flags are unaffected.
        INC(r: DoubleRegister) [1] => {
            let result = registers.get_double(&r).wrapping_add(1);
            registers.set_double(&r, result);
            Ok(2)
        }

        /// Decrement contents of `DoubleRegister` by 1.
        ///
        /// Flags are unaffected.
        DEC(r: DoubleRegister) [1] => {
            let result = registers.get_double(&r).wrapping_sub(1);
            registers.set_double(&r, result);
            Ok(2)
        }
    }
}

#[cfg(test)]
crate::instruction_tests! {
    addhl_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU16Bit::ADD_HL(DoubleRegister::BC).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(2, cycles);
    }

    addhl_adds_register_to_hl(registers, memory, cpu_flags) => {
        registers.set_double(&DoubleRegister::BC, 0xAABB);
        registers.set_double(&DoubleRegister::HL, 0x1122);
        ALU16Bit::ADD_HL(DoubleRegister::BC).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0xBBDD, registers.get_double(&DoubleRegister::HL));
    }

    addhl_sets_flags_correctly(registers, memory, cpu_flags) => {
        for (hl, bc, flags, expected_flags) in vec![
            (0x0001, 0x0002, 0b0000_0000, 0b0000_0000),
            (0x0001, 0x0002, 0b0100_0000, 0b0000_0000),
            (0x0001, 0x0002, 0b1000_0000, 0b1000_0000),
            (0xFF00, 0x1100, 0b0000_0000, 0b0001_0000),
            (0x0FFF, 0x0111, 0b0000_0000, 0b0010_0000),
            (0xFFFF, 0x1111, 0b0000_0000, 0b0011_0000),
            (0xFFFF, 0x1111, 0b1000_0000, 0b1011_0000),
        ] {
            registers.set_double(&DoubleRegister::HL, hl);
            registers.set_double(&DoubleRegister::BC, bc);
            registers.set_flags(flags);

            ALU16Bit::ADD_HL(DoubleRegister::BC).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            assert_eq!(expected_flags, registers.get_flags(), "Expected {:08b} from {:04x} + {:04x} (flags: {:08b})", expected_flags, hl, bc, flags);
        }
    }

    addsp_takes_4_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU16Bit::ADD_SP(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(4, cycles);
    }

    addsp_adds_operand_to_sp(registers, memory, cpu_flags) => {
        registers.set_double(&DoubleRegister::SP, 0x1122);
        ALU16Bit::ADD_SP(0xAB).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0x11CD, registers.get_double(&DoubleRegister::SP));
    }

    addsp_sets_flags_correctly(registers, memory, cpu_flags) => {
        for (sp, operand, flags, expected_flags) in vec![
            (0x0001, 0x02, 0b0000_0000, 0b0000_0000),
            (0x0003, 0x04, 0b0100_0000, 0b0000_0000),
            (0x0005, 0x06, 0b1000_0000, 0b0000_0000),
            (0x0F11, 0xFF, 0b0000_0000, 0b0010_0000),
            (0xFFFF, 0xFF, 0b0000_0000, 0b0011_0000),
            (0xFFFF, 0xFF, 0b1000_0000, 0b0011_0000),
        ] {
            registers.set_double(&DoubleRegister::SP, sp);
            registers.set_flags(flags);

            ALU16Bit::ADD_SP(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            assert_eq!(expected_flags, registers.get_flags(), "Expected {:08b} from {:04x} + {:04x} (flags: {:08b})", expected_flags, sp, operand, flags);
        }
    }

    inc_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU16Bit::INC(DoubleRegister::BC).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(2, cycles);
    }

    inc_increments_register(registers, memory, cpu_flags) => {
        registers.set_double(&DoubleRegister::BC, 0xABCD);
        ALU16Bit::INC(DoubleRegister::BC).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0xABCE, registers.get_double(&DoubleRegister::BC));
    }

    inc_flags_are_unaffected(registers, memory, cpu_flags) => {
        registers.set_double(&DoubleRegister::BC, 0xABCD);
        registers.set_flags(0b1111_0000);
        ALU16Bit::INC(DoubleRegister::BC).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1111_0000, registers.get_flags());
    }

    dec_takes_2_machine_cycles(registers, memory, cpu_flags) => {
        let cycles = ALU16Bit::DEC(DoubleRegister::BC).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(2, cycles);
    }

    dec_decrements_register(registers, memory, cpu_flags) => {
        registers.set_double(&DoubleRegister::BC, 0xABCD);
        ALU16Bit::DEC(DoubleRegister::BC).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0xABCC, registers.get_double(&DoubleRegister::BC));
    }

    dec_flags_are_unaffected(registers, memory, cpu_flags) => {
        registers.set_double(&DoubleRegister::BC, 0xABCD);
        registers.set_flags(0b1111_0000);
        ALU16Bit::DEC(DoubleRegister::BC).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1111_0000, registers.get_flags());
    }
}
