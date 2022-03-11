mod emulator;
mod disassembler;

use chip8emu::*;
use disassembler::*;
use emulator::Chip8;
use std::thread::{self, JoinHandle};
use std::{fs, io};

use sdl2;
use sdl2::pixels::Color;

#[allow(non_snake_case)]

fn main() -> Result<(), io::Error> {
    // Read instructions from binary
    let bytes = fs::read("./roms/ibmlogo.ch8")?;
    println!("{}", disassemble_all(&pair_bytes(&bytes)));

    let pixel_size = 10;

    let mut emu = Chip8::new().load_program(bytes); // Create emulator

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("CHIP-8", pixel_size*64, pixel_size*32)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap(); // To draw onto

    let WHITE = Color::RGB(0xAA, 0xB3, 0xB0); // Define black and white pixel color
    let BLACK = Color::RGB(0x29, 0x2C, 0x35);
    
    // Execute instructions
    loop {
        let instr = emu.fetch();
        match emu.exec(instr) {
            Err(_) => panic!("Failed to execute instruction 0x{:04X} !", instr),
            _ => ()
        }

        // PLAY BEEPING SOUND IF SOUND TIMER IS ABOVE 0
        
        // Update display if instruction is clear screen or draw
        match Chip8::decode_to_nibbles(instr) {
            (0xD, _, _, _) | (0x0, 0x0, 0xE, 0x0) => {
                display_chip8(emu.get_display(), &mut canvas, pixel_size, WHITE, BLACK).unwrap();
                canvas.present();
            },
            (_, _, _, _) => ()
        }
    }
}
