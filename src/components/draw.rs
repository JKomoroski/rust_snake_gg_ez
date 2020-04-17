use crate::components::grid_position::GridPosition;
use ggez::graphics::{Align, Text};
use ggez::{graphics, Context, GameResult};
use std::f32;

pub fn draw_rect(pos: GridPosition, color: [f32; 4], ctx: &mut Context) -> GameResult {
    let rectangle =
        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), pos.into(), color.into())?;
    graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    Ok(())
}

pub fn draw_text(pos: GridPosition, text: String, ctx: &mut Context) -> GameResult {
    let mut foo: Text = graphics::Text::default();
    let _ = foo.set_bounds(
        ggez::mint::Point2 {
            x: f32::INFINITY,
            y: f32::INFINITY,
        },
        Align::Center,
    );
    let _ = foo.add(text);
    graphics::draw(ctx, &foo, (pos,))?;
    Ok(())
}
