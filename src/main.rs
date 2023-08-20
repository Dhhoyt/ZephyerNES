mod cpu_memory;
mod mos6502;

fn main() {
    let num = u16::from_le_bytes([0xAB, 0xCD]);
    println!("{:#06x}", num);
}
