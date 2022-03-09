mod emulator;
mod disassembler;

use chip8emu::*;
use disassembler::*;
use emulator::Chip8;
use std::fs;

use ggez::GameResult;
use ggez::graphics::Color;

#[allow(non_snake_case)]

fn main() -> GameResult {
    // Read instructions from binary
    let bytes = fs::read("./roms/test_opcode.ch8")?;
    println!("{}", disassemble_all(&pair_bytes(&bytes)));

    let (mut ctx, events_loop) = build_context()?; // Create context
    let mut emu = Chip8::new().load_program(bytes); // Create emulator

    let WHITE: Color = Color::from_rgb_u32(0xAAB3B0); // Define black and white pixel color
    let BLACK = Color::from_rgb_u32(0x292C35);

    // Execute instructions
    loop {
        let instr = emu.fetch();
        match emu.exec(instr) {
            Err(_) => panic!("Failed to execute instruction 0x{:04X} !", instr),
            _ => ()
        }
        // Update display if instruction is clear screen or draw
        match Chip8::decode_to_nibbles(instr) {
            (0xD, _, _, _) | (0x0, 0x0, 0xE, 0x0) => {
                display_chip8(&mut ctx, emu.get_display(), WHITE, BLACK)?; // Debug
            },
            (_, _, _, _) => ()
        }
    }
    // Ok(())
}
