//! A small snake game done after watching
//! <https://www.youtube.com/watch?v=HCwMb0KslX8>
//! to showcase ggez and how it relates/differs from piston.
//!
//! Note that this example is meant to highlight the general
//! structure of a ggez game. Some of the details may need to
//! be changed to scale the game. For example, if we needed to
//! draw hundreds or thousands of shapes, a SpriteBatch is going
//! to offer far better performance than the direct draw calls
//! that this example uses.
//!
//! Author: @termhn
//! Original repo: https://github.com/termhn/ggez_snake

use ggez::conf::FullscreenType;
use ggez::{event, GameResult};
use std::fmt::Debug;

use config;
use serde::Deserialize;

mod components;
use components::game_state::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref SNAKE_CONFIG: SnakeConfig = try_config();
    pub static ref SCREEN_SIZE: (f32, f32) = (
        SNAKE_CONFIG.grid_width as f32 * SNAKE_CONFIG.cell_width as f32,
        SNAKE_CONFIG.grid_height as f32 * SNAKE_CONFIG.cell_height as f32,
    );
}

fn main() -> GameResult {
    let window = ggez::conf::WindowMode {
        width: SCREEN_SIZE.0,
        height: SCREEN_SIZE.1,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };

    // Here we use a ContextBuilder to setup metadata about our game. First the title and author
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("snake", "Gray Olson")
        // Next we set up the window. This title will be displayed in the title bar of the window.
        .window_setup(ggez::conf::WindowSetup::default().title("Snake!"))
        // Now we get to set the size of the window, which we use our SCREEN_SIZE constant from earlier to help with
        .window_mode(window)
        // And finally we attempt to build the context and create the window. If it fails, we panic with the message
        // "Failed to build ggez context"
        .build()?;

    // Next we create a new instance of our GameState struct, which implements EventHandler
    let state = &mut GameState::new();
    // And finally we actually run our game, passing in our context and state.
    return event::run(ctx, events_loop, state);
}

fn try_config() -> SnakeConfig {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Config")).unwrap();
    settings
        .merge(config::Environment::with_prefix("SNAKE"))
        .unwrap();
    settings.try_into::<SnakeConfig>().unwrap()
}

#[derive(Deserialize, Debug)]
pub struct SnakeConfig {
    pub grid_width: u8,
    pub grid_height: u8,
    pub cell_width: u8,
    pub cell_height: u8,
    pub updates_per_second: f32,
    pub background: [f32; 4],
    pub snake_head: [f32; 4],
    pub snake_body: [f32; 4],
    pub food: [f32; 4],
}
