use ggez::graphics::{self, Color, DrawMode, Rect, Mesh};
use ggez::event::{self, ControlFlow};
use ggez::{self, Context, GameResult, GameError};
use glam::*;

const PIXEL_SIZE: u32 = 10; // Should add this as launch argument later

// Builds the window context
pub fn build_context() -> GameResult<(Context, ggez::event::EventLoop<()>)> {
    let c = ggez::ContextBuilder::new("chip-8 emu", "KilyLC")
        .window_setup(ggez::conf::WindowSetup::default().title("CHIP-8 Emulator"))
        .window_mode(ggez::conf::WindowMode::default().dimensions((64*PIXEL_SIZE) as f32, (32*PIXEL_SIZE) as f32))
        .build()?;
    Ok(c)
}

// Creates a mesh on a context from x, y and color
pub fn create_mesh(ctx: &mut Context, color: Color) -> Result<Mesh, GameError> {
    let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0.0, 0.0, PIXEL_SIZE as f32, PIXEL_SIZE as f32), color)?;
    Ok(mesh)
}

// Display a CHIP-8 display into a context
pub fn display_chip8(ctx: &mut Context, ch8display: [[bool; 64]; 32], colwhite: Color, colblack: Color) -> Result<(), GameError> {
    // Create color meshes
    let mesh_white = create_mesh(ctx, colwhite)?;
    let mesh_black = create_mesh(ctx, colblack)?;
    for (y, row) in ch8display.iter().enumerate() {
        for (x, pix) in row.iter().enumerate() {
            // Create a mesh for the pixel
            let pix_mesh = if *pix { mesh_white.clone() } else { mesh_black.clone() };
            // Add it onto the context
            graphics::draw(ctx, &pix_mesh, (Vec2::new(x as f32 * PIXEL_SIZE as f32, y as f32 * PIXEL_SIZE as f32),))?;
        }
    }
    graphics::present(ctx)?; // Update context
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
