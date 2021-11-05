use crate::instruction_group;

instruction_group! {
    /// Miscelleneous instructions
    Misc (_registers, _memory, cpu_flags) {

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
}
