//! # Sharp SM83 register
//!
//! ## 16-bit special purpose registers
//!
//! * `PC` (Program counter)
//! * `SP` (Stack pointer)
//!
//! ## General purpose registers
//!
//! ```asciidoc
//! ,---.---.
//! | A | F |
//! |---|---|
//! | B | C |
//! |---|---|
//! | D | E |
//! |---|---|
//! | H | L |
//! `---´---´
//! ```
//!
//! Each register is 8-bit, but can be combined for 16-bit values according to the table rows above.
//!
//! ## Flag register F
//!
//! Register F is the Flag register with the following flag mapping:
//!
//! ```asciidoc
//! ,---.---.---.---.---.---.---.---.
//! | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 | Bit
//! |---|---|---|---|---|---|---|---|
//! | Z | N | H | C | 0 | 0 | 0 | 0 | Flag
//! `---´---´---´---´---´---´---´---´
//! ```
//!
//! Where bit 0-3 are grounded to `0` and can't be overwritten and `C` is for carry, `H` for half-carry, `N`
//! for negative and `Z` for zero.
//!
//! ## Stack pointer register (SP)
//!
//! The stack pointer register is initialized to `0xFFFE` and grows top-down, which means it is decremented.

use std::fmt::Display;

#[allow(non_snake_case)]
pub struct Registers {
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    F: u8,
    H: u8,
    L: u8,

    pub PC: u16,
    pub SP: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            F: 0,
            H: 0,
            L: 0,

            PC: 0,
            SP: 0xFFFE,
        }
    }

    /// Sets the value of a `SingleRegister`.
    ///
    /// ## Examples
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let mut registers = Registers::new();
    ///
    /// registers.set_single(&SingleRegister::A, 42);
    ///
    /// assert_eq!(42, registers.get_single(&SingleRegister::A));
    /// ```
    ///
    /// ## Special cases
    ///
    /// Lowest nibble of register `F` is always `0` and can't be overwritten.
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # let mut registers = Registers::new();
    /// registers.set_single(&SingleRegister::F, 0xFF);
    ///
    /// assert_eq!(0xF0, registers.get_single(&SingleRegister::F));
    /// ```
    pub fn set_single(&mut self, r: &SingleRegister, value: u8) {
        match r {
            SingleRegister::A => {
                self.A = value;
            }
            SingleRegister::B => {
                self.B = value;
            }
            SingleRegister::C => {
                self.C = value;
            }
            SingleRegister::D => {
                self.D = value;
            }
            SingleRegister::E => {
                self.E = value;
            }
            SingleRegister::F => {
                self.F = value & 0xF0;
            }
            SingleRegister::H => {
                self.H = value;
            }
            SingleRegister::L => {
                self.L = value;
            }
        }
    }

    /// Gets the current value of a `SingleRegister`.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let registers = Registers::new();
    ///
    /// assert_eq!(0, registers.get_single(&SingleRegister::A));
    /// ```
    pub fn get_single(&self, r: &SingleRegister) -> u8 {
        match r {
            SingleRegister::A => self.A,
            SingleRegister::B => self.B,
            SingleRegister::C => self.C,
            SingleRegister::D => self.D,
            SingleRegister::E => self.E,
            SingleRegister::F => self.F,
            SingleRegister::H => self.H,
            SingleRegister::L => self.L,
        }
    }

    /// Gets value from a double 16-bit register
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let mut registers = Registers::new();
    /// registers.set_single(&SingleRegister::B, 0xAB);
    /// registers.set_single(&SingleRegister::C, 0xCD);
    ///
    /// assert_eq!(0xABCD, registers.get_double(&DoubleRegister::BC));
    /// ````
    pub fn get_double(&self, r: &DoubleRegister) -> u16 {
        match r {
            DoubleRegister::AF => u16::from_be_bytes([self.A, self.F]),
            DoubleRegister::BC => u16::from_be_bytes([self.B, self.C]),
            DoubleRegister::DE => u16::from_be_bytes([self.D, self.E]),
            DoubleRegister::HL => u16::from_be_bytes([self.H, self.L]),
            DoubleRegister::SP => self.SP,
        }
    }

    /// Sets value of a double 16-bit register
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let mut registers = Registers::new();
    /// registers.set_double(&DoubleRegister::BC, 0xAABB);
    ///
    /// assert_eq!(0xAABB, registers.get_double(&DoubleRegister::BC));
    /// ```
    ///
    /// ## Special cases
    ///
    /// Lowest nibble of `DoubleRegister::AF` is always `0` and cannot be overwritten.
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let mut registers = Registers::new();
    /// registers.set_double(&DoubleRegister::AF, 0xABCD);
    ///
    /// assert_eq!(0xABC0, registers.get_double(&DoubleRegister::AF));
    /// ```
    pub fn set_double(&mut self, r: &DoubleRegister, value: u16) {
        let [hi, lo] = value.to_be_bytes();
        match r {
            DoubleRegister::AF => {
                self.A = hi;
                self.F = lo & 0xF0;
            }
            DoubleRegister::BC => {
                self.B = hi;
                self.C = lo;
            }
            DoubleRegister::DE => {
                self.D = hi;
                self.E = lo;
            }
            DoubleRegister::HL => {
                self.H = hi;
                self.L = lo;
            }
            DoubleRegister::SP => {
                self.SP = u16::from_be_bytes([hi, lo]);
            }
        }
    }

    /// Returns `true` if the carry flag is set.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let mut registers = Registers::new();
    ///
    /// registers.set_single(&SingleRegister::F, 0b0000_0000);
    /// assert_eq!(false, registers.is_carry());
    ///
    /// registers.set_single(&SingleRegister::F, 0b0001_0000);
    /// assert_eq!(true, registers.is_carry());
    /// ```
    pub fn is_carry(&self) -> bool {
        self.F & 0b0001_0000 > 0
    }

    /// Returns `true` if the half carry flag is set.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let mut registers = Registers::new();
    ///
    /// registers.set_single(&SingleRegister::F, 0b0000_0000);
    /// assert_eq!(false, registers.is_half_carry());
    ///
    /// registers.set_single(&SingleRegister::F, 0b0010_0000);
    /// assert_eq!(true, registers.is_half_carry());
    /// ```
    pub fn is_half_carry(&self) -> bool {
        self.F & 0b0010_0000 > 0
    }

    /// Returns `true` if the negative flag is set.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let mut registers = Registers::new();
    ///
    /// registers.set_single(&SingleRegister::F, 0b0000_0000);
    /// assert_eq!(false, registers.is_negative());
    ///
    /// registers.set_single(&SingleRegister::F, 0b0100_0000);
    /// assert_eq!(true, registers.is_negative());
    /// ```
    pub fn is_negative(&self) -> bool {
        self.F & 0b0100_0000 > 0
    }

    /// Returns `true` if the zero flag is set.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let mut registers = Registers::new();
    ///
    /// registers.set_single(&SingleRegister::F, 0b0000_0000);
    /// assert_eq!(false, registers.is_zero());
    ///
    /// registers.set_single(&SingleRegister::F, 0b1000_0000);
    /// assert_eq!(true, registers.is_zero());
    /// ```
    pub fn is_zero(&self) -> bool {
        self.F & 0b1000_0000 > 0
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
PC:{:04x?} SP:{:04x?}

 A:{:02x?} {:02x?}:F
 B:{:02x?} {:02x?}:C
 D:{:02x?} {:02x?}:E
 H:{:02x?} {:02x?}:L
",
            self.PC, self.SP, self.A, self.F, self.B, self.C, self.D, self.E, self.H, self.L
        )
    }
}

/// Represents an 8-bit general purpose register.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SingleRegister {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

impl From<(u8, u8, u8)> for SingleRegister {
    fn from(x: (u8, u8, u8)) -> Self {
        match (x.0 > 0, x.1 > 0, x.2 > 0) {
            (false, false, false) => SingleRegister::B,
            (false, false, true) => SingleRegister::C,
            (false, true, false) => SingleRegister::D,
            (false, true, true) => SingleRegister::E,
            (true, false, false) => SingleRegister::H,
            (true, false, true) => SingleRegister::L,
            (true, true, false) => SingleRegister::F,
            (true, true, true) => SingleRegister::A,
        }
    }
}

/// Represents a 16-bit general purpose register.
#[derive(Debug, PartialEq)]
pub enum DoubleRegister {
    AF,
    BC,
    DE,
    HL,
    SP,
}

impl From<(u8, u8)> for DoubleRegister {
    fn from(x: (u8, u8)) -> Self {
        match (x.0 > 0, x.1 > 0) {
            (false, false) => DoubleRegister::BC,
            (false, true) => DoubleRegister::DE,
            (true, false) => DoubleRegister::HL,
            (true, true) => DoubleRegister::SP,
        }
    }
}
