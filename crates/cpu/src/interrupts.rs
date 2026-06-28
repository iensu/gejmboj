use crate::bus::{ADDR_IE, ADDR_IF, Bus};

const INTERRUPT_PRIORITY: [Interrupt; 5] = [
    Interrupt::VBlank,
    Interrupt::LCD,
    Interrupt::Timer,
    Interrupt::Serial,
    Interrupt::Joypad,
];

#[derive(Debug, Clone, Copy)]
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
