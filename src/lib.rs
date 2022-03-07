use ggez::graphics::{self, Color, DrawMode, Rect, Mesh};
use ggez::event::{self, ControlFlow};
use ggez::{self, Context, GameResult, GameError};

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
pub fn create_mesh(ctx: &mut Context, x: f32, y: f32, color: Color) -> Result<Mesh, GameError> {
    let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(x, y, PIXEL_SIZE as f32, PIXEL_SIZE as f32), color)?;
    Ok(mesh)
}