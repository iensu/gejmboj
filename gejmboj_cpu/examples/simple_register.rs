use gejmboj_cpu::registers::{DoubleRegister, Registers, SingleRegister};

fn main() {
    let mut registers = Registers::new();

    registers.set_single(&SingleRegister::C, 0x99);
    registers.set_double(&DoubleRegister::AF, 0xFEDC);
    registers.set_double(&DoubleRegister::DE, 0x1144);
    registers.set_double(&DoubleRegister::HL, 0x3322);

    println!("{}", registers);
}
