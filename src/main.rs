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
    // Read instructions
    let bytes = fs::read("./roms/Airplane.ch8")?;
    println!("{}", disassemble_all(&disassembler::pair_bytes(&bytes)));

    let (mut ctx, events_loop) = build_context()?; // Create context
    let emu = Chip8::new().load_program(bytes); // Create emulator

    let WHITE: Color = Color::from_rgb_u32(0xAAB3B0); // Define black and white pixel color
    let BLACK = Color::from_rgb_u32(0x292C35);

    display_chip8(&mut ctx, emu.get_display(), WHITE, BLACK)?;

    Ok(())
}