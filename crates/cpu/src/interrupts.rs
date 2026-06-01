#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Interrupt {
    VBlank,
    LCD_STAT,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    pub fn priority(&self) -> u8 {
        match self {
            Interrupt::VBlank => 0,
            Interrupt::LCD_STAT => 1,
            Interrupt::Timer => 2,
            Interrupt::Serial => 3,
            Interrupt::Joypad => 4,
        }
    }

    pub fn vector(&self) -> u16 {
        match self {
            Interrupt::VBlank => 0x0040,
            Interrupt::LCD_STAT => 0x0048,
            Interrupt::Timer => 0x0050,
            Interrupt::Serial => 0x0058,
            Interrupt::Joypad => 0x0060,
        }
    }
}
