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

fn get_bit_mask(operand: &u8) -> u8 {
    let bit_designator = (operand >> 3) & 0b111;

    1 << bit_designator
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
            let (value, register) = get_register_value(registers, memory, *operand);
            let bit_mask = get_bit_mask(operand);
            let designated_bit = value & bit_mask;

            registers.set_zero(designated_bit == 0);
            registers.set_negative(false);
            registers.set_half_carry(true);

            match register {
                Some(_) => Ok(2),
                None => Ok(3),
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
mod tests {
    use super::*;

    #[test]
    fn get_bit_mask_works() {
        for (operand, expected) in vec![
            (0b01_000_111, 0b0000_0001),
            (0b01_001_111, 0b0000_0010),
            (0b01_010_111, 0b0000_0100),
            (0b01_011_111, 0b0000_1000),
            (0b01_100_111, 0b0001_0000),
            (0b01_101_111, 0b0010_0000),
            (0b01_110_111, 0b0100_0000),
            (0b01_111_111, 0b1000_0000),
        ] {
            assert_eq!(expected, get_bit_mask(&operand));
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

    bit_sets_zero_flag_to_zero_if_specified_bit_is_one(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0x80);

        Bit::Bit(0b01_111_111).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(false, registers.is_zero());
    }

    bit_sets_zero_flag_to_one_if_specified_bit_is_zero(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::L, 0xEF);

        Bit::Bit(0b01_100_101).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(true, registers.is_zero());
    }

    bit_sets_the_half_carry_flag(registers, memory, cpu_flags) => {
        Bit::Bit(0b01_111_111).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(true, registers.is_half_carry());
    }

    bit_resets_the_negative_flag(registers, memory, cpu_flags) => {
        registers.set_flags(MASK_FLAG_NEGATIVE);

        Bit::Bit(0b01_111_111).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(false, registers.is_negative());
    }

    bit_leaves_carry_flag_unchanged(registers, memory, cpu_flags) => {
        registers.set_flags(MASK_FLAG_CARRY);

        Bit::Bit(0b01_111_111).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(true, registers.is_carry());

        registers.set_flags(0);

        Bit::Bit(0b01_111_111).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

        assert_eq!(false, registers.is_carry());
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
