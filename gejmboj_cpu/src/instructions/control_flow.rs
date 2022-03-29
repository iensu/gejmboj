use crate::instruction_group;
use crate::{instructions::Condition, registers::DoubleRegister};

instruction_group! {
    /// Program control flow instructions
    ControlFlow (registers, memory, cpu_flags) {

        /// Unconditional jump to location specified by 16-bit operand.
        JP(operand: u16) [3] => {
            registers.PC = *operand;
            Ok(4)
        }

        /// Conditional jump to location specified by 16-bit operand.
        JPC(operand: u16, condition: Condition) [3] => {
            if condition.is_fulfilled(registers) {
                registers.PC = *operand;
                Ok(4)
            } else {
                Ok(3)
            }
        }

        /// Unconditional jump to location specified by register HL
        JP_HL() [1] => {
            registers.PC = registers.get_double(&DoubleRegister::HL);
            Ok(1)
        }

        /// Unconditional jump to location at current + offset (-129 to 126), where the offset is
        /// relative to the next instruction.
        ///
        /// **Positive example**
        ///
        /// | Location | Instruction   |
        /// |---------:|:--------------|
        /// |    0x480 | JR            |
        /// |    0x481 | 0x03          |
        /// |    0x482 | -             |
        /// |    0x483 | -             |
        /// |    0x484 | -             |
        /// |    0x485 | PC after jump |
        ///
        /// **Negative example**
        ///
        /// | Location | Instruction   |
        /// |---------:|:--------------|
        /// |    0x47C | PC after jump |
        /// |    0x47D | -             |
        /// |    0x47E | -             |
        /// |    0x47F | -             |
        /// |    0x480 | JR            |
        /// |    0x481 | 0xFA          |
        JR(operand: u8) [2] => {
            let offset = *operand as i8;

            if offset >= 0 {
                registers.PC += offset as u16;
            } else {
                registers.PC -= offset.abs() as u16;
            }

            Ok(3)
        }

        /// Conditional jump to relative address specified by offset operand.
        ///
        /// **Positive example**
        ///
        /// | Location | Instruction   |
        /// |---------:|:--------------|
        /// |    0x480 | JR            |
        /// |    0x481 | 0x03          |
        /// |    0x482 | -             |
        /// |    0x483 | -             |
        /// |    0x484 | -             |
        /// |    0x485 | PC after jump |
        ///
        /// **Negative example**
        ///
        /// | Location | Instruction   |
        /// |---------:|:--------------|
        /// |    0x47C | PC after jump |
        /// |    0x47D | -             |
        /// |    0x47E | -             |
        /// |    0x47F | -             |
        /// |    0x480 | JR            |
        /// |    0x481 | 0xFA          |
        JRC(operand: u8, condition: Condition) [2] => {
            if condition.is_fulfilled(registers) {
                let offset = *operand as i8;

                if offset >= 0 {
                    registers.PC += offset as u16;
                } else {
                    registers.PC -= offset.abs() as u16;
                }

                Ok(3)
            } else {
                Ok(2)
            }
        }

        /// Unconditional call of the function at operand address.
        CALL(operand: u16) [3] => {
            let sp = registers.decrement_sp();
            let next_pc = registers.PC + 3;
            memory.set_u16(sp.into(), next_pc);
            registers.PC = *operand;

            Ok(6)
        }

        /// Conditional function call.
        CALLC(operand: u16, condition: Condition) [3] => {
            if condition.is_fulfilled(registers) {
                let sp = registers.decrement_sp();

                memory.set_u16(sp.into(), registers.PC);
                registers.PC = *operand;

                Ok(6)
            } else {
                Ok(3)
            }
        }

        /// Unconditional return from function.
        RET() [1] => {
            registers.PC = memory.get_u16(registers.SP.into());
            registers.increment_sp();
            Ok(4)
        }

        /// Conditionally return from function.
        RETC(condition: Condition) [1] => {
            if condition.is_fulfilled(registers) {
                registers.PC = memory.get_u16(registers.SP.into());
                registers.increment_sp();
                Ok(5)
            } else {
                Ok(2)
            }
        }

        /// Unconditional return from a function which enables interrupts
        RETI() [1] => {
            registers.PC = memory.get_u16(registers.SP.into());
            registers.increment_sp();
            cpu_flags.IME = true;
            Ok(4)
        }

        /// Unconditional function call to the RESET address defined by bits 3-5
        ///
        /// Possible RESET addresses are:
        ///
        /// * `0x00`
        /// * `0x08`
        /// * `0x10`
        /// * `0x18`
        /// * `0x20`
        /// * `0x28`
        /// * `0x30`
        /// * `0x38`
        RST(opcode: u8) [1] => {
            registers.PC = get_reset_address(*opcode);
            Ok(4)
        }
    }
}

fn get_reset_address(opcode: u8) -> u16 {
    (opcode & 0b00111000) as u16
}

#[cfg(test)]
crate::instruction_tests! {
    jp_jumps_to_address(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JP(0xBADA);

        assert_eq!(0, registers.PC);
        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0xBADA, registers.PC);
    }

    jpc_jumps_to_location_if_condition_if_fulfilled(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JPC(0xBADA, Condition::Carry);

        let mut cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0, registers.PC);
        assert_eq!(3, cycles);

        registers.set_flags(MASK_FLAG_CARRY);
        cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0xBADA, registers.PC);
        assert_eq!(4, cycles);
    }

    jp_hl_jumps_to_location_in_register_hl(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JP_HL();

        registers.set_double(&DoubleRegister::HL, 0xBADA);
        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0xBADA, registers.PC);
    }

    jr_jumps_to_current_plus_offset_steps(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JR(0x40);
        registers.PC = 0x0200;

        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x0242, registers.PC + instruction.length());
    }

    jr_continues_if_passed_zero(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JR(0x00);
        registers.PC = 0x0200;
        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x0202, registers.PC + instruction.length());
    }

    jr_can_wrap_around(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JR(0xFE);
        registers.PC = 0x0200;
        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x0200, registers.PC + instruction.length());
    }

    jr_zilog_manual_example_one(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JR(0x03);
        registers.PC = 0x480;
        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x485, registers.PC + instruction.length());
    }

    jr_zilog_manual_example_two(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JR(0xFA);
        registers.PC = 0x480;
        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x47C, registers.PC + instruction.length());
    }

    jrc_jumps_to_offset_if_condition_is_fulfilled(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JRC(0x40, Condition::Zero);
        registers.PC = 0x0200;

        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x0200, registers.PC);
        assert_eq!(2, cycles);

        registers.set_flags(MASK_FLAG_ZERO);

        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x0242, registers.PC + instruction.length());
        assert_eq!(3, cycles);
    }

    jrc_continues_if_passed_zero(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JRC(0x00, Condition::Zero);
        registers.PC = 0x0200;
        registers.set_zero(true);

        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x0202, registers.PC + instruction.length());
    }

    jrc_can_wrap_around(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JRC(0xFE, Condition::Zero);
        registers.PC = 0x0200;
        registers.set_zero(true);

        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x0200, registers.PC + instruction.length());
    }

    jrc_zilog_manual_example_one(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JRC(0x03, Condition::Zero);
        registers.PC = 0x480;
        registers.set_zero(true);
        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x485, registers.PC + instruction.length());
    }

    jrc_zilog_manual_example_two(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::JRC(0xFA, Condition::Zero);
        registers.PC = 0x480;
        registers.set_zero(true);

        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0x47C, registers.PC + instruction.length());
    }

    call_calls_function_at_operand(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::CALL(0xABCD);
        registers.PC = 0xAAAA;
        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0xABCD, registers.PC);
        assert_eq!(0xFFFC, registers.SP);
        assert_eq!(0xAAAD, memory.get_u16(registers.SP.into()));
    }

    call_sets_sp_correctly(registers, memory, cpu_flags) => {
        registers.PC = 0x8000;
        registers.SP = 0xFFFE;
        memory.set_u16(0x8001, 0x1234);

        ControlFlow::CALL(0x1234).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0x1234, registers.PC);
        assert_eq!(0xFFFC, registers.SP);
        assert_eq!(0x80, memory.get(0xFFFD));
        assert_eq!(0x03, memory.get(0xFFFC));
    }

    callc_does_not_call_function_if_condition_is_unfulfilled(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::CALLC(0xABCD, Condition::Carry);
        registers.PC = 0xAAAA;

        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(registers.PC, 0xAAAA);
        assert_eq!(registers.SP, 0xFFFE);
        assert_eq!(3, cycles);
    }

    callc_calls_function_if_condition_is_unfulfilled(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::CALLC(0xABCD, Condition::Carry);
        registers.PC = 0xAAAA;
        registers.set_flags(MASK_FLAG_CARRY);

        let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(registers.PC, 0xABCD);
        assert_eq!(registers.SP, 0xFFFC);
        assert_eq!(6, cycles);
    }

    ret_returns_from_function_call(registers, memory, cpu_flags) => {
        let function_call = ControlFlow::CALL(0xABCD);
        let return_call = ControlFlow::RET();

        registers.PC = 0xAAAA;
        function_call.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        return_call.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0xAAAD, registers.PC);
        assert_eq!(0xFFFE, registers.SP);
    }

    retc_returns_from_call_if_condition_is_fulfilled(registers, memory, cpu_flags) => {
        registers.PC = 0xAAAA;

        let call = ControlFlow::CALL(0xABCD);
        call.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        let ret = ControlFlow::RETC(Condition::Carry);
        let cycles = ret.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0xABCD, registers.PC);
        assert_eq!(0xFFFC, registers.SP);
        assert_eq!(2, cycles);

        registers.set_flags(MASK_FLAG_CARRY);
        let cycles = ret.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0xAAAD, registers.PC);
        assert_eq!(0xFFFE, registers.SP);
        assert_eq!(5, cycles);
    }

    reti_returns_from_a_function_and_enables_interrupts(registers, memory, cpu_flags) => {
        let call = ControlFlow::CALL(0xABCD);
        let reti = ControlFlow::RETI();

        registers.PC = 0xAAAA;
        call.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        reti.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0xAAAD, registers.PC);
        assert_eq!(0xFFFE, registers.SP);
        assert_eq!(true, cpu_flags.IME);
    }

    rst_calls_function_at_reset_address(registers, memory, cpu_flags) => {
        let instruction = ControlFlow::RST(0b1101_0111);

        instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(0x10, registers.PC);
    }
}
