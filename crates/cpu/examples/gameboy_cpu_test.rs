use std::{fs, path::PathBuf, str::FromStr};

use gejmboj_cpu::{
    cpu::CPU,
    memory::Memory,
    registers::{Registers, SingleRegister},
};

fn main() {
    let Some(path) = std::env::args().skip(1).next() else {
        panic!("Must specify a test file path!");
    };

    let path = PathBuf::from_str(&path).unwrap();

    if path.is_dir() {
        let mut files: Vec<PathBuf> = fs::read_dir(path).unwrap().fold(Vec::new(), |mut acc, e| {
            let path = e.unwrap().path();

            if path.extension().unwrap() == "json" {
                acc.push(path)
            }

            acc
        });
        files.sort();

        let mut cleared_tests = 0;
        let mut exit = false;
        for file_path in files {
            if exit {
                break;
            }
            println!("--------------------------------------------------------");
            println!("FILE: {}", file_path.to_str().unwrap());

            let bytes = std::fs::read(file_path).unwrap();

            let tests: Vec<CpuTest> = serde_json::from_slice(&bytes).unwrap();

            for (index, t) in tests.iter().enumerate() {
                print!("T{:03}: ", index);
                if run_cpu_test(t).is_ok() {
                    cleared_tests += 1;
                } else {
                    eprintln!("CLEARED TESTS: {cleared_tests}");
                    exit = true;
                    break;
                };
            }
        }
    } else {
        let bytes = std::fs::read(path).unwrap();

        let tests: Vec<CpuTest> = serde_json::from_slice(&bytes).unwrap();

        for (index, t) in tests.iter().enumerate() {
            print!("T{:03}: ", index);
            run_cpu_test(t).unwrap();
        }
    }
}

fn run_cpu_test(t: &CpuTest) -> Result<(), &'static str> {
    let mut memory = Memory::new();
    let mut registers = Registers::new();
    let mut cpu = CPU::new();

    registers.set_single(&SingleRegister::A, t.initial_state.a);
    registers.set_single(&SingleRegister::B, t.initial_state.b);
    registers.set_single(&SingleRegister::C, t.initial_state.c);
    registers.set_single(&SingleRegister::D, t.initial_state.d);
    registers.set_single(&SingleRegister::E, t.initial_state.e);
    registers.set_single(&SingleRegister::F, t.initial_state.f);
    registers.set_single(&SingleRegister::H, t.initial_state.h);
    registers.set_single(&SingleRegister::L, t.initial_state.l);
    registers.PC = t.initial_state.pc;
    registers.SP = t.initial_state.sp;

    for (addr, value) in &t.initial_state.ram {
        // NOTE: Initial PC points to the second RAM byte, not the first.
        //       I'm not yet sure why, but for the time being the workaround
        //       is to shift the RAM addresses by 1 byte during execution
        memory.set(*addr, *value);
    }

    let (_, instruction) = cpu
        .tick_gameboy_cpu_test(&mut registers, &mut memory)
        .unwrap();

    print!("{} -> {instruction:?}\n", t.name);
    if check_result(&registers, &memory, &t) {
        Ok(())
    } else {
        Err("failed")
    }
}

fn check_result(registers: &Registers, memory: &Memory, test: &CpuTest) -> bool {
    let addresses: Vec<u16> = test.final_state.ram.iter().map(|(addr, _)| *addr).collect();
    let state = TestSettings::extract(registers, memory, &addresses);
    let expected = &test.final_state;

    let mut success = true;

    if state.a != expected.a {
        eprintln!("A: {:02X} != {:02X}", expected.a, state.a);
        success = false;
    }
    if state.b != expected.b {
        eprintln!("B: {:02X} != {:02X}", expected.b, state.b);
        success = false;
    }
    if state.c != expected.c {
        eprintln!("C: {:02X} != {:02X}", expected.c, state.c);
        success = false;
    }
    if state.d != expected.d {
        eprintln!("D: {:02X} != {:02X}", expected.d, state.d);
        success = false;
    }
    if state.e != expected.e {
        eprintln!("E: {:02X} != {:02X}", expected.e, state.e);
        success = false;
    }
    if state.f != expected.f {
        eprintln!("F: {:08b} != {:08b}", expected.f, state.f);
        success = false;
    }
    if state.h != expected.h {
        eprintln!("H: {:02X} != {:02X}", expected.h, state.h);
        success = false;
    }
    if state.l != expected.l {
        eprintln!("L: {:02X} != {:02X}", expected.l, state.l);
        success = false;
    }
    if state.pc != expected.pc {
        eprintln!("PC: {:04X} != {:04X}", expected.pc, state.pc);
        success = false;
    }
    if state.sp != expected.sp {
        eprintln!("SP: {:04X} != {:04X}", expected.sp, state.sp);
        success = false;
    }
    if state.ram != expected.ram {
        eprintln!("RAM: [expected] {:?}", expected.ram);
        eprintln!("     [actual]   {:?}", state.ram);
        success = false;
    }

    if !success {
        eprintln!("DATA:\n{test:#?}");
    }

    success
}

#[derive(Debug, serde::Deserialize)]
struct CpuTest {
    pub name: String,
    #[serde(rename = "initial")]
    pub initial_state: TestSettings,
    #[serde(rename = "final")]
    pub final_state: TestSettings,
    #[allow(unused)]
    pub cycles: Vec<Option<(u16, u8, String)>>,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
struct TestSettings {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
    pub ram: Vec<(u16, u8)>,
}

impl TestSettings {
    pub fn extract(registers: &Registers, memory: &Memory, addresses: &[u16]) -> Self {
        let ram: Vec<(u16, u8)> = addresses
            .iter()
            // NOTE: RAM addresses were shifted, so we need to add 1 to the address
            //       to get the intended byte.
            .map(|addr| (*addr, memory.get(*addr)))
            .collect();
        Self {
            a: registers.get_single(&SingleRegister::A),
            b: registers.get_single(&SingleRegister::B),
            c: registers.get_single(&SingleRegister::C),
            d: registers.get_single(&SingleRegister::D),
            e: registers.get_single(&SingleRegister::E),
            f: registers.get_single(&SingleRegister::F),
            h: registers.get_single(&SingleRegister::H),
            l: registers.get_single(&SingleRegister::L),
            pc: registers.PC,
            sp: registers.SP,
            ram,
        }
    }
}
