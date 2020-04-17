use crate::components::direction::*;
use crate::components::draw::draw_rect;
use crate::components::food::*;
use crate::components::grid_position::GridPosition;
use crate::SNAKE_CONFIG;
use ggez::{Context, GameResult};
use std::collections::LinkedList;

/// Now we make a struct that contains all the information needed to describe the
/// state of the Snake itself.
pub struct Snake {
    /// First we have the head of the snake, which is a single `Segment`.
    pub head: Segment,
    /// Then we have the current direction the snake is moving. This is
    /// the direction it will move when `update` is called on it.
    pub dir: Direction,
    /// Next we have the body, which we choose to represent as a `LinkedList`
    /// of `Segment`s.
    pub body: LinkedList<Segment>,
    /// Now we have a property that represents the result of the last update
    /// that was performed. The snake could have eaten nothing (None), Food (Some(Ate::Food)),
    /// or Itself (Some(Ate::Itself))
    pub ate: Option<Ate>,
    /// Finally we store the direction that the snake was traveling the last
    /// time that `update` was called, which we will use to determine valid
    /// directions that it could move the next time update is called.
    pub last_update_dir: Direction,
    /// Store the direction that will be used in the `update` after the next `update`
    /// This is needed so a user can press two directions (eg. left then up)
    /// before one `update` has happened. It sort of queues up key press input
    pub buffered_dir: Option<Direction>,
}

impl Snake {
    pub fn new(pos: GridPosition) -> Self {
        let mut body = LinkedList::new();
        // Our snake will initially have a head and one body segment,
        // and will be moving to the right.
        body.push_back(Segment::new((pos.x - 1, pos.y).into()));
        Snake {
            head: Segment::new(pos),
            dir: Direction::Right,
            last_update_dir: Direction::Right,
            body: body,
            ate: None,
            buffered_dir: None,
        }
    }

    /// A helper function that determines whether
    /// the snake eats a given piece of Food based
    /// on its current position
    pub fn eats(&self, food: &Food) -> bool {
        if self.head.pos == food.pos {
            true
        } else {
            false
        }
    }

    /// A helper function that determines whether
    /// the snake eats itself based on its current position
    pub fn eats_self(&self) -> bool {
        self.body.iter().any(|&seg| seg.pos == self.head.pos)
    }

    /// The main update function for our snake which gets called every time
    /// we want to update the game state.
    pub fn update(&mut self, food: &Food) {
        // If `last_update_dir` has already been updated to be the same as `dir`
        // and we have a `next_dir`, then set `dir` to `next_dir` and unset `next_dir`

        if self.last_update_dir == self.dir && self.buffered_dir.is_some() {
            self.dir = self.buffered_dir.unwrap();
            self.buffered_dir = None;
        }
        // First we get a new head position by using our `new_from_move` helper
        // function from earlier. We move our head in the direction we are currently
        // heading.
        let new_head_pos = GridPosition::new_from_move(self.head.pos, self.dir);
        // Next we create a new segment will be our new head segment using the
        // new position we just made.
        let new_head = Segment::new(new_head_pos);
        // Then we push our current head Segment onto the front of our body
        self.body.push_front(self.head);
        // And finally make our actual head the new Segment we created. This has
        // effectively moved the snake in the current direction.
        self.head = new_head;
        // Next we check whether the snake eats itself or some food, and if so,
        // we set our `ate` member to reflect that state.
        if self.eats_self() {
            self.ate = Some(Ate::Itself);
        } else if self.eats(food) {
            self.ate = Some(Ate::Food);
        } else {
            self.ate = None
        }
        // If we didn't eat anything this turn, we remove the last segment from our body,
        // which gives the illusion that the snake is moving. In reality, all the segments stay
        // stationary, we just add a segment to the front and remove one from the back. If we eat
        // a piece of food, then we leave the last segment so that we extend our body by one.
        if let None = self.ate {
            self.body.pop_back();
        }
        // And set our last_update_dir to the direction we just moved.
        self.last_update_dir = self.dir;
    }

    /// Here we have the Snake draw itself. This is very similar to how we saw the Food
    /// draw itself earlier.
    ///
    /// Again, note that this approach to drawing is fine for the limited scope of this
    /// example, but larger scale games will likely need a more optimized render path
    /// using SpriteBatch or something similar that batches draw calls.
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // We first iterate through the body segments and draw them.
        self.body
            .iter()
            .map(|seg| seg.pos)
            .try_for_each(|pos| draw_rect(pos, SNAKE_CONFIG.snake_body, ctx))?;

        // And then we do the same for the head, instead making it fully red to distinguish it.
        draw_rect(self.head.pos, SNAKE_CONFIG.snake_head, ctx)?;
        Ok(())
    }
}

/// This is mostly just a semantic abstraction over a `GridPosition` to represent
/// a segment of the snake. It could be useful to, say, have each segment contain its
/// own color or something similar. This is an exercise left up to the reader ;)
#[derive(Clone, Copy, Debug)]
pub struct Segment {
    pub pos: GridPosition,
}

impl Segment {
    pub fn new(pos: GridPosition) -> Self {
        Segment { pos }
    }
}
