use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{BOUNDARY_DIMENSION, FOOD_COLOR, MAP_SIZE};

#[derive(Component)]
struct Food;

fn spawn_food(mut commands: Commands) {
    let pos_x = thread_rng().gen_range(
        (-(MAP_SIZE / 2.0) + (SNAKE_DIMENSION + BOUNDARY_DIMENSION + 50.0)) as f32
            ..((MAP_SIZE / 2.0) - (SNAKE_DIMENSION + BOUNDARY_DIMENSION + 50.0)) as f32,
    );

    let pos_y = thread_rng().gen_range(
        (-(MAP_SIZE / 2.0) + (SNAKE_DIMENSION + BOUNDARY_DIMENSION + 50.0)) as f32
            ..((500.0 / 2.0) - (SNAKE_DIMENSION + BOUNDARY_DIMENSION + 50.0)) as f32,
    );

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: FOOD_COLOR,
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(pos_x, pos_y, 0.0),
            ..default()
        },
        ..default()
    });
}

pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_food);
    }
}
