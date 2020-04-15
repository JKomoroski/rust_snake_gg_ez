use crate::components::draw::draw_rect;
use crate::components::grid_position::GridPosition;
use crate::SNAKE_CONFIG;
use ggez::{Context, GameResult};

/// This is again an abstraction over a `GridPosition` that represents
/// a piece of food the snake can eat. It can draw itself.
pub struct Food {
    pub pos: GridPosition,
}

impl Food {
    pub fn new(pos: GridPosition) -> Self {
        Food { pos }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        draw_rect(self.pos, SNAKE_CONFIG.food, ctx)?;
        Ok(())
    }
}

/// Here we define an enum of the possible things that the snake could have "eaten"
/// during an update of the game. It could have either eaten a piece of `Food`, or
/// it could have eaten `Itself` if the head ran into its body.
#[derive(Clone, Copy, Debug)]
pub enum Ate {
    Itself,
    Food,
}
