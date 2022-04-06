use crate::{
    instruction_group,
    instructions::utils,
    registers::{SingleRegister, MASK_FLAG_CARRY, MASK_FLAG_ZERO},
};

instruction_group! {
    /// Miscelleneous instructions
    ///
    /// Some instructions operate on the flag register F:
    ///
    /// ```asciidoc
    /// ,---.---.---.---.---.---.---.---.
    /// | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 | Bit
    /// |---|---|---|---|---|---|---|---|
    /// | Z | N | H | C | 0 | 0 | 0 | 0 | Flag
    /// `---´---´---´---´---´---´---´---´
    /// ```
    ///
    /// Where bit 0-3 are grounded to `0` and can't be overwritten and `C` is for carry, `H` for half-carry, `N`
    /// for negative and `Z` for zero.
    Misc (registers, _memory, cpu_flags) {

        /// No operation
        NOP() [1] => {
            Ok(1)
        }

        /// Disable interrupt handling
        DI() [1] => {
            cpu_flags.IME = false;
            Ok(1)
        }

        /// Schedules interrupt handling to be enabled after the next machine cycle
        EI() [1] => {
            cpu_flags.IME_scheduled = true;
            Ok(1)
        }

        /// Flips the carry flag (C) and clears the negative (N) and half-carry (H) flags
        CCF() [1] => {
            let value = registers.get_flags();
            let value = value & 0b1001_0000; // Clear N and H flags
            let value = value ^ 0b0001_0000; // Flip C
            registers.set_flags(value);
            Ok(1)
        }

        /// Sets the carry flag (C) and clears the negative (N) and half-carry (H) flags
        SCF() [1] => {
            let value = registers.get_flags();
            let value = value & 0b1001_0000; // Clear N and H flags
            let value = value | 0b0001_0000; // Set C
            registers.set_flags(value);
            Ok(1)
        }

        /// Decimal Adjust Accumulator (DAA)
        ///
        /// This instruction affects the A register and should be called after Binary Coded Decimal (BCD) addition or
        /// subtraction instructions. This instruction converts the value stored in A into BCD representation. The
        /// instruction sets the Carry and Zero flags if appropriate.
        ///
        /// In BCD representation each nibble (4 bits) represents a digit:
        ///
        /// | Decimal | Binary        | BCD                  |
        /// |:--------|:--------------|:---------------------|
        /// | `1`     | `0b0000_0001` | `0b0000_0001` (0, 1) |
        /// | `10`    | `0b0000_1010` | `0b0001_0000` (1, 0) |
        /// | `28`    | `0b0001_1100` | `0b0010_1000` (2, 8) |
        ///
        /// When you operate on BCD numbers you have to convert the result into BCD as well according to the following rules:
        /// * Add 6 to each digit above 9 if addition
        /// * Subtract 6 from each digit above 9 if subtraction
        ///
        /// All subtraction is achieved by adding the Two's Complement (inverse of number + 1).
        ///
        /// **BCD example: 29 + 13 = 42**
        ///
        /// ```asciidoc
        ///   0b0010_1001
        /// + 0b0001_0011
        ///   -----------
        ///   0b0011_1100 (BCD: 3(12), Binary: 60)
        ///
        ///   0b0011_1100
        /// + 0b0000_0110
        ///   -----------
        ///   0b0100_0010 (BCD: 42, Binary: 66)
        /// ```
        /// **BCD example: 23 - 19 = 4**
        ///
        /// ```asciidoc
        ///     11     11
        ///   0b0010_0011
        /// + 0b1110_0111 (Two's Complement of 19)
        ///   -----------
        ///   0b0000_1010 (BCD: 8, Binary: 8)
        ///
        ///     1111   1
        ///   0b0000_1010
        /// + 0b1111_1010
        ///   -----------
        ///   0b0000_0100 (BCD: 4, Binary: 4)
        /// ```
        ///
        /// BCD representation is often used instead of converting back-and-forth between binary and decimal when doing addition
        /// and subtraction, especially when no micro-processor is involved since the necessary circuit becomes a lot simpler. A
        /// common use-case is Seven Segment Displays where each display represents a digit.
        DAA() [1] => {
            let a = registers.get_single(&SingleRegister::A);
            let mut bcd_correction = 0;
            let mut flags = 0;

            if registers.is_half_carry() || (a & 0xF) > 9 {
                bcd_correction = bcd_correction | 0x6;
            }
            if registers.is_carry() || a > 0x99 {
                bcd_correction = bcd_correction | 0x60;
                flags = flags | MASK_FLAG_CARRY;
            }

            if registers.is_negative() {
                bcd_correction = utils::twos_complement(bcd_correction);
            };

            let bcd = a.wrapping_add(bcd_correction);
            registers.set_single(&SingleRegister::A, bcd);

            if bcd == 0 {
                flags = flags | MASK_FLAG_ZERO;
            }

            registers.set_flags(flags);
            Ok(1)
        }

        /// Flips all bits in the A register and sets the negative (N) and half-carry (H) flags
        CPL() [1] => {
            let flags = registers.get_flags();
            let flags = flags | 0b0110_0000; // Set N and H

            let value = registers.get_single(&SingleRegister::A);
            let value = value ^ 0b1111_1111; // Flip all bits

            registers.set_flags(flags);
            registers.set_single(&SingleRegister::A, value);
            Ok(1)
        }
    }
}

#[cfg(test)]
crate::instruction_tests! {
    di_disables_interrupt_handling(registers, memory, cpu_flags) => {
        cpu_flags.IME = true;
        let cycles = Misc::DI().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
        assert_eq!(false, cpu_flags.IME);
    }

    ei_schedules_interrupt_handling(registers, memory, cpu_flags) => {
        assert_eq!(false, cpu_flags.IME_scheduled);

        let cycles = Misc::EI().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
        assert_eq!(true, cpu_flags.IME_scheduled);
    }

    ccf_takes_one_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = Misc::CCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    ccf_clears_the_negative_and_half_carry_flags(registers, memory, cpu_flags) => {
        registers.set_flags(0b1111_0000);

        Misc::CCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1000_0000, registers.get_flags());
    }

    ccf_flips_the_carry_flag(registers, memory, cpu_flags) => {
        registers.set_flags(MASK_FLAG_CARRY);

        Misc::CCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0000_0000, registers.get_flags());

        Misc::CCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0001_0000, registers.get_flags());
    }

    scf_takes_one_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = Misc::SCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    scf_clears_the_negative_and_half_carry_flags(registers, memory, cpu_flags) => {
        registers.set_flags(0b1111_0000);

        Misc::SCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1001_0000, registers.get_flags());

        Misc::SCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1001_0000, registers.get_flags());
    }

    scf_sets_the_carry_flag(registers, memory, cpu_flags) => {
        registers.set_flags(0b0000_0000);

        Misc::SCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0001_0000, registers.get_flags());

        Misc::SCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0001_0000, registers.get_flags());
    }


    daa_takes_one_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = Misc::DAA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    daa_clears_the_half_carry_flag(registers, memory, cpu_flags) => {
        registers.set_flags(MASK_FLAG_HALF_CARRY);

        Misc::DAA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        let h_flag = registers.get_flags() & MASK_FLAG_HALF_CARRY;

        assert_eq!(0, h_flag);

        Misc::DAA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        let h_flag = registers.get_flags() & MASK_FLAG_HALF_CARRY;

        assert_eq!(0, h_flag);
    }

    daa_flips_sets_the_zero_flag(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0);

        Misc::DAA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert!(registers.is_zero());

        registers.set_single(&SingleRegister::A, 6);

        Misc::DAA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0000_0000, registers.get_flags());
    }

    daa_example_test(registers, memory, cpu_flags) => {
        use crate::instructions::alu_8bit::{ALU8Bit};

        registers.set_single(&SingleRegister::A, 0x45);
        registers.set_single(&SingleRegister::B, 0x38);

        ALU8Bit::ADD(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x7D, registers.get_single(&SingleRegister::A));
        assert!(!registers.is_negative());

        Misc::DAA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x7D + 0x06, registers.get_single(&SingleRegister::A));
        assert!(!registers.is_carry());

        ALU8Bit::SUB(SingleRegister::B).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x83 - 0x38, registers.get_single(&SingleRegister::A));
        assert!(registers.is_negative());

        Misc::DAA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x4Bu8.wrapping_add(0xFA), registers.get_single(&SingleRegister::A));
    }

    cpl_takes_one_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = Misc::CPL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    cpl_sets_the_negative_and_half_carry_flags(registers, memory, cpu_flags) => {
        registers.set_flags(0b0000_0000);

        Misc::CPL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0110_0000, registers.get_flags());

        Misc::CPL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0110_0000, registers.get_flags());
    }

    cpl_flips_all_bits_in_the_a_register(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b0000_0000);

        Misc::CPL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1111_1111, registers.get_single(&SingleRegister::A));

        Misc::CPL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0000_0000, registers.get_single(&SingleRegister::A));
    }
}
