use ggez::graphics::{self, Color, DrawMode, Rect, Mesh};
use ggez::event::{self, ControlFlow};
use ggez::{self, Context, GameResult, GameError};
use glam::*;

const PIXEL_SIZE: u32 = 5; // Should add this as launch argument later

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
    for (y, row) in ch8display.iter().enumerate() {
        for (x, pix) in row.iter().enumerate() {
            let color = match pix {
                true => colwhite,
                false => colblack
            };

            // Create a mesh for the pixel
            let pix_mesh = create_mesh(ctx, color)?;
            // Add it onto the context
            graphics::draw(ctx, &pix_mesh, (Vec2::new(x as f32 * PIXEL_SIZE as f32, y as f32 * PIXEL_SIZE as f32),))?;
        }
    }
    Ok(())
}