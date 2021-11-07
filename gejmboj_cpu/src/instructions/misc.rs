use crate::{instruction_group, registers::SingleRegister};

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
        Noop() [1] => {
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
            let value = registers.get_single(&SingleRegister::F);
            let value = value & 0b1001_0000; // Clear N and H flags
            let value = value ^ 0b0001_0000; // Flip C
            registers.set_single(&SingleRegister::F, value);
            Ok(1)
        }

        /// Sets the carry flag (C) and clears the negative (N) and half-carry (H) flags
        SCF() [1] => {
            let value = registers.get_single(&SingleRegister::F);
            let value = value & 0b1001_0000; // Clear N and H flags
            let value = value | 0b0001_0000; // Set C
            registers.set_single(&SingleRegister::F, value);
            Ok(1)
        }

        /// Flips the zero (Z) and carry (C) flags and clears the half-carry (H) flag
        DAA() [1] => {
            let value = registers.get_single(&SingleRegister::F);
            let value = value & 0b1101_0000; // Clear H
            let value = value ^ 0b1001_0000; // Flip Z and C
            registers.set_single(&SingleRegister::F, value);
            Ok(1)
        }

        /// Flips all bits in the A register and sets the negative (N) and half-carry (H) flags
        CPL() [1] => {
            let flags = registers.get_single(&SingleRegister::F);
            let flags = flags | 0b0110_0000; // Set N and H

            let value = registers.get_single(&SingleRegister::A);
            let value = value ^ 0b1111_1111; // Flip all bits

            registers.set_single(&SingleRegister::F, flags);
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
        registers.set_single(&SingleRegister::F, 0b1111_0000);

        Misc::CCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1000_0000, registers.get_single(&SingleRegister::F));
    }

    ccf_flips_the_carry_flag(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::F, 0b0001_0000);

        Misc::CCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0000_0000, registers.get_single(&SingleRegister::F));

        Misc::CCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0001_0000, registers.get_single(&SingleRegister::F));
    }

    scf_takes_one_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = Misc::SCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    scf_clears_the_negative_and_half_carry_flags(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::F, 0b1111_0000);

        Misc::SCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1001_0000, registers.get_single(&SingleRegister::F));

        Misc::SCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1001_0000, registers.get_single(&SingleRegister::F));
    }

    scf_sets_the_carry_flag(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::F, 0b0000_0000);

        Misc::SCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0001_0000, registers.get_single(&SingleRegister::F));

        Misc::SCF().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0001_0000, registers.get_single(&SingleRegister::F));
    }


    daa_takes_one_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = Misc::DAA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    daa_clears_the_half_carry_flag(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::F, 0b0010_0000);

        Misc::DAA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        let h_flag = registers.get_single(&SingleRegister::F) & 0b0010_0000;

        assert_eq!(0, h_flag);

        Misc::DAA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        let h_flag = registers.get_single(&SingleRegister::F) & 0b0010_0000;

        assert_eq!(0, h_flag);
    }

    daa_flips_the_zero_and_carry_flags(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::F, 0b0000_0000);

        Misc::DAA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1001_0000, registers.get_single(&SingleRegister::F));

        Misc::DAA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0000_0000, registers.get_single(&SingleRegister::F));
    }

    cpl_takes_one_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = Misc::CPL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(1, cycles);
    }

    cpl_sets_the_negative_and_half_carry_flags(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::F, 0b0000_0000);

        Misc::CPL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0110_0000, registers.get_single(&SingleRegister::F));

        Misc::CPL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0110_0000, registers.get_single(&SingleRegister::F));
    }

    cpl_flips_all_bits_in_the_a_register(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b0000_0000);

        Misc::CPL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b1111_1111, registers.get_single(&SingleRegister::A));

        Misc::CPL().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0b0000_0000, registers.get_single(&SingleRegister::A));
    }
}
