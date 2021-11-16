use crate::{instruction_group, registers::DoubleRegister};

instruction_group! {
    /// 16-bit ALU instructions
    ALU16Bit (_registers, _memory, _cpu_flags) {
        AddHL(r: DoubleRegister) [1] => {
            unimplemented!()
        }

        AddSP(operand: u8) [2] => {
            unimplemented!()
        }

        Inc(r: DoubleRegister) [1] => {
            unimplemented!()
        }

        Dec(r: DoubleRegister) [1] => {
            unimplemented!()
        }
    }
}
