use std::env;
use std::fs;

use cpu::cpu::CPU;
use cpu::memory::Memory;
use cpu::registers::Registers;
use error::AppResult;
use log::debug;

mod error;

fn main() -> AppResult<()> {
    env_logger::init();

    let Some(file_path) = env::args().nth(1) else {
        panic!("You must provide a path to a Game Boy rom file!");
    };
    let rom_bytes = fs::read(&file_path)?;

    debug!("Read {} bytes from {file_path}", rom_bytes.len());

    let mut registers = Registers::new();
    let mut memory = Memory::new();
    let mut cpu = CPU::new();

    registers.reset();
    memory.reset();

    loop {
        let prev_pc = registers.PC;

        let (location, instruction) = cpu.tick(&mut registers, &mut memory)?;

        debug!("Executed instruction [{location:04X}] {instruction:?}");

        // Detect self-jump (jr $FE), PC remains unchanged across instruction ticks.
        if registers.PC == prev_pc {
            break;
        }
    }

    Ok(())
}
