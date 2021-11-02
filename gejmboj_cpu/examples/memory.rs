use gejmboj_cpu::memory::Memory;

fn main() {
    let mut memory = Memory::new();

    memory.set_u16(0xFFF0, 0xABCD);
    memory.set(0xFFFF, 0x11);

    println!("{}", memory);
    println!("0xFFF0: {}", memory.get_u16(0xFFF0));
    println!("0xFFFF: {}", memory.get(0xFFFF));
}
