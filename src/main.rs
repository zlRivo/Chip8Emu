mod emulator;
mod disassembler;

use chip8emu::*;
use disassembler::*;
use emulator::Chip8;
use std::thread::{self, JoinHandle};
use std::{fs, io};

use ncurses::*;

#[allow(non_snake_case)]

fn main() -> Result<(), io::Error> {
    // Read instructions from binary
    let bytes = fs::read("./roms/ibmlogo.ch8")?;
    println!("{}", disassemble_all(&pair_bytes(&bytes)));

    let mut emu = Chip8::new().load_program(bytes); // Create emulator
    initscr(); // Init ncurses
    start_color(); // Init terminal colors
    
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
                display_chip8(emu.get_display());
            },
            (_, _, _, _) => ()
        }
    }
    // endwin();
    // Ok(())
}
