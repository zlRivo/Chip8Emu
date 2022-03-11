pub mod emulator;
pub mod disassembler;

use sdl2::{pixels::Color, render::Canvas};
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::rect::*;

use emulator::Chip8;

/// Displays a CHIP-8 onto a canvas (can overflow if pixel_size is too big)
pub fn display_chip8(ch8display: [[bool; 64]; 32], canvas: &mut Canvas<Window>, pixel_size: u32, white: Color, black: Color) -> Result<(), String>{
    for (y, row) in ch8display.iter().enumerate() {
        for (x, pix) in row.iter().enumerate() {
            canvas.set_draw_color({ // Set draw color
                if *pix { white } else { black }
            });
            canvas.fill_rect(Rect::new( // Draw the pixel
                x as i32 * pixel_size as i32,
                y as i32 * pixel_size as i32,
                pixel_size,
                pixel_size
            ))?;
        }
    }
    Ok(())
}

/// Updates CHIP-8 keystates from an SDL EventPump
pub fn update_keys(ch8: &mut Chip8, events: &mut EventPump) -> Result<(), ()> {
    for event in events.poll_iter() {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => { ch8.update_key(0, true)?; },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => { ch8.update_key(1, true)?; },
            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => { ch8.update_key(2, true)?; },
            Event::KeyDown { keycode: Some(Keycode::Num4), .. } => { ch8.update_key(3, true)?; },
            Event::KeyDown { keycode: Some(Keycode::Q), .. } => { ch8.update_key(4, true)?; },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => { ch8.update_key(5, true)?; },
            Event::KeyDown { keycode: Some(Keycode::E), .. } => { ch8.update_key(6, true)?; },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => { ch8.update_key(7, true)?; },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => { ch8.update_key(8, true)?; },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => { ch8.update_key(9, true)?; },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => { ch8.update_key(0xA, true)?; },
            Event::KeyDown { keycode: Some(Keycode::F), .. } => { ch8.update_key(0xB, true)?; },
            Event::KeyDown { keycode: Some(Keycode::Y), .. } => { ch8.update_key(0xC, true)?; },
            Event::KeyDown { keycode: Some(Keycode::X), .. } => { ch8.update_key(0xD, true)?; },
            Event::KeyDown { keycode: Some(Keycode::C), .. } => { ch8.update_key(0xE, true)?; },
            Event::KeyDown { keycode: Some(Keycode::V), .. } => { ch8.update_key(0xF, true)?; },

            
            Event::KeyUp { keycode: Some(Keycode::Num1), .. } => { ch8.update_key(0, false)?; },
            Event::KeyUp { keycode: Some(Keycode::Num2), .. } => { ch8.update_key(1, false)?; },
            Event::KeyUp { keycode: Some(Keycode::Num3), .. } => { ch8.update_key(2, false)?; },
            Event::KeyUp { keycode: Some(Keycode::Num4), .. } => { ch8.update_key(3, false)?; },
            Event::KeyUp { keycode: Some(Keycode::Q), .. } => { ch8.update_key(4, false)?; },
            Event::KeyUp { keycode: Some(Keycode::W), .. } => { ch8.update_key(5, false)?; },
            Event::KeyUp { keycode: Some(Keycode::E), .. } => { ch8.update_key(6, false)?; },
            Event::KeyUp { keycode: Some(Keycode::R), .. } => { ch8.update_key(7, false)?; },
            Event::KeyUp { keycode: Some(Keycode::A), .. } => { ch8.update_key(8, false)?; },
            Event::KeyUp { keycode: Some(Keycode::S), .. } => { ch8.update_key(9, false)?; },
            Event::KeyUp { keycode: Some(Keycode::D), .. } => { ch8.update_key(0xA, false)?; },
            Event::KeyUp { keycode: Some(Keycode::F), .. } => { ch8.update_key(0xB, false)?; },
            Event::KeyUp { keycode: Some(Keycode::Y), .. } => { ch8.update_key(0xC, false)?; },
            Event::KeyUp { keycode: Some(Keycode::X), .. } => { ch8.update_key(0xD, false)?; },
            Event::KeyUp { keycode: Some(Keycode::C), .. } => { ch8.update_key(0xE, false)?; },
            Event::KeyUp { keycode: Some(Keycode::V), .. } => { ch8.update_key(0xF, false)?; },
            _ => ()
        }
    }
    Ok(())
}

pub fn get_default_font() -> Vec<u8> {
    [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ].to_vec()
}
