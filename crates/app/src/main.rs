use std::env;
use std::fs;

use cpu::cpu::CPU;
use cpu::memory::Memory;
use cpu::registers::Registers;
use error::AppResult;
use log::debug;

mod error;

const SB: usize = 0xFF01;
const SC: usize = 0xFF02;

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
    memory.load(&rom_bytes)?;

    let mut instruction_count = 0;

    let mut message: Vec<char> = Vec::new();

    loop {
        instruction_count += 1;

        let prev_pc = registers.PC;

        let (location, instruction) = cpu.tick(&mut registers, &mut memory).inspect_err(|e| {
            eprintln!(
                "INSTR NO: {instruction_count}, PC: {:04X}, {e}",
                registers.PC
            );
        })?;

        debug!("Executed instruction [{instruction_count:08}] ({location:04X}) {instruction:?}");

        if let (c, 0x81) = (memory.get(SB), memory.get(SC)) {
            let c = c as char;
            debug!("Got char: {c} [binary: {:08b}]", c as u8);
            if c != '\0' {
                message.push(c);
            }
            memory.set(SC, 0);
        }

        // Detect self-jump (jr $FE), PC remains unchanged across instruction ticks.
        if registers.PC == prev_pc {
            break;
        }

        if instruction_count > 100_000 {
            println!("Max instruction count exceeded, breaking...");
            break;
        }
    }

    let message: String = message.into_iter().collect();

    println!("Got message: {message}");

    Ok(())
}
