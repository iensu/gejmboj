//! Shared helpers for unit tests within this crate.

use crate::bus::Bus;
use crate::cpu::CpuFlags;
use crate::registers::Registers;

/// Returns a fresh set of registers, memory, and CPU flags for use in tests.
pub fn setup() -> (Registers, Bus, CpuFlags) {
    let r = Registers::new();
    let b = Bus::new();
    let c = CpuFlags::new();

    (r, b, c)
}
