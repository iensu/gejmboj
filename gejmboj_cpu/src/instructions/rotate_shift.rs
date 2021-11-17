use super::utils;
/// Rotate Shift instructions
///
/// Some of the Rotate Shift instructions share their opcode and it's necessary to
/// check the operand to distinguish between them.
///
/// | Opcode       | Operand     | Instruction |
/// |--------------|-------------|-------------|
/// | `0000_0111`  | -           | `RlcA`      |
/// | `0001_0111`  | -           | `RlA`       |
/// | `0000_1111`  | -           | `RrcA`      |
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
    registers::{DoubleRegister, Registers, SingleRegister},
};

/// Decodes the `operand` into a `RotateShift` instruction.
pub fn decode(operand: u8) -> Result<RotateShift, CpuError> {
    match utils::into_bits(operand) {
        (0, 0, 0, 0, 0, _, _, _) => Ok(RotateShift::RlC(operand)),
        (0, 0, 0, 0, 1, _, _, _) => Ok(RotateShift::RrC(operand)),
        (0, 0, 0, 1, 0, _, _, _) => Ok(RotateShift::Rl(operand)),
        (0, 0, 0, 1, 1, _, _, _) => Ok(RotateShift::Rr(operand)),
        (0, 0, 1, 0, 0, _, _, _) => Ok(RotateShift::SlA(operand)),
        (0, 0, 1, 0, 1, _, _, _) => Ok(RotateShift::SrA(operand)),
        (0, 0, 1, 1, 0, _, _, _) => Ok(RotateShift::Swap(operand)),
        (0, 0, 1, 1, 1, _, _, _) => Ok(RotateShift::SrL(operand)),
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
            let flags = 0b0000_0000 | (bit7 >> 3);
            let result = value.rotate_left(1);

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
        SlA(_operand: u8) [2] => {
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
        SrA(_operand: u8) [2] => {
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
        SrL(_operand: u8) [2] => {
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
