use crate::{
    memory::Memory,
    registers::{DoubleRegister, Registers, SingleRegister},
};

/// Instruction utility functions

pub fn into_bits(x: u8) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    (
        (x & 0b1000_0000) >> 7,
        (x & 0b0100_0000) >> 6,
        (x & 0b0010_0000) >> 5,
        (x & 0b0001_0000) >> 4,
        (x & 0b0000_1000) >> 3,
        (x & 0b0000_0100) >> 2,
        (x & 0b0000_0010) >> 1,
        x & 0b0000_0001,
    )
}

/// Return a tuple of the value from the register designated by the operand
/// and optionally the affected `SingleRegister`.
///
/// Reads either from a `SingleRegister` or `(HL)`.
pub fn get_register_value(
    registers: &Registers,
    memory: &Memory,
    operand: u8,
) -> (u8, Option<SingleRegister>) {
    match into_bits(operand) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_bits_works() {
        assert_eq!(into_bits(0b1000_0000), (1, 0, 0, 0, 0, 0, 0, 0));
        assert_eq!(into_bits(0b0100_0000), (0, 1, 0, 0, 0, 0, 0, 0));
        assert_eq!(into_bits(0b0010_0000), (0, 0, 1, 0, 0, 0, 0, 0));
        assert_eq!(into_bits(0b0001_0000), (0, 0, 0, 1, 0, 0, 0, 0));
        assert_eq!(into_bits(0b0000_1000), (0, 0, 0, 0, 1, 0, 0, 0));
        assert_eq!(into_bits(0b0000_0100), (0, 0, 0, 0, 0, 1, 0, 0));
        assert_eq!(into_bits(0b0000_0010), (0, 0, 0, 0, 0, 0, 1, 0));
        assert_eq!(into_bits(0b0000_0001), (0, 0, 0, 0, 0, 0, 0, 1));
        assert_eq!(into_bits(0b0000_0000), (0, 0, 0, 0, 0, 0, 0, 0));
        assert_eq!(into_bits(0b1111_1111), (1, 1, 1, 1, 1, 1, 1, 1));
        assert_eq!(into_bits(0b1000_1000), (1, 0, 0, 0, 1, 0, 0, 0));
    }

    #[test]
    fn get_register_value_gets_the_correct_registers() {
        let registers = Registers::new();
        let memory = Memory::new();

        for (operand, expected_register) in vec![
            (0b000, Some(SingleRegister::B)),
            (0b001, Some(SingleRegister::C)),
            (0b010, Some(SingleRegister::D)),
            (0b011, Some(SingleRegister::E)),
            (0b100, Some(SingleRegister::H)),
            (0b101, Some(SingleRegister::L)),
            (0b110, None), // HL
            (0b111, Some(SingleRegister::A)),
        ] {
            let (_, register) = get_register_value(&registers, &memory, operand);
            assert_eq!(expected_register, register);
        }
    }

    #[test]
    fn get_register_value_gets_the_correct_single_register_values() {
        let mut registers = Registers::new();
        let memory = Memory::new();

        for (operand, register, value) in vec![
            (0b000, SingleRegister::B, 1),
            (0b001, SingleRegister::C, 2),
            (0b010, SingleRegister::D, 3),
            (0b011, SingleRegister::E, 4),
            (0b100, SingleRegister::H, 5),
            (0b101, SingleRegister::L, 6),
            (0b111, SingleRegister::A, 7),
        ] {
            registers.set_single(&register, value);
            let (result, _) = get_register_value(&registers, &memory, operand);
            assert_eq!(value, result);
        }
    }

    #[test]
    fn get_register_value_gets_the_correct_value_for_hl() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();

        registers.set_double(&DoubleRegister::HL, 0xAB);
        memory.set(0xAB, 0xCD);

        let (result, _) = get_register_value(&registers, &memory, 0b110);
        assert_eq!(0xCD, result);
    }
}
