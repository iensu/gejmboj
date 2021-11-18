use super::utils;
/// Rotate Shift instructions
///
/// Some of the Rotate Shift instructions share their opcode and it's necessary to
/// check the operand to distinguish between them.
///
/// | Opcode       | Operand     | Instruction |
/// |--------------|-------------|-------------|
/// | `0000_0111`  | -           | `RlcA`      |
/// | `0000_1111`  | -           | `RrcA`      |
/// | `0001_0111`  | -           | `RlA`       |
/// | `0001_1111`  | -           | `RrA`       |
/// | `1100_1011`  | `0000_0110` | `RlC (HL)`  |
/// | `1100_1011`  | `0000_0rrr` | `RlC rrr`   |
/// | `1100_1011`  | `0000_1110` | `RrC (HL)`  |
/// | `1100_1011`  | `0000_1rrr` | `RrC rrr`   |
/// | `1100_1011`  | `0001_0110` | `Rl (HL)`   |
/// | `1100_1011`  | `0001_0rrr` | `Rl rrr`    |
/// | `1100_1011`  | `0001_1110` | `Rr (HL)`   |
/// | `1100_1011`  | `0001_1rrr` | `Rr rrr`    |
/// | `1101_1011`^ | `0010_0110` | `Sla (HL)`  |
/// | `1100_1011`  | `0010_0rrr` | `Sla rrr`   |
/// | `1100_1011`  | `0010_1110` | `Sra (HL)`  |
/// | `1100_1011`  | `0010_1rrr` | `Sra rrr`   |
/// | `1100_1011`  | `0011_0110` | `Swap (HL)` |
/// | `1100_1011`  | `0011_0rrr` | `Swap rrr`  |
/// | `1100_1011`  | `0011_1110` | `Srl (HL)`  |
/// | `1100_1011`  | `0011_1rrr` | `Srl rrr`   |
///
/// ^ Does not follow the general pattern so possibly a typo in the manual.
use crate::{
    errors::CpuError,
    instruction_group,
    memory::Memory,
    registers::{DoubleRegister, Registers, SingleRegister, MASK_FLAG_ZERO},
};

/// Decodes the `operand` into a `RotateShift` instruction.
///
/// | Operand      | Instruction |
/// |--------------|-------------|
/// | `00_000_rrr` | `RlC`       |
/// | `00_001_rrr` | `RrC`       |
/// | `00_010_rrr` | `Rl`        |
/// | `00_011_rrr` | `Rr`        |
/// | `00_100_rrr` | `Sla`       |
/// | `00_101_rrr` | `Sra`       |
/// | `00_110_rrr` | `Swap`      |
/// | `00_111_rrr` | `Srl`       |
pub fn decode(operand: u8) -> Result<RotateShift, CpuError> {
    match utils::into_bits(operand) {
        (0, 0, 0, 0, 0, _, _, _) => Ok(RotateShift::RlC(operand)),
        (0, 0, 0, 0, 1, _, _, _) => Ok(RotateShift::RrC(operand)),
        (0, 0, 0, 1, 0, _, _, _) => Ok(RotateShift::Rl(operand)),
        (0, 0, 0, 1, 1, _, _, _) => Ok(RotateShift::Rr(operand)),
        (0, 0, 1, 0, 0, _, _, _) => Ok(RotateShift::Sla(operand)),
        (0, 0, 1, 0, 1, _, _, _) => Ok(RotateShift::Sra(operand)),
        (0, 0, 1, 1, 0, _, _, _) => Ok(RotateShift::Swap(operand)),
        (0, 0, 1, 1, 1, _, _, _) => Ok(RotateShift::Srl(operand)),
        _ => Err(CpuError::UnknownInstruction(operand)),
    }
}

/// Return a tuple of the value from the register designated by the operand
/// and optionally the affected `SingleRegister`.
///
/// Reads either from a `SingleRegister` or `(HL)`.
fn get_register_value(
    registers: &Registers,
    memory: &Memory,
    operand: u8,
) -> (u8, Option<SingleRegister>) {
    match utils::into_bits(operand) {
        (_, _, _, _, _, 1, 1, 0) => {
            let value = memory.get(registers.get_double(&DoubleRegister::HL).into());
            (value, None)
        }
        (_, _, _, _, _, a, b, c) => {
            let r = (a, b, c).into();
            let value = registers.get_single(&r);
            (value, Some(r))
        }
    }
}

instruction_group! {
    /// Bit rotate and shift instructions.
    ///
    /// Some instructions operate on `m` which is indicated by the 8-bit operand passed
    /// to the instruction as per the following table:
    ///
    /// | Operand     | Target                                                    |
    /// |-------------|-----------------------------------------------------------|
    /// | `0000_0110` | `(HL)`, the memory contents pointed to by the HL register |
    /// | `0000_0rrr` | 8-bit register `rrr`                                      |
    RotateShift (registers, memory, _cpu_flags) {
        /// Rotate contents of register A to the left.
        /// Bit 7 is placed in both C and Bit 0.
        ///
        /// **Flags**
        ///
        /// | Flag | Effect        |
        /// |------|---------------|
        /// | Z    | `0`           |
        /// | N    | `0`           |
        /// | H    | `0`           |
        /// | C    | A<sup>7</sup> |
        RlcA() [1] => {
            unimplemented!()
        }

        /// Rotates contents of register A to the left.
        /// C is put in A<sup>0</sup> and A<sup>7</sup> is put in C.
        ///
        /// **Flags**
        ///
        /// | Flag | Effect        |
        /// |------|---------------|
        /// | Z    | `0`           |
        /// | N    | `0`           |
        /// | H    | `0`           |
        /// | C    | A<sup>7</sup> |
        RlA() [1] => {
            unimplemented!()
        }

        /// Rotate contents of register A to the right.
        /// Bit 0 is placed in both C and Bit 7.
        ///
        /// **Flags**
        ///
        /// | Flag | Effect        |
        /// |------|---------------|
        /// | Z    | `0`           |
        /// | N    | `0`           |
        /// | H    | `0`           |
        /// | C    | A<sup>0</sup> |
        RrcA() [1] => {
            unimplemented!()
        }

        /// Rotates contents of register A to the right.
        /// C is put in A<sup>7</sup> and A<sup>0</sup> is put in C.
        ///
        /// **Flags**
        ///
        /// | Flag | Effect        |
        /// |------|---------------|
        /// | Z    | `0`           |
        /// | N    | `0`           |
        /// | H    | `0`           |
        /// | C    | A<sup>0</sup> |
        RrA() [1] => {
            unimplemented!()
        }

        /// Rotates contents of `m` to the left.
        ///
        /// m<sup>7</sup> is copied to both C and m<sup>0</sup>.
        ///
        /// **Flags**
        ///
        /// | Flag | Effect        |
        /// |------|---------------|
        /// | Z    | Set if `0`    |
        /// | N    | `0`           |
        /// | H    | `0`           |
        /// | C    | m<sup>7</sup> |
        RlC(operand: u8) [2] => {
            let (value, register) = get_register_value(registers, memory, *operand);
            let bit7 = value & 0b1000_0000;
            let mut flags = 0b0000_0000 | (bit7 >> 3);
            let result = value.rotate_left(1);

            if result == 0 {
                flags |= MASK_FLAG_ZERO;
            }

            registers.set_single(&SingleRegister::F, flags);

            match register {
                Some(r) => {
                    registers.set_single(&r, result);
                    Ok(2)
                }
                None => {
                    memory.set(registers.get_double(&DoubleRegister::HL).into(), result);
                    Ok(4)
                }
            }
        }

        /// Rotates contents of `m` to the left.
        ///
        /// C is copied to m<sup>0</sup>.
        ///
        /// **Flags**
        ///
        /// | Flag | Effect        |
        /// |------|---------------|
        /// | Z    | Set if `0`    |
        /// | N    | `0`           |
        /// | H    | `0`           |
        /// | C    | m<sup>7</sup> |
        Rl(_operand: u8) [2] => {
            unimplemented!()
        }

        /// Rotates contents of `m` to the right.
        ///
        /// m<sup>0</sup> is copied to both C and m<sup>7</sup>.
        ///
        /// **Flags**
        ///
        /// | Flag | Effect        |
        /// |------|---------------|
        /// | Z    | Set if `0`    |
        /// | N    | `0`           |
        /// | H    | `0`           |
        /// | C    | m<sup>0</sup> |
        RrC(_operand: u8) [2] => {
            unimplemented!()
        }

        /// Rotates contents of `m` to the right.
        ///
        /// C is copied to m<sup>7</sup>.
        ///
        /// **Flags**
        ///
        /// | Flag | Effect        |
        /// |------|---------------|
        /// | Z    | Set if `0`    |
        /// | N    | `0`           |
        /// | H    | `0`           |
        /// | C    | m<sup>0</sup> |
        Rr(_operand: u8) [2] => {
            unimplemented!()
        }

        /// Shifts the contents of `m` to the left.
        ///
        /// m<sup>7</sup> is copied to C and m<sup>0</sup> is reset to 0.
        ///
        /// **Flags**
        ///
        /// | Flag | Effect        |
        /// |------|---------------|
        /// | Z    | Set if `0`    |
        /// | N    | `0`           |
        /// | H    | `0`           |
        /// | C    | m<sup>7</sup> |
        Sla(_operand: u8) [2] => {
            unimplemented!()
        }

        /// Shifts the contents of `m` to the right.
        ///
        /// m<sup>7</sup> is unchanged and m<sup>0</sup> is copied to C.
        ///
        /// **Flags**
        ///
        /// | Flag | Effect        |
        /// |------|---------------|
        /// | Z    | Set if `0`    |
        /// | N    | `0`           |
        /// | H    | `0`           |
        /// | C    | m<sup>0</sup> |
        Sra(_operand: u8) [2] => {
            unimplemented!()
        }

        /// Shifts the contents of `m` to the right.
        ///
        /// m<sup>7</sup> is set to 0 and m<sup>0</sup> is copied to C.
        ///
        /// **Flags**
        ///
        /// | Flag | Effect        |
        /// |------|---------------|
        /// | Z    | Set if `0`    |
        /// | N    | `0`           |
        /// | H    | `0`           |
        /// | C    | m<sup>0</sup> |
        Srl(_operand: u8) [2] => {
            unimplemented!()
        }

        /// Swaps the high and low nibble of `m`.
        ///
        /// **Flags**
        ///
        /// | Flag | Effect        |
        /// |------|---------------|
        /// | Z    | Set if `0`    |
        /// | N    | `0`           |
        /// | H    | `0`           |
        /// | C    | `0`           |
        Swap(_operand: u8) [2] => {
            unimplemented!()
        }
    }
}

#[cfg(test)]
crate::instruction_tests! {
    rlc_returns_the_correct_machine_cycles(registers, memory, cpu_flags) => {
        for operand in 0..8 {
            let cycles = RotateShift::RlC(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(4, cycles, "Incorrect number of machine cycles for HL");
            } else {
                assert_eq!(2, cycles, "Incorrect number of machine cycles for single register ({:08b})", operand);
            }
        }
    }

    rlc_rotates_the_correct_register(registers, memory, cpu_flags) => {
        use std::convert::TryInto;

        let value: u8 = 0b0101_0101;
        let expected: u8 = 0b1010_1010;

        for operand in 0..8 {
            if operand == 0b110 {
                memory.set(registers.get_double(&DoubleRegister::HL).into(), value);
            } else {
                registers.set_single(&operand.try_into().unwrap(), value);
            }

            RotateShift::RlC(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(expected, memory.get(registers.get_double(&DoubleRegister::HL).into()), "Incorrect result for (HL)");
            } else {
                let r: SingleRegister = operand.try_into().unwrap();
                assert_eq!(expected, registers.get_single(&r), "Incorrect result for register {:?}", r);
            }

            registers.clear();
        }
    }

    rlc_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::B, 0b0);
        RotateShift::RlC(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1000_0000, registers.get_single(&SingleRegister::F), "Z flag not set");

        registers.set_single(&SingleRegister::B, 0b1000_0000);
        RotateShift::RlC(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_single(&SingleRegister::F), "C flag not set");
    }
}
