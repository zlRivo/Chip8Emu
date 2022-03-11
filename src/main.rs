use chip8emu::*;
use disassembler::*;
use emulator::Chip8;
use std::fs;

use sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::thread;
use std::time::Duration;

use std::path::Path;

use clap;
use clap::Parser;

/// CHIP-8 Emulator running with SDL2
#[derive(Parser, Debug)]
struct Args {
    /// Path to the target rom
    #[clap(short, long)]
    rom: String,
}

#[allow(non_snake_case)]

fn main() -> Result<(), ()> {
    let args = Args::parse();
    
    // Check if file exists
    if !Path::new(&args.rom).is_file() {
        println!("Provided path is not a file !");
        return Err(())
    }
     
    // Read instructions from rom
    let bytes = fs::read(args.rom).unwrap();

    let pixel_size = 10;

    let mut emu = Chip8::new().load_program(bytes); // Create emulator

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("CHIP-8", pixel_size*64, pixel_size*32)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap(); // Event pump
 
    let mut canvas = window.into_canvas().build().unwrap(); // To draw onto

    let WHITE = Color::RGB(0xAA, 0xB3, 0xB0); // Define black and white pixel color
    let BLACK = Color::RGB(0x29, 0x2C, 0x35);

    let mut cycles = 0; // Count number of cycles to decrement timers
    
    // Execute instructions
    'main: loop {
        // Fetch
        let instr = emu.fetch();
        println!("0x{:04X} -> {}", instr, disassemble(instr));
        
        // Update key states if key instruction
        update_keys(&mut emu, &mut event_pump).expect("Failed to update key registers");
        /* // Cause input lag
        match Chip8::decode_to_nibbles(instr) {
            (0xE, _, 0x9, 0xE) |
            (0xE, _, 0xA, 0x1) |
            (0xF, _, 0x0, 0xA) => {  }
            (_, _, _, _) => ()
        }
        */
        
        // Execute
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

        // Poll SDL Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                _ => {}
            }
        }
        cycles += 1;
        if cycles >= emu.get_freq() / 60 { // 60Hz Timers
            emu.decr_timers();
            cycles = 0; // Reset cpu cycles
        }
        thread::sleep(Duration::new(0, {
            (1 as f64 / emu.get_freq() as f64 * 1_000_000_000f64).ceil() as u32
            // Take 1 second, divide it by cpu frequency to get the time to wait in seconds,
            // multiply by 10^9 to convert it to nanosecond, ceil it, and finally cast it to u32.
            // (Casts may cause runtime errors if too big)
        }));
    }
    Ok(())
}
