use std::env;
use std::fs;

use cpu::bus::Bus;
use cpu::cpu::CPU;
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
    let mut bus = Bus::new();
    let mut cpu = CPU::new();

    registers.reset();
    bus.reset();
    bus.load(&rom_bytes)?;

    let mut instruction_count = 0;

    loop {
        instruction_count += 1;

        let prev_pc = registers.PC;

        let (location, instruction) = cpu.tick(&mut registers, &mut bus).inspect_err(|e| {
            eprintln!(
                "INSTR NO: {instruction_count}, PC: {:04X}, {e}",
                registers.PC
            );
        })?;

        debug!("Executed instruction [{instruction_count:08}] ({location:04X}) {instruction:?}");

        // Detect self-jump (jr $FE), PC remains unchanged across instruction ticks.
        if registers.PC == prev_pc {
            break;
        }
    }

    Ok(())
}
