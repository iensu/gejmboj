use std::{env, fs};

use cpu::{
    cpu::CPU,
    instructions::{self, Instruction},
    memory::Memory,
    registers::Registers,
};
use log::debug;

const SB: usize = 0xFF01;
const SC: usize = 0xFF02;

fn main() {
    env_logger::init();

    let Some(file_path) = env::args().nth(1) else {
        panic!("You must provide a path to a Game Boy rom file!");
    };
    let rom_bytes = fs::read(&file_path).unwrap();

    let mut registers = Registers::new();
    let mut memory = Memory::new();

    let out = fs::File::create("./blargg-test.log").unwrap();
    let mut cpu = CPU::new_with_trace(Box::new(out));

    registers.reset();
    memory.reset();
    memory.load(&rom_bytes).unwrap();
    memory.set(0xFF44, 0x90); // Initialize for blargg test

    let mut instruction_count = 0;

    let mut test_result: Vec<char> = Vec::new();

    loop {
        instruction_count += 1;

        let prev_pc = registers.PC;

        let (location, instruction) = cpu
            .tick(&mut registers, &mut memory)
            .inspect_err(|e| {
                eprintln!(
                    "INSTR NO: {instruction_count}, PC: {:04X}, {e}",
                    registers.PC
                );
            })
            .unwrap();

        debug!("Executed instruction [{instruction_count:08}] ({location:04X}) {instruction:?}");

        if let (c, 0x81) = (memory.get(SB), memory.get(SC)) {
            let c = c as char;
            if c != '\0' {
                test_result.push(c);
            }
            memory.set(SC, 0);
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

fn bytes_to_instructions(bytes: &Vec<u8>) -> Vec<Instruction> {
    let mut pc: u16 = 0;

    let mut result: Vec<Instruction> = Vec::new();

    let mut memory = Memory::new();
    memory.load(bytes).unwrap();

    while let Some(opcode) = bytes.get(pc as usize) {
        let instruction = instructions::decode(*opcode, pc, &memory).unwrap();

        pc += instruction.length();

        result.push(instruction);
    }

    result
}

fn to_byte_string(bytes: &Vec<u8>) -> String {
    bytes.iter().map(|b| format!("{b:02X} ")).collect()
}
