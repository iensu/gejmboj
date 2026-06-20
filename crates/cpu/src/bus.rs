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

use crate::errors::CpuError;

pub struct Bus {
    /// Internal counter.
    counter: u16,
    memory: Vec<u8>,
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
        }
    }

    /// Reset memory to expected startup values.
    ///
    /// References:
    /// - <https://gbdev.io/pandocs/Power_Up_Sequence.html#hardware-registers>
    pub fn reset(&mut self) {
        self.memory[TIMA] = 0x00;
        self.memory[TMA] = 0x00;
        self.memory[TAC] = 0x00;
        self.memory[IF] = 0xE1;
        self.memory[NR10] = 0x80;
        self.memory[NR11] = 0xBF;
        self.memory[NR12] = 0xF3;
        self.memory[NR14] = 0xBF;
        self.memory[NR21] = 0x3F;
        self.memory[NR22] = 0x00;
        self.memory[NR24] = 0xBF;
        self.memory[NR30] = 0x7F;
        self.memory[NR31] = 0xFF;
        self.memory[NR32] = 0x9F;
        self.memory[NR34] = 0xBF;
        self.memory[NR41] = 0xFF;
        self.memory[NR42] = 0x00;
        self.memory[NR43] = 0x00;
        self.memory[NR44] = 0xBF;
        self.memory[NR50] = 0x77;
        self.memory[NR51] = 0xF3;
        self.memory[NR52] = 0xF1;
        self.memory[LCDC] = 0x91;
        self.memory[SCY] = 0x00;
        self.memory[SCX] = 0x00;
        self.memory[LYC] = 0x00;
        self.memory[BGP] = 0xFC;
        self.memory[OBP0] = 0xFF;
        self.memory[OBP1] = 0xFF;
        self.memory[WY] = 0x00;
        self.memory[WX] = 0x00;
        self.memory[IE] = 0x00;
    }

    /// Increase the internal counter by `machine_cycles` converted to T cycles.
    pub fn tick(&mut self, machine_cycles: u16) {
        let t_cycles = 4 * machine_cycles;

        for _ in 0..t_cycles {
            let value = self.counter.wrapping_add(1);
            self.counter = value;
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
        match location as usize {
            DIV => {
                // NOTE: Writing to DIV resets the counter
                self.counter = 0;
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

        match location as usize {
            IF => data | 0b1110_0000,
            DIV => (self.counter >> 8) as u8,
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
    pub fn with_memory(mut self, values: &[(u16, u8)]) -> Self {
        for (addr, val) in values {
            self.memory[*addr as usize] = *val;
        }
        self
    }

    pub fn with_counter(mut self, value: u16) -> Self {
        self.counter = value;
        self
    }
}

// Semantically significant memory addresses
const DIV: usize = 0xFF04;
const TIMA: usize = 0xFF05;
const TMA: usize = 0xFF06;
const TAC: usize = 0xFF07;
const IF: usize = 0xFF0F;
const NR10: usize = 0xFF10;
const NR11: usize = 0xFF11;
const NR12: usize = 0xFF12;
const NR14: usize = 0xFF14;
const NR21: usize = 0xFF16;
const NR22: usize = 0xFF17;
const NR24: usize = 0xFF19;
const NR30: usize = 0xFF1A;
const NR31: usize = 0xFF1B;
const NR32: usize = 0xFF1C;
const NR34: usize = 0xFF1E;
const NR41: usize = 0xFF20;
const NR42: usize = 0xFF21;
const NR43: usize = 0xFF22;
const NR44: usize = 0xFF23;
const NR50: usize = 0xFF24;
const NR51: usize = 0xFF25;
const NR52: usize = 0xFF26;
const LCDC: usize = 0xFF40;
const SCY: usize = 0xFF42;
const SCX: usize = 0xFF43;
const LYC: usize = 0xFF45;
const BGP: usize = 0xFF47;
const OBP0: usize = 0xFF48;
const OBP1: usize = 0xFF49;
const WY: usize = 0xFF4A;
const WX: usize = 0xFF4B;
const IE: usize = 0xFFFF;

#[allow(non_snake_case, clippy::cast_possible_truncation)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addr_IF_bits_5_to_7_always_return_1() {
        let mut b = Bus::new();
        b.set(IF as u16, 0b0001_1111);
        let value = b.get(IF as u16);

        assert_eq!(0b1111_1111, value, "Got {value:08b}");

        b.set((IF + 1) as u16, 0b1010_1010);

        let value = b.get_u16(IF as u16);

        assert_eq!(0b1010_1010_1111_1111, value, "Got {value:08b}");

        let addr = (IF - 1) as u16;

        b.set(addr, 0b0101_0101);

        let value = b.get_u16(addr);

        assert_eq!(0b1111_1111_0101_0101, value, "Got {value:08b}");
    }

    #[test]
    fn writes_to_addr_DIV_resets_the_16bit_counter() {
        let div = DIV as u16;
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
        b.tick(1);

        assert!(b.counter > previous);
    }

    #[test]
    fn tick_advances_the_counter_in_t_cycles() {
        let mut b = Bus::new();
        assert_eq!(0, b.counter);

        b.tick(3);

        assert_eq!(3 * 4, b.counter);
    }

    #[test]
    fn div_is_updated_every_64_machine_cycles() {
        let div = DIV as u16;
        let mut b = Bus::new();
        assert_eq!(0, b.counter);
        assert_eq!(0, b.get(div));

        for _ in 0..64 {
            b.tick(1);
        }

        assert_eq!(1, b.get(div));

        for _ in 0..64 {
            b.tick(1);
        }

        assert_eq!(2, b.get(div));
    }
}
