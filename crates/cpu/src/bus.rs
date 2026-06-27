//! # Bus implementation
//!
//! The bus contains memory data which is stored in Little-Endian fashion, which means that
//! the least significant byte is stored at a lower memory location than the most significant
//! byte.
//!
//! ## Memory map
//!
//! ```asciidoc
//! 0000-7FFF: External bus (ROM)
//! 8000-9FFF: VRAM
//! A000-BFFF: External bus (RAM)
//! C000-DFFF: WRAM
//! E000-FDFF: Echo (WRAM)
//! FE00-FE9F: Object Attribute Memory (OAM)
//! FEA0-FEFF: Invalid OAM
//! FF00-FF7F: Memory mapped I/O
//! FF80-FFFE: High RAM (HRAM)
//! FFFF:      IE register
//! ```

use std::fmt::Display;

use crate::{cycles::MachineCycles, errors::CpuError, interrupts::Interrupt};

pub const MASK_TIMER_ENABLED: u8 = 0b0000_0100;
const MASK_TIMER_SELECT: u8 = 0b0000_0011;

#[derive(Debug, Default)]
pub enum Clock {
    #[default]
    T1024,
    T16,
    T64,
    T256,
}

impl Clock {
    #[rustfmt::skip]
    #[must_use]
    pub fn mask(&self) -> u16 {
        match self {
            Self::T16   => 0b0000_0000_0000_1000,
            Self::T64   => 0b0000_0000_0010_0000,
            Self::T256  => 0b0000_0000_1000_0000,
            Self::T1024 => 0b0000_0010_0000_0000,
        }
    }
}

impl From<u8> for Clock {
    fn from(tac: u8) -> Self {
        let selected = tac & MASK_TIMER_SELECT;
        match selected {
            0 => Self::T1024,
            1 => Self::T16,
            2 => Self::T64,
            3 => Self::T256,
            _ => unreachable!("Invalid TAC value: {tac:08b}"),
        }
    }
}

impl From<Clock> for u8 {
    fn from(value: Clock) -> Self {
        match value {
            Clock::T1024 => 0,
            Clock::T16 => 1,
            Clock::T64 => 2,
            Clock::T256 => 3,
        }
    }
}

pub struct Bus {
    /// Internal counter.
    counter: u16,
    memory: Vec<u8>,
    clock: Clock,
    timer_enabled: bool,
    timer_reset_t_cycles: Option<u8>,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    #[must_use]
    pub fn new() -> Self {
        Self {
            counter: 0,
            // 65536 bytes which is 0xFFFF + 1
            memory: vec![0; 0xFFFF + 1],
            clock: Clock::default(),
            timer_enabled: false,
            timer_reset_t_cycles: None,
        }
    }

    /// Reset memory to expected startup values.
    ///
    /// References:
    /// - <https://gbdev.io/pandocs/Power_Up_Sequence.html#hardware-registers>
    pub fn reset(&mut self) {
        self.set(ADDR_TIMA, 0x00);
        self.set(ADDR_TMA, 0x00);
        self.set(ADDR_TAC, 0x00);
        self.set(ADDR_IF, 0xE1);
        self.set(ADDR_NR10, 0x80);
        self.set(ADDR_NR11, 0xBF);
        self.set(ADDR_NR12, 0xF3);
        self.set(ADDR_NR14, 0xBF);
        self.set(ADDR_NR21, 0x3F);
        self.set(ADDR_NR22, 0x00);
        self.set(ADDR_NR24, 0xBF);
        self.set(ADDR_NR30, 0x7F);
        self.set(ADDR_NR31, 0xFF);
        self.set(ADDR_NR32, 0x9F);
        self.set(ADDR_NR34, 0xBF);
        self.set(ADDR_NR41, 0xFF);
        self.set(ADDR_NR42, 0x00);
        self.set(ADDR_NR43, 0x00);
        self.set(ADDR_NR44, 0xBF);
        self.set(ADDR_NR50, 0x77);
        self.set(ADDR_NR51, 0xF3);
        self.set(ADDR_NR52, 0xF1);
        self.set(ADDR_LCDC, 0x91);
        self.set(ADDR_SCY, 0x00);
        self.set(ADDR_SCX, 0x00);
        self.set(ADDR_LYC, 0x00);
        self.set(ADDR_BGP, 0xFC);
        self.set(ADDR_OBP0, 0xFF);
        self.set(ADDR_OBP1, 0xFF);
        self.set(ADDR_WY, 0x00);
        self.set(ADDR_WX, 0x00);
        self.set(ADDR_IE, 0x00);
    }

    /// Increase the internal counter by `machine_cycles`.
    pub fn tick(&mut self, machine_cycles: MachineCycles) {
        for _ in 0..machine_cycles.to_t_cycles() {
            self.t_cycle_tick();
        }
    }

    /// Increase the internal counter by 1 T-cycle.
    fn t_cycle_tick(&mut self) {
        if let Some(cycles) = self.timer_reset_t_cycles {
            if cycles > 1 {
                self.timer_reset_t_cycles = Some(cycles - 1);
            } else {
                let value = self.get(ADDR_IF) | Interrupt::Timer.mask();
                self.set(ADDR_IF, value);
                self.set(ADDR_TIMA, self.get(ADDR_TMA));
                self.timer_reset_t_cycles = None;
            }
        }

        let previous_counter = self.counter;
        self.counter = self.counter.wrapping_add(1);

        if self.timer_enabled {
            let clock_mask = self.clock.mask();
            if previous_counter & clock_mask > 0 && self.counter & clock_mask == 0 {
                let current = self.get(ADDR_TIMA);
                let (value, overflow) = current.overflowing_add(1);
                self.set(ADDR_TIMA, value);

                if overflow {
                    self.timer_reset_t_cycles = Some(4);
                }
            }
        }
    }

    /// Load bytes into memory starting at 0x0000.
    ///
    /// Returns an error if the `data` size exceeds the maximum memory size.
    pub fn load(&mut self, data: &[u8]) -> Result<(), CpuError> {
        let data_size = data.len();
        if data_size > 0x8000 {
            return Err(CpuError::MemoryExceeded {
                size: data_size,
                max: 0x8000,
            });
        }

        self.memory[..data.len()].copy_from_slice(data);

        Ok(())
    }

    /// Sets a `u8` value in memory.
    ///
    /// ```
    /// # use gejmboj_cpu::bus::Bus;
    /// let mut bus = Bus::new();
    /// let value = 0xAB;
    ///
    /// bus.set(0, value);
    ///
    /// assert_eq!(value, bus.get(0));
    /// ```
    pub fn set(&mut self, location: u16, value: u8) {
        match location {
            ADDR_DIV => {
                // NOTE: Writing to DIV resets the counter
                self.counter = 0;
            }
            ADDR_TAC => {
                self.timer_enabled = value & MASK_TIMER_ENABLED > 0;
                self.clock = Clock::from(value);
                self.memory[location as usize] = value;
            }
            _ => {
                self.memory[location as usize] = value;
            }
        }
    }

    /// Gets a `u8` value from memory.
    ///
    /// ```
    /// # use gejmboj_cpu::bus::Bus;
    /// let mut bus = Bus::new();
    /// let value = 0xAB;
    ///
    /// bus.set(0, value);
    ///
    /// assert_eq!(value, bus.get(0));
    /// ```
    #[must_use]
    pub fn get(&self, location: u16) -> u8 {
        let data = self.memory[location as usize];

        match location {
            ADDR_IF => data | 0b1110_0000,
            ADDR_DIV => (self.counter >> 8) as u8,
            _ => data,
        }
    }

    /// Gets a `u16` value from memory.
    ///
    /// ```
    /// # use gejmboj_cpu::bus::Bus;
    /// let mut bus = Bus::new();
    /// let value = 0xABCD;
    ///
    /// bus.set_u16(42, value);
    ///
    /// assert_eq!(value, bus.get_u16(42));
    /// ```
    #[must_use]
    pub fn get_u16(&self, location: u16) -> u16 {
        let lo = self.get(location);
        let hi = self.get(location + 1);

        u16::from_le_bytes([lo, hi])
    }

    /// Sets a `u16` value in memory as LE bytes.
    ///
    /// ```
    /// # use gejmboj_cpu::bus::Bus;
    /// let mut bus = Bus::new();
    /// let value = 0xABCD;
    ///
    /// bus.set_u16(0, value);
    ///
    /// assert_eq!(value, bus.get_u16(0));
    /// ```
    pub fn set_u16(&mut self, location: u16, value: u16) {
        let [lo, hi] = value.to_le_bytes();

        self.set(location, lo);
        self.set(location + 1, hi);
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let columns = 16;
        let bytes_string: String = self
            .memory
            .iter()
            .map(|x| format!("{x:02x?}"))
            .collect::<Vec<String>>()
            .chunks(columns)
            .enumerate()
            .map(|(idx, bytes)| format!("{:03x?} | {} |", idx, bytes.join(" ").replace("00", "--")))
            .collect::<Vec<String>>()
            .join("\n");

        write!(
            f,
            "
       0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
    ,-------------------------------------------------,
{bytes_string}
    `-------------------------------------------------´"
        )
    }
}

#[cfg(test)]
impl Bus {
    #[must_use]
    pub fn with_memory(mut self, values: &[(u16, u8)]) -> Self {
        for (addr, val) in values {
            self.set(*addr, *val);
        }
        self
    }

    #[must_use]
    pub fn with_counter(mut self, value: u16) -> Self {
        self.counter = value;
        self
    }
}

// Semantically significant memory addresses
pub const ADDR_DIV: u16 = 0xFF04;
pub const ADDR_TIMA: u16 = 0xFF05;
pub const ADDR_TMA: u16 = 0xFF06;
pub const ADDR_TAC: u16 = 0xFF07;
pub const ADDR_IF: u16 = 0xFF0F;
pub const ADDR_NR10: u16 = 0xFF10;
pub const ADDR_NR11: u16 = 0xFF11;
pub const ADDR_NR12: u16 = 0xFF12;
pub const ADDR_NR14: u16 = 0xFF14;
pub const ADDR_NR21: u16 = 0xFF16;
pub const ADDR_NR22: u16 = 0xFF17;
pub const ADDR_NR24: u16 = 0xFF19;
pub const ADDR_NR30: u16 = 0xFF1A;
pub const ADDR_NR31: u16 = 0xFF1B;
pub const ADDR_NR32: u16 = 0xFF1C;
pub const ADDR_NR34: u16 = 0xFF1E;
pub const ADDR_NR41: u16 = 0xFF20;
pub const ADDR_NR42: u16 = 0xFF21;
pub const ADDR_NR43: u16 = 0xFF22;
pub const ADDR_NR44: u16 = 0xFF23;
pub const ADDR_NR50: u16 = 0xFF24;
pub const ADDR_NR51: u16 = 0xFF25;
pub const ADDR_NR52: u16 = 0xFF26;
pub const ADDR_LCDC: u16 = 0xFF40;
pub const ADDR_SCY: u16 = 0xFF42;
pub const ADDR_SCX: u16 = 0xFF43;
pub const ADDR_LYC: u16 = 0xFF45;
pub const ADDR_BGP: u16 = 0xFF47;
pub const ADDR_OBP0: u16 = 0xFF48;
pub const ADDR_OBP1: u16 = 0xFF49;
pub const ADDR_WY: u16 = 0xFF4A;
pub const ADDR_WX: u16 = 0xFF4B;
pub const ADDR_IE: u16 = 0xFFFF;

#[allow(non_snake_case, clippy::cast_possible_truncation)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr_IF_bits_5_to_7_always_return_1() {
        let mut b = Bus::new();
        b.set(ADDR_IF, 0b0001_1111);
        let value = b.get(ADDR_IF);

        assert_eq!(0b1111_1111, value, "Got {value:08b}");

        b.set(ADDR_IF + 1, 0b1010_1010);

        let value = b.get_u16(ADDR_IF);

        assert_eq!(0b1010_1010_1111_1111, value, "Got {value:08b}");

        let addr = ADDR_IF - 1;

        b.set(addr, 0b0101_0101);

        let value = b.get_u16(addr);

        assert_eq!(0b1111_1111_0101_0101, value, "Got {value:08b}");
    }

    #[test]
    fn writes_to_addr_DIV_resets_the_16bit_counter() {
        let div = ADDR_DIV;
        let mut b = Bus::new().with_counter(0xABCD);

        assert_eq!(0xAB, b.get(div));
        assert_eq!(0xABCD, b.counter);

        b.set(div, 0xFF);
        assert_eq!(0, b.get(div));
        assert_eq!(0, b.counter);

        let mut b = Bus::new().with_counter(0xABCD);

        b.set_u16(div - 1, 0xFFFF);
        assert_eq!(0x00FF, b.get_u16(div - 1));
        assert_eq!(0, b.counter);

        let mut b = Bus::new().with_counter(0xABCD);

        b.set_u16(div, 0xFFFF);
        assert_eq!(0xFF00, b.get_u16(div));
        assert_eq!(0, b.counter);
    }

    #[test]
    fn tick_advances_the_counter() {
        let mut b = Bus::new();
        assert_eq!(0, b.counter);

        let previous = b.counter;
        b.tick(MachineCycles::new(1));

        assert!(b.counter > previous);
    }

    #[test]
    fn tick_advances_the_counter_in_t_cycles() {
        let mut b = Bus::new();
        assert_eq!(0, b.counter);

        b.tick(MachineCycles::new(3));

        assert_eq!(3 * 4, b.counter);
    }

    #[test]
    fn div_is_updated_every_64_machine_cycles() {
        let mut b = Bus::new();
        assert_eq!(0, b.counter);
        assert_eq!(0, b.get(ADDR_DIV));

        for _ in 0..64 {
            b.tick(MachineCycles::new(1));
        }

        assert_eq!(1, b.get(ADDR_DIV));

        for _ in 0..64 {
            b.tick(MachineCycles::new(1));
        }

        assert_eq!(2, b.get(ADDR_DIV));
    }

    #[test]
    fn timer_test_t16_cadence() {
        let mut bus = Bus::new();
        bus.set(ADDR_TAC, MASK_TIMER_ENABLED | u8::from(Clock::T16));

        assert_eq!(0, bus.get(ADDR_TIMA));

        bus.tick(MachineCycles::new(4));

        assert_eq!(1, bus.get(ADDR_TIMA));

        bus.tick(MachineCycles::new(4));

        assert_eq!(2, bus.get(ADDR_TIMA));
    }

    #[test]
    fn timer_test_t64_cadence() {
        let mut bus = Bus::new();
        bus.set(ADDR_TAC, MASK_TIMER_ENABLED | u8::from(Clock::T64));

        assert_eq!(0, bus.get(ADDR_TIMA));

        bus.tick(MachineCycles::new(16));

        assert_eq!(1, bus.get(ADDR_TIMA));

        bus.tick(MachineCycles::new(16));

        assert_eq!(2, bus.get(ADDR_TIMA));
    }

    #[test]
    fn timer_test_t256_cadence() {
        let mut bus = Bus::new();
        bus.set(ADDR_TAC, MASK_TIMER_ENABLED | u8::from(Clock::T256));

        assert_eq!(0, bus.get(ADDR_TIMA));

        bus.tick(MachineCycles::new(64));

        assert_eq!(1, bus.get(ADDR_TIMA));

        bus.tick(MachineCycles::new(64));

        assert_eq!(2, bus.get(ADDR_TIMA));
    }

    #[test]
    fn timer_test_t1024_cadence() {
        let mut bus = Bus::new();
        bus.set(ADDR_TAC, MASK_TIMER_ENABLED | u8::from(Clock::T1024));

        assert_eq!(0, bus.get(ADDR_TIMA));

        bus.tick(MachineCycles::new(256));

        assert_eq!(1, bus.get(ADDR_TIMA));

        bus.tick(MachineCycles::new(256));

        assert_eq!(2, bus.get(ADDR_TIMA));
    }

    #[test]
    fn timer_test_disabled_timer_works() {
        let mut bus = Bus::new();
        bus.set(ADDR_TAC, u8::from(Clock::T16));

        assert_eq!(0, bus.get(ADDR_TIMA));

        bus.tick(MachineCycles::new(12));

        assert_eq!(0, bus.get(ADDR_TIMA));

        bus.set(ADDR_TAC, MASK_TIMER_ENABLED | u8::from(Clock::T16));

        bus.tick(MachineCycles::new(12));

        assert_eq!(3, bus.get(ADDR_TIMA));

        bus.set(ADDR_TAC, u8::from(Clock::T16));

        bus.tick(MachineCycles::new(12));

        assert_eq!(3, bus.get(ADDR_TIMA));
    }

    #[test]
    fn timer_overflow_reloads_the_timer_after_1_m_cycle() {
        let mut bus = Bus::new().with_memory(&[
            (ADDR_TIMA, 0xFF),
            (ADDR_TMA, 0xAB),
            (ADDR_TAC, MASK_TIMER_ENABLED | u8::from(Clock::T16)),
        ]);

        bus.tick(MachineCycles::new(4));

        assert_eq!(0, bus.get(ADDR_TIMA));
        assert_eq!(0, bus.get(ADDR_IF) & Interrupt::Timer.mask());

        for _ in 0..3 {
            bus.t_cycle_tick();
            assert_eq!(0, bus.get(ADDR_TIMA));
            assert_eq!(0, bus.get(ADDR_IF) & Interrupt::Timer.mask());
        }

        bus.t_cycle_tick();

        assert_eq!(0xAB, bus.get(ADDR_TIMA));
        assert!(bus.get(ADDR_IF) & Interrupt::Timer.mask() > 0);
    }
}
