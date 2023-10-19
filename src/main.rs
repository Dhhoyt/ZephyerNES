mod cpu_memory;
mod mos6502;

fn main() {
    let stuff: u8 = 5;
    let stuff = stuff.wrapping_add(255);
    println!("{}", stuff);
}
