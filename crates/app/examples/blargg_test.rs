use std::{env, fmt::Write, fs};

use cpu::{
    bus::Bus,
    cpu::CPU,
    instructions::{self, Instruction},
    registers::Registers,
};
use log::trace;

const SB: u16 = 0xFF01;
const SC: u16 = 0xFF02;

fn main() {
    env_logger::init();

    let Some(file_path) = env::args().nth(1) else {
        panic!("You must provide a path to a Game Boy rom file!");
    };
    let rom_bytes = fs::read(&file_path).unwrap();

    let mut registers = Registers::new();
    let mut bus = Bus::new();

    let mut cpu = match std::env::var("GAMEBOY_DOCTOR").ok() {
        Some(val) if val == "1" => {
            let out = fs::File::create("./blargg-test.log").unwrap();
            CPU::new_with_trace(Box::new(out))
        }
        _ => CPU::new(),
    };

    registers.reset();
    bus.reset();
    bus.load(&rom_bytes).unwrap();
    bus.set(0xFF44, 0x90); // Initialize for blargg test

    let mut instruction_count = 0;

    let mut test_result: Vec<char> = Vec::new();

    loop {
        instruction_count += 1;

        let prev_pc = registers.PC;

        let (location, instruction) = cpu
            .tick(&mut registers, &mut bus)
            .inspect_err(|e| {
                eprintln!(
                    "INSTR NO: {instruction_count}, PC: {:04X}, {e}",
                    registers.PC
                );
            })
            .unwrap();

        trace!("Executed instruction [{instruction_count:08}] ({location:04X}) {instruction:?}");

        if let (c, 0x81) = (bus.get(SB), bus.get(SC)) {
            let c = c as char;
            if c != '\0' {
                test_result.push(c);
            }
            bus.set(SC, 0);
        }

        // Detect self-jump (jr $FE), PC remains unchanged across instruction ticks.
        if registers.PC == prev_pc {
            break;
        }
    }

    let test_result: String = test_result.into_iter().collect();

    print_test_result(&test_result);
}

fn print_test_result(message: &str) {
    if let Some((test_name, rest)) = message.split_once("\n\n") {
        println!("TEST: {test_name}");

        if let Some((instructions, result)) = rest.split_once('\n') {
            println!("INSTRUCTION STRING: {instructions}");

            let bytes: Vec<u8> = instructions
                .split(' ')
                .filter(|b| b.len() == 2)
                .map(|b| u8::from_str_radix(b, 16).unwrap())
                .collect();

            println!("INTERPRETED BYTES:  {}", to_byte_string(&bytes));

            let instrs = bytes_to_instructions(&bytes);

            println!("INSTRUCTIONS:");
            for (idx, instr) in instrs.iter().enumerate() {
                println!("    {idx:03}:  {instr:?}");
            }

            println!("RESULT: {result}");
        } else {
            println!("Failed to parse result");
        }
    } else {
        println!("Got message: {message}");
    }
}

fn bytes_to_instructions(bytes: &[u8]) -> Vec<Instruction> {
    let mut pc: u16 = 0;

    let mut result: Vec<Instruction> = Vec::new();

    let mut bus = Bus::new();
    bus.load(bytes).unwrap();

    while let Some(opcode) = bytes.get(pc as usize) {
        let instruction = instructions::decode(*opcode, pc, &bus).unwrap();

        pc += instruction.length();

        result.push(instruction);
    }

    result
}

fn to_byte_string(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut out, b| {
        let _ = write!(out, "{b:02X} ");
        out
    })
}
