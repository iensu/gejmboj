//! # Cycle units
//!
//! Newtypes for CPU timing quantities so they can't be confused with each other
//! or with plain `u16` values such as memory addresses.

use std::ops::{Add, AddAssign};

/// A number of machine cycles (M-cycles).
///
/// One M-cycle is 4 T-cycles on the Game Boy.
///
/// ```
/// # use gejmboj_cpu::cycles::MachineCycles;
/// let cycles = MachineCycles::new(2);
///
/// assert_eq!(2, cycles.value());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MachineCycles(u16);

impl MachineCycles {
    #[must_use]
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    /// Returns the raw machine cycle count.
    #[must_use]
    pub fn value(self) -> u16 {
        self.0
    }

    /// Get the corresponding amount of T-cycles
    #[must_use]
    pub fn to_t_cycles(&self) -> u16 {
        self.0 * 4
    }
}

impl Add for MachineCycles {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for MachineCycles {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
