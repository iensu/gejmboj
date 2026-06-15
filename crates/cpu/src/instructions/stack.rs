use crate::instruction_group;
use crate::registers::{DoubleRegister, MASK_FLAG_CARRY, MASK_FLAG_HALF_CARRY};

instruction_group! {
    /// Stack manipulation instructions.
    Stack(registers, _memory, _cpu_flags) {
        /// Add the signed value e8 to SP and copy the result in HL.
        ///
        /// H is set if overflow from bit 3.
        /// C is set if overflow from bit 7.
        ///
        /// LD HL,SP+e8 [0xF8]
        LD_HL_SP_E(operand: u8) [2] => {
            let extended = if operand.cast_signed() >= 0 {
                u16::from(*operand)
            }  else {
                0xFF00 | u16::from(*operand)
            };
            let mut flags = 0;
            let sp = registers.SP.wrapping_add(extended);

            if sp & 0xFF < registers.SP & 0xFF {
                flags |= MASK_FLAG_CARRY;
            }
            if sp & 0xF < registers.SP & 0xF {
                flags |= MASK_FLAG_HALF_CARRY;
            }

            registers.set_double(&DoubleRegister::HL, sp);
            registers.set_flags(flags);

            Ok(3)
        }
    }
}
