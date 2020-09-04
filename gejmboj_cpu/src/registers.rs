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

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,

    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }

    /// Sets the value of a `SingleRegister`.
    ///
    /// ## Examples
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let mut registers = Registers::new();
    ///
    /// registers.set_single(SingleRegister::A, 42);
    ///
    /// assert_eq!(42, registers.get_single(SingleRegister::A));
    /// ```
    ///
    /// ## Special cases
    ///
    /// Lowest nibble of register `F` is always `0` and can't be overwritten.
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// # let mut registers = Registers::new();
    /// registers.set_single(SingleRegister::F, 0xFF);
    ///
    /// assert_eq!(0xF0, registers.get_single(SingleRegister::F));
    /// ```
    pub fn set_single(&mut self, r: SingleRegister, value: u8) {
        match r {
            SingleRegister::A => {
                self.a = value;
            }
            SingleRegister::B => {
                self.b = value;
            }
            SingleRegister::C => {
                self.c = value;
            }
            SingleRegister::D => {
                self.d = value;
            }
            SingleRegister::E => {
                self.e = value;
            }
            SingleRegister::F => {
                self.f = value & 0xF0;
            }
            SingleRegister::H => {
                self.h = value;
            }
            SingleRegister::L => {
                self.l = value;
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
    /// assert_eq!(0, registers.get_single(SingleRegister::A));
    /// ```
    pub fn get_single(&self, r: SingleRegister) -> u8 {
        match r {
            SingleRegister::A => self.a,
            SingleRegister::B => self.b,
            SingleRegister::C => self.c,
            SingleRegister::D => self.d,
            SingleRegister::E => self.e,
            SingleRegister::F => self.f,
            SingleRegister::H => self.h,
            SingleRegister::L => self.l,
        }
    }

    /// Gets value from a double 16-bit register
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let mut registers = Registers::new();
    /// registers.set_single(SingleRegister::B, 0xAB);
    /// registers.set_single(SingleRegister::C, 0xCD);
    ///
    /// assert_eq!(0xABCD, registers.get_double(DoubleRegister::BC));
    /// ````
    pub fn get_double(&self, r: DoubleRegister) -> u16 {
        match r {
            DoubleRegister::AF => u16::from_be_bytes([self.a, self.f]),
            DoubleRegister::BC => u16::from_be_bytes([self.b, self.c]),
            DoubleRegister::DE => u16::from_be_bytes([self.d, self.e]),
            DoubleRegister::HL => u16::from_be_bytes([self.h, self.l]),
        }
    }

    /// Sets value of a double 16-bit register
    ///
    /// ## Examples
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let mut registers = Registers::new();
    /// registers.set_double(DoubleRegister::BC, 0xAABB);
    ///
    /// assert_eq!(0xAABB, registers.get_double(DoubleRegister::BC));
    /// ```
    ///
    /// ## Special cases
    ///
    /// Lowest nibble of `DoubleRegister::AF` is always `0` and cannot be overwritten.
    ///
    /// ```
    /// # use gejmboj_cpu::registers::*;
    /// let mut registers = Registers::new();
    /// registers.set_double(DoubleRegister::AF, 0xABCD);
    ///
    /// assert_eq!(0xABC0, registers.get_double(DoubleRegister::AF));
    /// ```
    pub fn set_double(&mut self, r: DoubleRegister, value: u16) {
        let [hi, lo] = value.to_be_bytes();
        match r {
            DoubleRegister::AF => {
                self.a = hi;
                self.f = lo & 0xF0;
            }
            DoubleRegister::BC => {
                self.b = hi;
                self.c = lo;
            }
            DoubleRegister::DE => {
                self.d = hi;
                self.e = lo;
            }
            DoubleRegister::HL => {
                self.h = hi;
                self.l = lo;
            }
        }
    }
}

/// Represents an 8-bit general purpose register.
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

/// Represents a 16-bit general purpose register.
pub enum DoubleRegister {
    AF,
    BC,
    DE,
    HL,
}
