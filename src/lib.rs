use bevy::prelude::*;

pub mod food;
pub mod snake;

pub const MAP_SIZE: f32 = 500.0;
pub const SNAKE_COLOR: Color = Color::WHITE;
pub const TAIL_COLOR: Color = Color::GRAY;
pub const FOOD_COLOR: Color = Color::RED;
pub const BOUNDARY_DIMENSION: f32 = 10.0;
pub const FRAME_TIME: f64 = 0.1;
pub const SNAKE_SPEED: f32 = 2.0;
pub const ARENA_WIDTH: u32 = 10;
pub const ARENA_HEIGHT: u32 = 10;
