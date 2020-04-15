use crate::components::grid_position::GridPosition;
use ggez::graphics::{Align, Text};
use ggez::{graphics, Context, GameResult};

pub fn draw_rect(pos: GridPosition, color: [f32; 4], ctx: &mut Context) -> GameResult {
    let rectangle =
        graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), pos.into(), color.into())?;
    graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    Ok(())
}

pub fn draw_text(pos: GridPosition, text: String, ctx: &mut Context) -> GameResult {
    let mut foo: Text = graphics::Text::default();
    foo.set_bounds(pos, Align::Center);
    foo.add(text);
    graphics::draw(ctx, &foo, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
    Ok(())
}
