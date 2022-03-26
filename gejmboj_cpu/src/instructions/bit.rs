use crate::{errors::CpuError, instruction_group};

use super::utils::{self, get_register_value};

/// Decodes the `operand` into a `Bit` instruction.
///
/// | Operand      | Instruction |
/// |--------------|-------------|
/// | `01_bbb_rrr` | `Bit`       |
/// | `11_bbb_rrr` | `Set`       |
/// | `10_bbb_rrr` | `Res`        |
pub fn decode(operand: u8) -> Result<Bit, CpuError> {
    match utils::into_bits(operand) {
        (0, 1, _, _, _, _, _, _) => Ok(Bit::Bit(operand)),
        (1, 1, _, _, _, _, _, _) => Ok(Bit::Set(operand)),
        (1, 0, _, _, _, _, _, _) => Ok(Bit::Res(operand)),
        _ => Err(CpuError::UnknownInstruction(operand)),
    }
}

instruction_group! {
    /// Bit operations
    ///
    /// These operations act on specific bits of a register or location in memory: `xx-bbb-rrr`.
    /// The registers are resolved as usual, and the specific bit is resolved as the following table:
    ///
    /// | Bit | `bbb`      |
    /// |:----|:-----------|
    /// | `0` | `000`      |
    /// | `1` | `001`      |
    /// | `2` | `010`      |
    /// | `3` | `011`      |
    /// | `4` | `100`      |
    /// | `5` | `101`      |
    /// | `6` | `110`      |
    /// | `7` | `111`      |
    ///
    Bit (registers, memory, _cpu_flags) {

        /// Copies the complement of the contents of the specified bit in `m` to the Z flag of the program status word (PSW).
        Bit(operand: u8) [2] => {
            let (_value, register) = get_register_value(registers, memory, *operand);

            match register {
                None => Ok(3),
                _ => Ok(2),
            }
        }

        /// Sets the specified bit to 1 in `m`.
        Set(operand: u8) [2] => {
            let (_value, register) = get_register_value(registers, memory, *operand);

            match register {
                None => Ok(4),
                _ => Ok(2),
            }
        }

        /// Resets the specified bit to 0 in `m`.
        Res(operand: u8) [2] => {
            let (_value, register) = get_register_value(registers, memory, *operand);

            match register {
                None => Ok(4),
                _ => Ok(2),
            }
        }
    }
}

#[cfg(test)]
crate::instruction_tests! {

    bit_returns_the_correct_number_of_machine_cycles(registers, memory, cpu_flags) => {
        for operand in 0..8 {
            let cycles = Bit::Bit(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(3, cycles, "Incorrect number of machine cycles for HL");
            } else {
                assert_eq!(2, cycles, "Incorrect number of machine cycles for single register ({:08b})", operand);
            }
        }
    }

    set_returns_the_correct_number_of_machine_cycles(registers, memory, cpu_flags) => {
        for operand in 0..8 {
            let cycles = Bit::Set(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(4, cycles, "Incorrect number of machine cycles for HL");
            } else {
                assert_eq!(2, cycles, "Incorrect number of machine cycles for single register ({:08b})", operand);
            }
        }
    }

    res_returns_the_correct_number_of_machine_cycles(registers, memory, cpu_flags) => {
        for operand in 0..8 {
            let cycles = Bit::Res(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(4, cycles, "Incorrect number of machine cycles for HL");
            } else {
                assert_eq!(2, cycles, "Incorrect number of machine cycles for single register ({:08b})", operand);
            }
        }
    }
}
