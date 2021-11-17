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
}
