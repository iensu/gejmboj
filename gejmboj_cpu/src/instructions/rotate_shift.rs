use super::utils::{self, get_register_value};
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
    registers::{DoubleRegister, SingleRegister, MASK_FLAG_CARRY, MASK_FLAG_ZERO},
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
        (0, 0, 0, 0, 0, _, _, _) => Ok(RotateShift::RLC(operand)),
        (0, 0, 0, 0, 1, _, _, _) => Ok(RotateShift::RRC(operand)),
        (0, 0, 0, 1, 0, _, _, _) => Ok(RotateShift::RL(operand)),
        (0, 0, 0, 1, 1, _, _, _) => Ok(RotateShift::RR(operand)),
        (0, 0, 1, 0, 0, _, _, _) => Ok(RotateShift::SLA(operand)),
        (0, 0, 1, 0, 1, _, _, _) => Ok(RotateShift::SRA(operand)),
        (0, 0, 1, 1, 0, _, _, _) => Ok(RotateShift::SWAP(operand)),
        (0, 0, 1, 1, 1, _, _, _) => Ok(RotateShift::SRL(operand)),
        _ => Err(CpuError::UnknownInstruction(operand)),
    }
}

/// Configuration for the Op operations.
#[derive(Default)]
struct OpConfig {
    /// Set to `true` if the Carry bit should be added to the result.
    add_carry: bool,
    /// Set to `true` if the Zero flag should be handled in the operation.
    set_z: bool,
    /// Set to `true` if the tailing bit should be repeated instead of 0 when shifting.
    repeat_tail: bool,
}

impl OpConfig {
    pub fn builder() -> OpConfigBuilder {
        OpConfigBuilder::new()
    }
}

#[derive(Default)]
struct OpConfigBuilder {
    config: OpConfig,
}

impl OpConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: OpConfig::default(),
        }
    }

    pub fn set_z(mut self) -> OpConfigBuilder {
        self.config.set_z = true;
        self
    }

    pub fn add_carry(mut self) -> OpConfigBuilder {
        self.config.add_carry = true;
        self
    }

    pub fn repeat_tail(mut self) -> OpConfigBuilder {
        self.config.repeat_tail = true;
        self
    }

    pub fn build(self) -> OpConfig {
        self.config
    }
}

enum Op {
    RotateLeft(u8),
    RotateRight(u8),
    ShiftLeft(u8),
    ShiftRight(u8),
}

impl Op {
    /// Run the designated function and return a tuple of (result, flags).
    ///
    /// `flags` is the desired default configuration of the register flags.
    ///
    /// If `add_carry` is `true` the carry bit is set on the result on either the
    /// first or last bit depending on direction.
    ///
    /// If `set_z` is `true` the Z flag will be set if the result is 0.
    pub fn execute(&self, flags: u8, config: &OpConfig) -> (u8, u8) {
        let mut result = match self {
            Op::RotateLeft(x) => x.rotate_left(1),
            Op::RotateRight(x) => x.rotate_right(1),
            Op::ShiftLeft(x) => x << 1,
            Op::ShiftRight(x) => x >> 1,
        };
        let (to_carry, from_carry, tail_bit) = match self {
            Op::RotateLeft(x) | Op::ShiftLeft(x) => (x & 0x80, 0x01, x & 0x01),
            Op::RotateRight(x) | Op::ShiftRight(x) => (x & 0x01, 0x80, x & 0x80),
        };

        if config.add_carry && flags & MASK_FLAG_CARRY > 0 {
            result |= from_carry;
        }
        if config.repeat_tail {
            result |= tail_bit;
        }

        let mut flags = flags;
        if to_carry > 0 {
            flags |= MASK_FLAG_CARRY;
        } else {
            flags &= !MASK_FLAG_CARRY;
        }
        if config.set_z && result == 0 {
            flags |= MASK_FLAG_ZERO;
        } else if config.set_z {
            flags &= !MASK_FLAG_ZERO;
        }

        (result, flags)
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
        RLCA() [1] => {
            let value = registers.get_single(&SingleRegister::A);
            let (result, flags) = Op::RotateLeft(value).execute(0, &OpConfig::default());
            registers.set_single(&SingleRegister::A, result);
            registers.set_flags(flags);
            Ok(1)
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
        RLA() [1] => {
            let value = registers.get_single(&SingleRegister::A);
            let (result, flags) = Op::RotateLeft(value).execute(
                registers.get_flags() & MASK_FLAG_CARRY,
                &OpConfig::builder().add_carry().build(),
            );
            registers.set_single(&SingleRegister::A, result);
            registers.set_flags(flags);
            Ok(1)
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
        RRCA() [1] => {
            let value = registers.get_single(&SingleRegister::A);
            let (result, flags) = Op::RotateRight(value).execute(0, &OpConfig::default());
            registers.set_single(&SingleRegister::A, result);
            registers.set_flags(flags);
            Ok(1)
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
        RRA() [1] => {
            let value = registers.get_single(&SingleRegister::A);
            let (result, flags) = Op::RotateRight(value).execute(
                registers.get_flags() & MASK_FLAG_CARRY,
                &OpConfig::builder().add_carry().build()
            );
            registers.set_single(&SingleRegister::A, result);
            registers.set_flags(flags);
            Ok(1)
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
        RLC(operand: u8) [2] => {
            let (value, register) = get_register_value(registers, memory, *operand);
            let (result, flags) = Op::RotateLeft(value).execute(0, &OpConfig::builder().set_z().build());

            registers.set_flags(flags);

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
        RL(operand: u8) [2] => {
            let (value, register) = get_register_value(registers, memory, *operand);
            let (result, flags) = Op::RotateLeft(value).execute(
                registers.get_flags() & MASK_FLAG_CARRY,
                &OpConfig::builder().add_carry().set_z().build()
            );

            registers.set_flags(flags);

            match register {
                Some(r) => {
                    registers.set_single(&r, result);
                    Ok(2)
                },
                None => {
                    memory.set(registers.get_double(&DoubleRegister::HL).into(), result);
                    Ok(4)
                }
            }
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
        RRC(operand: u8) [2] => {
            let (value, register) = get_register_value(registers, memory, *operand);
            let (result, flags) = Op::RotateRight(value).execute(0, &OpConfig::builder().set_z().build());

            registers.set_flags(flags);

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
        RR(operand: u8) [2] => {
            let (value, register) = get_register_value(registers, memory, *operand);
            let (result, flags) = Op::RotateRight(value).execute(
                registers.get_flags() & MASK_FLAG_CARRY,
                &OpConfig::builder().add_carry().set_z().build()
            );

            registers.set_flags(flags);

            match register {
                Some(r) => {
                    registers.set_single(&r, result);
                    Ok(2)
                },
                None => {
                    memory.set(registers.get_double(&DoubleRegister::HL).into(), result);
                    Ok(4)
                }
            }
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
        SLA(operand: u8) [2] => {
            let (value, register) = get_register_value(registers, memory, *operand);
            let (result, flags) = Op::ShiftLeft(value).execute(0, &OpConfig::builder().set_z().build());

            registers.set_flags(flags);

            match register {
                Some(r) => {
                    registers.set_single(&r, result);
                    Ok(2)
                },
                None => {
                    memory.set(registers.get_double(&DoubleRegister::HL).into(), result);
                    Ok(4)
                }
            }
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
        SRA(operand: u8) [2] => {
            let (value, register) = get_register_value(registers, memory, *operand);
            let (result, flags) = Op::ShiftRight(value).execute(0, &OpConfig::builder().set_z().repeat_tail().build());

            registers.set_flags(flags);

            match register {
                Some(r) => {
                    registers.set_single(&r, result);
                    Ok(2)
                },
                None => {
                    memory.set(registers.get_double(&DoubleRegister::HL).into(), result);
                    Ok(4)
                }
            }
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
        SRL(operand: u8) [2] => {
            let (value, register) = get_register_value(registers, memory, *operand);
            let (result, flags) = Op::ShiftRight(value).execute(0, &OpConfig::builder().set_z().build());

            registers.set_flags(flags);

            match register {
                Some(r) => {
                    registers.set_single(&r, result);
                    Ok(2)
                },
                None => {
                    memory.set(registers.get_double(&DoubleRegister::HL).into(), result);
                    Ok(4)
                }
            }
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
        SWAP(operand: u8) [2] => {
            let (value, register) = get_register_value(registers, memory, *operand);

            let flags = if value == 0 { MASK_FLAG_ZERO } else { 0 };
            registers.set_flags(flags);

            let lo_nibble = value & 0x0F;
            let hi_nibble = value & 0xF0;
            let result = (lo_nibble << 4) + (hi_nibble >> 4);

            match register {
                Some(r) => {
                    registers.set_single(&r, result);
                    Ok(2)
                },
                None => {
                    memory.set(registers.get_double(&DoubleRegister::HL).into(), result);
                    Ok(4)
                }
            }
        }
    }
}

#[cfg(test)]
crate::instruction_tests! {
    rlca_takes_1_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = RotateShift::RLCA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(1, cycles);
    }

    rlca_rotates_the_a_register_left(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b0001_0100);
        RotateShift::RLCA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0010_1000, registers.get_single(&SingleRegister::A));
    }

    rlca_sets_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_flags(0b1110_0000);
        RotateShift::RLCA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0000_0000, registers.get_flags(), "Did not clear Z, N and H");
        registers.clear();

        registers.set_single(&SingleRegister::A, 0b0101_0101);
        RotateShift::RLCA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0000_0000, registers.get_flags(), "C should NOT be set");
        registers.clear();

        registers.set_single(&SingleRegister::A, 0b1010_1010);
        RotateShift::RLCA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "C should be set");
    }

    rla_takes_1_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = RotateShift::RLA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(1, cycles);
    }

    rla_rotates_the_a_register_left(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b0001_0100);
        RotateShift::RLA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0010_1000, registers.get_single(&SingleRegister::A));
    }

    rla_sets_bit0_to_c(registers, memory, cpu_flags) => {
        registers.set_flags(MASK_FLAG_CARRY);
        registers.set_single(&SingleRegister::A, 0b0000_0000);
        RotateShift::RLA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0000_0001, registers.get_single(&SingleRegister::A));
    }

    rla_sets_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_flags(0b1110_0000);
        RotateShift::RLA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0000_0000, registers.get_flags(), "Did not clear Z, N and H");
        registers.clear();

        registers.set_single(&SingleRegister::A, 0b0101_0101);
        RotateShift::RLA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0000_0000, registers.get_flags(), "C should NOT be set");
        registers.clear();

        registers.set_single(&SingleRegister::A, 0b1010_1010);
        RotateShift::RLA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "C should be set");
    }

    rrca_takes_1_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = RotateShift::RRCA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(1, cycles);
    }

    rrca_rotates_the_a_register_right(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b0010_1000);
        RotateShift::RRCA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0100, registers.get_single(&SingleRegister::A));
    }

    rrca_sets_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_flags(0b1110_0000);
        RotateShift::RRCA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0000_0000, registers.get_flags(), "Did not clear Z, N and H");
        registers.clear();

        registers.set_single(&SingleRegister::A, 0b0101_0101);
        RotateShift::RRCA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "C should be set");
        registers.clear();

        registers.set_single(&SingleRegister::A, 0b1010_1010);
        RotateShift::RRCA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0000_0000, registers.get_flags(), "C should NOT be set");
        registers.clear();
    }

    rra_takes_1_machine_cycle(registers, memory, cpu_flags) => {
        let cycles = RotateShift::RRA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(1, cycles);
    }

    rra_rotates_the_a_register_left(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::A, 0b001_01000);
        RotateShift::RRA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0100, registers.get_single(&SingleRegister::A));
    }

    rra_sets_bit7_to_c(registers, memory, cpu_flags) => {
        registers.set_flags(MASK_FLAG_CARRY);
        registers.set_single(&SingleRegister::A, 0b0000_0000);
        RotateShift::RRA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1000_0000, registers.get_single(&SingleRegister::A));
    }

    rra_sets_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_flags(0b1110_0000);
        RotateShift::RRA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0000_0000, registers.get_flags(), "Did not clear Z, N and H");
        registers.clear();

        registers.set_single(&SingleRegister::A, 0b0101_0101);
        RotateShift::RRA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "C should be set");
        registers.clear();

        registers.set_single(&SingleRegister::A, 0b1010_1010);
        RotateShift::RRA().execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0000_0000, registers.get_flags(), "C should NOT be set");
        registers.clear();
    }

    rlc_returns_the_correct_machine_cycles(registers, memory, cpu_flags) => {
        for operand in 0..8 {
            let cycles = RotateShift::RLC(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

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

            RotateShift::RLC(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

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
        RotateShift::RLC(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1000_0000, registers.get_flags(), "Z flag not set");
        registers.clear();

        registers.set_single(&SingleRegister::B, 0b1000_0000);
        RotateShift::RLC(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "C flag not set");
        registers.clear();
    }

    rrc_returns_the_correct_machine_cycles(registers, memory, cpu_flags) => {
        for operand in 0..8 {
            let cycles = RotateShift::RRC(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(4, cycles, "Incorrect number of machine cycles for HL");
            } else {
                assert_eq!(2, cycles, "Incorrect number of machine cycles for single register ({:08b})", operand);
            }
        }
    }

    rrc_rotates_the_correct_register(registers, memory, cpu_flags) => {
        use std::convert::TryInto;

        let value: u8 = 0b0010_1000;
        let expected: u8 = 0b0001_0100;

        for operand in 0..8 {
            if operand == 0b110 {
                memory.set(registers.get_double(&DoubleRegister::HL).into(), value);
            } else {
                registers.set_single(&operand.try_into().unwrap(), value);
            }

            RotateShift::RRC(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(expected, memory.get(registers.get_double(&DoubleRegister::HL).into()), "Incorrect result for (HL)");
            } else {
                let r: SingleRegister = operand.try_into().unwrap();
                assert_eq!(expected, registers.get_single(&r), "Incorrect result for register {:?}", r);
            }

            registers.clear();
        }
    }

    rrc_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::B, 0b0);
        RotateShift::RRC(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1000_0000, registers.get_flags(), "Z flag not set");
        registers.clear();

        registers.set_single(&SingleRegister::B, 0b0000_0001);
        RotateShift::RRC(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "C flag not set");
        registers.clear();
    }

    rl_returns_the_correct_machine_cycles(registers, memory, cpu_flags) => {
        for operand in 0..8 {
            let cycles = RotateShift::RL(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(4, cycles, "Incorrect number of machine cycles for HL");
            } else {
                assert_eq!(2, cycles, "Incorrect number of machine cycles for single register ({:08b})", operand);
            }
        }
    }

    rl_rotates_the_correct_register_to_the_left(registers, memory, cpu_flags) => {
        use std::convert::TryInto;

        let value: u8 = 0b0001_0100;
        let expected: u8 = 0b0010_1000;

        for operand in 0..8 {
            if operand == 0b110 {
                memory.set(registers.get_double(&DoubleRegister::HL).into(), value);
            } else {
                registers.set_single(&operand.try_into().unwrap(), value);
            }

            RotateShift::RL(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(expected, memory.get(registers.get_double(&DoubleRegister::HL).into()), "Incorrect result for (HL)");
            } else {
                let r: SingleRegister = operand.try_into().unwrap();
                assert_eq!(expected, registers.get_single(&r), "Incorrect result for register {:?}", r);
            }

            registers.clear();
        }
    }

    rl_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::B, 0b0);
        RotateShift::RL(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1000_0000, registers.get_flags(), "Z flag not set");
        registers.clear();

        registers.set_single(&SingleRegister::B, 0b1000_0000);
        RotateShift::RL(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "C flag not set");
        registers.clear();

        registers.set_flags(MASK_FLAG_CARRY);
        RotateShift::RL(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0000_0001, registers.get_single(&SingleRegister::B), "C flag not moved to m0");
        println!("Flags: {:08b}", registers.get_flags());
        assert_eq!(false, registers.is_carry(), "C flag was still set");
        registers.clear();
    }

    rr_returns_the_correct_machine_cycles(registers, memory, cpu_flags) => {
        for operand in 0..8 {
            let cycles = RotateShift::RR(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(4, cycles, "Incorrect number of machine cycles for HL");
            } else {
                assert_eq!(2, cycles, "Incorrect number of machine cycles for single register ({:08b})", operand);
            }
        }
    }

    rr_rotates_the_correct_register_to_the_right(registers, memory, cpu_flags) => {
        use std::convert::TryInto;

        let value: u8 = 0b0010_1000;
        let expected: u8 = 0b0001_0100;

        for operand in 0..8 {
            if operand == 0b110 {
                memory.set(registers.get_double(&DoubleRegister::HL).into(), value);
            } else {
                registers.set_single(&operand.try_into().unwrap(), value);
            }

            RotateShift::RR(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(expected, memory.get(registers.get_double(&DoubleRegister::HL).into()), "Incorrect result for (HL)");
            } else {
                let r: SingleRegister = operand.try_into().unwrap();
                assert_eq!(expected, registers.get_single(&r), "Incorrect result for register {:?}", r);
            }

            registers.clear();
        }
    }

    rr_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::B, 0b0);
        RotateShift::RR(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1000_0000, registers.get_flags(), "Z flag not set");
        registers.clear();

        registers.set_single(&SingleRegister::B, 0b0000_0001);
        RotateShift::RR(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "C flag not set");
        registers.clear();

        registers.set_flags(MASK_FLAG_CARRY);
        RotateShift::RR(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1000_0000, registers.get_single(&SingleRegister::B), "C flag not moved to m7");
        assert_eq!(false, registers.is_carry(), "C flag was still set");
        registers.clear();
    }

    sla_returns_the_correct_machine_cycles(registers, memory, cpu_flags) => {
        for operand in 0..8 {
            let cycles = RotateShift::SLA(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(4, cycles, "Incorrect number of machine cycles for HL");
            } else {
                assert_eq!(2, cycles, "Incorrect number of machine cycles for single register ({:08b})", operand);
            }
        }
    }

    sla_shifts_the_correct_register_to_the_left(registers, memory, cpu_flags) => {
        use std::convert::TryInto;

        let value: u8 = 0b1001_0100;
        let expected: u8 = 0b0010_1000;

        for operand in 0..8 {
            if operand == 0b110 {
                memory.set(registers.get_double(&DoubleRegister::HL).into(), value);
            } else {
                registers.set_single(&operand.try_into().unwrap(), value);
            }

            RotateShift::SLA(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(expected, memory.get(registers.get_double(&DoubleRegister::HL).into()), "Incorrect result for (HL)");
            } else {
                let r: SingleRegister = operand.try_into().unwrap();
                assert_eq!(expected, registers.get_single(&r), "Incorrect result for register {:?}", r);
            }

            registers.clear();
        }
    }

    sla_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::B, 0b0);
        RotateShift::SLA(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1000_0000, registers.get_flags(), "Z flag not set");
        registers.clear();

        registers.set_single(&SingleRegister::B, 0b1000_0001);
        RotateShift::SLA(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "C flag not set");
        registers.clear();

        registers.set_single(&SingleRegister::B, 0b1000_0000);
        RotateShift::SLA(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1001_0000, registers.get_flags(), "C and Z flags not set");
        registers.clear();
    }

    sla_sets_the_correct_values(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::D, 0x80);
        memory.set(registers.get_double(&DoubleRegister::HL).into(), 0xFF);

        RotateShift::SLA(0b010).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0, registers.get_single(&SingleRegister::D));
        assert_eq!(true, registers.is_carry());
        assert_eq!(true, registers.is_zero());
        assert_eq!(false, registers.is_half_carry());
        assert_eq!(false, registers.is_negative());

        RotateShift::SLA(0b110).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0xfe, memory.get(registers.get_double(&DoubleRegister::HL).into()));
        assert_eq!(true, registers.is_carry());
        assert_eq!(false, registers.is_zero());
        assert_eq!(false, registers.is_half_carry());
        assert_eq!(false, registers.is_negative());
    }

    sra_returns_the_correct_machine_cycles(registers, memory, cpu_flags) => {
        for operand in 0..8 {
            let cycles = RotateShift::SRA(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(4, cycles, "Incorrect number of machine cycles for HL");
            } else {
                assert_eq!(2, cycles, "Incorrect number of machine cycles for single register ({:08b})", operand);
            }
        }
    }

    sra_shifts_the_correct_register_to_the_right(registers, memory, cpu_flags) => {
        use std::convert::TryInto;

        let value: u8 = 0b0010_1001;
        let expected: u8 = 0b0001_0100;

        for operand in 0..8 {
            if operand == 0b110 {
                memory.set(registers.get_double(&DoubleRegister::HL).into(), value);
            } else {
                registers.set_single(&operand.try_into().unwrap(), value);
            }

            RotateShift::SRA(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(expected, memory.get(registers.get_double(&DoubleRegister::HL).into()), "Incorrect result for (HL)");
            } else {
                let r: SingleRegister = operand.try_into().unwrap();
                assert_eq!(expected, registers.get_single(&r), "Incorrect result for register {:?}", r);
            }

            registers.clear();
        }
    }

    sra_repeats_the_seventh_bit(registers, memory, cpu_flags) => {
        use std::convert::TryInto;

        let value: u8 = 0b1010_1000;
        let expected: u8 = 0b1101_0100;

        for operand in 0..8 {
            if operand == 0b110 {
                memory.set(registers.get_double(&DoubleRegister::HL).into(), value);
            } else {
                registers.set_single(&operand.try_into().unwrap(), value);
            }

            RotateShift::SRA(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(expected, memory.get(registers.get_double(&DoubleRegister::HL).into()), "Incorrect result for (HL)");
            } else {
                let r: SingleRegister = operand.try_into().unwrap();
                assert_eq!(expected, registers.get_single(&r), "Incorrect result for register {:?}", r);
            }

            registers.clear();
        }
    }

    sra_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::B, 0b0);
        RotateShift::SRA(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1000_0000, registers.get_flags(), "Z flag not set");
        registers.clear();

        registers.set_single(&SingleRegister::B, 0b1000_0001);
        RotateShift::SRA(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "C flag not set");
        registers.clear();

        registers.set_single(&SingleRegister::B, 0b0000_0001);
        RotateShift::SRA(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1001_0000, registers.get_flags(), "C and Z flags not set");
        registers.clear();
    }

    srl_returns_the_correct_machine_cycles(registers, memory, cpu_flags) => {
        for operand in 0..8 {
            let cycles = RotateShift::SRL(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(4, cycles, "Incorrect number of machine cycles for HL");
            } else {
                assert_eq!(2, cycles, "Incorrect number of machine cycles for single register ({:08b})", operand);
            }
        }
    }

    srl_shifts_the_correct_register_to_the_right(registers, memory, cpu_flags) => {
        use std::convert::TryInto;

        let value: u8 = 0b0010_1001;
        let expected: u8 = 0b0001_0100;

        for operand in 0..8 {
            if operand == 0b110 {
                memory.set(registers.get_double(&DoubleRegister::HL).into(), value);
            } else {
                registers.set_single(&operand.try_into().unwrap(), value);
            }

            RotateShift::SRL(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(expected, memory.get(registers.get_double(&DoubleRegister::HL).into()), "Incorrect result for (HL)");
            } else {
                let r: SingleRegister = operand.try_into().unwrap();
                assert_eq!(expected, registers.get_single(&r), "Incorrect result for register {:?}", r);
            }

            registers.clear();
        }
    }

    srl_does_not_repeat_the_seventh_bit(registers, memory, cpu_flags) => {
        use std::convert::TryInto;

        let value: u8 = 0b1010_1000;
        let expected: u8 = 0b0101_0100;

        for operand in 0..8 {
            if operand == 0b110 {
                memory.set(registers.get_double(&DoubleRegister::HL).into(), value);
            } else {
                registers.set_single(&operand.try_into().unwrap(), value);
            }

            RotateShift::SRL(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(expected, memory.get(registers.get_double(&DoubleRegister::HL).into()), "Incorrect result for (HL)");
            } else {
                let r: SingleRegister = operand.try_into().unwrap();
                assert_eq!(expected, registers.get_single(&r), "Incorrect result for register {:?}", r);
            }

            registers.clear();
        }
    }

    srl_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::B, 0b0);
        RotateShift::SRL(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1000_0000, registers.get_flags(), "Z flag not set");
        registers.clear();

        registers.set_single(&SingleRegister::B, 0b1000_0001);
        RotateShift::SRL(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b0001_0000, registers.get_flags(), "C flag not set");
        registers.clear();

        registers.set_single(&SingleRegister::B, 0b0000_0001);
        RotateShift::SRL(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1001_0000, registers.get_flags(), "C and Z flags not set");
        registers.clear();
    }

    swap_returns_the_correct_machine_cycles(registers, memory, cpu_flags) => {
        for operand in 0..8 {
            let cycles = RotateShift::SWAP(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(4, cycles, "Incorrect number of machine cycles for HL");
            } else {
                assert_eq!(2, cycles, "Incorrect number of machine cycles for single register ({:08b})", operand);
            }
        }
    }

    swap_shifts_the_correct_register_to_the_right(registers, memory, cpu_flags) => {
        use std::convert::TryInto;

        let value: u8 = 0b0011_1100;
        let expected: u8 = 0b1100_0011;

        for operand in 0..8 {
            if operand == 0b110 {
                memory.set(registers.get_double(&DoubleRegister::HL).into(), value);
            } else {
                registers.set_single(&operand.try_into().unwrap(), value);
            }

            RotateShift::SWAP(operand).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();

            if operand == 0b110 {
                assert_eq!(expected, memory.get(registers.get_double(&DoubleRegister::HL).into()), "Incorrect result for (HL)");
            } else {
                let r: SingleRegister = operand.try_into().unwrap();
                assert_eq!(expected, registers.get_single(&r), "Incorrect result for register {:?}", r);
            }

            registers.clear();
        }
    }

    swap_handles_flags_correctly(registers, memory, cpu_flags) => {
        registers.set_single(&SingleRegister::B, 0b0);
        RotateShift::SWAP(0).execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
        assert_eq!(0b1000_0000, registers.get_flags(), "Z flag not set");
        registers.clear();
    }
}
