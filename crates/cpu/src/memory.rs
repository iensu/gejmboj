//! # Memory implementation
//!
//! Memory data is stored in Little-Endian fashion, which means that the least significant
//! byte is stored at a lower memory location than the most significant byte.
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

pub struct Memory {
    memory: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    #[must_use]
    pub fn new() -> Self {
        Self {
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
    /// # use gejmboj_cpu::memory::Memory;
    /// let mut memory = Memory::new();
    /// let value = 0xAB;
    ///
    /// memory.set(0, value);
    ///
    /// assert_eq!(value, memory.get(0));
    /// ```
    // TODO: usize -> u16
    pub fn set(&mut self, location: usize, value: u8) {
        self.memory[location] = value;
    }

    /// Gets a `u8` value from memory.
    ///
    /// ```
    /// # use gejmboj_cpu::memory::Memory;
    /// let mut memory = Memory::new();
    /// let value = 0xAB;
    ///
    /// memory.set(0, value);
    ///
    /// assert_eq!(value, memory.get(0));
    /// ```
    // TODO: usize -> u16
    #[must_use]
    pub fn get(&self, location: usize) -> u8 {
        self.memory[location]
    }

    /// Gets a `u16` value from memory.
    ///
    /// ```
    /// # use gejmboj_cpu::memory::Memory;
    /// let mut memory = Memory::new();
    /// let value = 0xABCD;
    ///
    /// memory.set_u16(42, value);
    ///
    /// assert_eq!(value, memory.get_u16(42));
    /// ```
    // TODO: usize -> u16
    #[must_use]
    pub fn get_u16(&self, location: usize) -> u16 {
        let lo = self.get(location);
        let hi = self.get(location + 1);

        u16::from_le_bytes([lo, hi])
    }

    /// Sets a `u16` value in memory as LE bytes.
    ///
    /// ```
    /// # use gejmboj_cpu::memory::Memory;
    /// let mut memory = Memory::new();
    /// let value = 0xABCD;
    ///
    /// memory.set_u16(0, value);
    ///
    /// assert_eq!(value, memory.get_u16(0));
    /// ```
    // TODO: usize -> u16
    pub fn set_u16(&mut self, location: usize, value: u16) {
        let [lo, hi] = value.to_le_bytes();

        self.set(location, lo);
        self.set(location + 1, hi);
    }
}

impl Display for Memory {
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

// Semantically significant memory addresses
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
