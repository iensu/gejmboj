#[derive(Debug, Clone, Copy)]
pub enum Interrupt {
    Timer,
}

impl Interrupt {
    #[must_use]
    pub fn mask(self) -> u8 {
        match self {
            Self::Timer => 0b000_0100,
        }
    }

    #[must_use]
    pub fn vector(self) -> u16 {
        match self {
            Self::Timer => 0x50,
        }
    }
}
