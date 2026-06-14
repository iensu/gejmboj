//! Shared helpers for unit tests within this crate.

use crate::cpu::CpuFlags;
use crate::memory::Memory;
use crate::registers::Registers;

/// Returns a fresh set of registers, memory, and CPU flags for use in tests.
pub fn setup() -> (Registers, Memory, CpuFlags) {
    let r = Registers::new();
    let m = Memory::new();
    let c = CpuFlags::new();

    (r, m, c)
}
