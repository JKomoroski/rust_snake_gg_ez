use rand;
use rand::Rng;
use ggez::graphics;
use crate::Direction;
use crate::GRID_SIZE;
use crate::CELL_SIZE;
use std::u8;

/// Now we define a struct that will hold an entity's position on our game board
/// or grid which we defined above. We'll use signed integers because we only want
/// to store whole numbers, and we need them to be signed so that they work properly
/// with our modulus arithmetic later.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub x: u8,
    pub y: u8,
}

impl GridPosition {
    /// We make a standard helper function so that we can create a new `GridPosition`
    /// more easily.
    pub fn new(x: u8, y: u8) -> Self {
        GridPosition { x, y }
    }

    /// As well as a helper function that will give us a random `GridPosition` from
    /// `(0, 0)` to `(max_x, max_y)`
    pub fn random(max_x: u8, max_y: u8) -> Self {
        let mut rng = rand::thread_rng();
        // We can use `.into()` to convert from `(u8, u8)` to a `GridPosition` since
        // we implement `From<(u8, u8)>` for `GridPosition` below.
        (
            rng.gen_range::<u8, u8, u8>(0, max_x),
            rng.gen_range::<u8, u8, u8>(0, max_y),
        )
            .into()
    }

    /// We'll make another helper function that takes one grid position and returns a new one after
    /// making one move in the direction of `dir`. We use our `SignedModulo` trait
    /// above, which is now implemented on `u8` because it satisfies the trait bounds,
    /// to automatically wrap around within our grid size if the move would have otherwise
    /// moved us off the board to the top, bottom, left, or right.
    pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
        match dir {
            Direction::Up => GridPosition::new(pos.x, (pos.y.checked_sub(1).unwrap_or(GRID_SIZE.1 - 1)) % GRID_SIZE.1),
            Direction::Down => GridPosition::new(pos.x, (pos.y + 1) % GRID_SIZE.1),
            Direction::Left => GridPosition::new(pos.x.checked_sub(1).unwrap_or(GRID_SIZE.0 - 1) % GRID_SIZE.0, pos.y),
            Direction::Right => GridPosition::new((pos.x + 1) % GRID_SIZE.0, pos.y),
        }
    }
}

/// We implement the `From` trait, which in this case allows us to convert easily between
/// a GridPosition and a ggez `graphics::Rect` which fills that grid cell.
/// Now we can just call `.into()` on a `GridPosition` where we want a
/// `Rect` that represents that grid cell.
impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * CELL_SIZE.0 as i32,
            pos.y as i32 * CELL_SIZE.1 as i32,
            CELL_SIZE.0 as i32,
            CELL_SIZE.1 as i32,
        )
    }
}

/// And here we implement `From` again to allow us to easily convert between
/// `(u8, u8)` and a `GridPosition`.
impl From<(u8, u8)> for GridPosition {
    fn from(pos: (u8, u8)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

/// This is a trait that provides a modulus function that works for negative values
/// rather than just the standard remainder op (%) which does not. We'll use this
/// to get our snake to wrap from one side of the game board around to the other
/// when it goes off the top, bottom, left, or right side of the screen.
trait ModuloSigned {
    fn modulo(&self, n: Self) -> Self;
}

/// Here we implement our `ModuloSigned` trait for any type T which implements
/// `Add` (the `+` operator) with an output type T and Rem (the `%` operator)
/// that also has an output type of T, and that can be cloned. These are the bounds
/// that we need in order to implement a modulus function that works for negative numbers
/// as well.
impl<T> ModuloSigned for T
    where
        T: std::ops::Add<Output = T> + std::ops::Rem<Output = T> + Clone,
{
    fn modulo(&self, n: T) -> T {
        // Because of our trait bounds, we can now apply these operators.
        (self.clone() % n.clone() + n.clone()) % n.clone()
    }
}
