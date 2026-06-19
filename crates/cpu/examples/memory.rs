use gejmboj_cpu::bus::Bus;

fn main() {
    let mut bus = Bus::new();

    bus.set_u16(0xFFF0, 0xABCD);
    bus.set(0xFFFF, 0x11);

    println!("{bus}");
    println!("0xFFF0: {}", bus.get_u16(0xFFF0));
    println!("0xFFFF: {}", bus.get(0xFFFF));
}
