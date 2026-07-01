use crate::bus::{ADDR_IE, ADDR_IF, Bus};

const INTERRUPT_PRIORITY: [Interrupt; 5] = [
    Interrupt::VBlank,
    Interrupt::LCD,
    Interrupt::Timer,
    Interrupt::Serial,
    Interrupt::Joypad,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interrupt {
    VBlank,
    LCD,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    #[must_use]
    pub const fn mask(self) -> u8 {
        match self {
            Self::VBlank => 0b0000_0001,
            Self::LCD => 0b0000_0010,
            Self::Timer => 0b0000_0100,
            Self::Serial => 0b0000_1000,
            Self::Joypad => 0b0001_0000,
        }
    }

    #[must_use]
    pub fn is_pending(&self, bus: &Bus) -> bool {
        0x1F & bus.get(ADDR_IF) & bus.get(ADDR_IE) & self.mask() != 0
    }

    #[must_use]
    pub const fn vector(self) -> u16 {
        match self {
            Self::VBlank => 0x40,
            Self::LCD => 0x48,
            Self::Timer => 0x50,
            Self::Serial => 0x58,
            Self::Joypad => 0x60,
        }
    }
}

#[must_use]
pub fn next_pending_interrupt(bus: &Bus) -> Option<Interrupt> {
    INTERRUPT_PRIORITY.into_iter().find(|i| i.is_pending(bus))
}

#[cfg(test)]
mod tests {
    use super::Interrupt as I;
    use super::*;

    #[test]
    fn next_pending_interrupt_works() {
        let pending = 0;
        let bus = Bus::new().with_memory(&[(ADDR_IF, pending), (ADDR_IE, pending)]);
        assert_eq!(None, next_pending_interrupt(&bus));

        let pending = 0b1110_0000;
        let bus = Bus::new().with_memory(&[(ADDR_IF, pending), (ADDR_IE, pending)]);
        assert_eq!(None, next_pending_interrupt(&bus));

        let pending = I::VBlank.mask() | I::Timer.mask() | I::Joypad.mask();
        let bus = Bus::new().with_memory(&[(ADDR_IF, pending), (ADDR_IE, pending)]);
        assert_eq!(Some(I::VBlank), next_pending_interrupt(&bus));

        let pending = I::Timer.mask() | I::Joypad.mask();
        let bus = Bus::new().with_memory(&[(ADDR_IF, pending), (ADDR_IE, pending)]);
        assert_eq!(Some(I::Timer), next_pending_interrupt(&bus));

        let pending = I::Serial.mask() | I::Joypad.mask();
        let bus = Bus::new().with_memory(&[(ADDR_IF, pending), (ADDR_IE, pending)]);
        assert_eq!(Some(I::Serial), next_pending_interrupt(&bus));

        let pending = I::Joypad.mask();
        let bus = Bus::new().with_memory(&[(ADDR_IF, pending), (ADDR_IE, pending)]);
        assert_eq!(Some(I::Joypad), next_pending_interrupt(&bus));
    }
}
