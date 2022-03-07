mod emulator;
mod disassembler;

use disassembler::*;

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read rom as vector of bytes
    let bytes = fs::read("./roms/Airplane.ch8")?;
    let mut new_bytes = Vec::<u16>::new();
    let mut index = 0;
    while index <= bytes.len() {
        // Check if there is a pair
        if let (Some(&b1), Some(&b2)) = (bytes.get(index), bytes.get(index + 1)) {
            // Convert pair to u16
            new_bytes.push(((b1 as u16) << 8) as u16 | b2 as u16);
        } else if let Some(b1) = bytes.get(index) {
            // Convert byte to u16
            new_bytes.push(((*b1 as u16) << 8) as u16);
        }
        index += 2;
    }
    println!("{}", disassemble_all(new_bytes));
    
    Ok(())
}