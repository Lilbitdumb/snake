use bevy::prelude::*;
use rand::random;
use bevy::time::common_conditions::on_timer;



use crate::{ARENA_HEIGHT, ARENA_WIDTH, FOOD_COLOR};
use crate::snake::Size;
use crate::snake::Position;

#[derive(Component)]
pub struct Food;


fn spawn_food(mut commands: Commands) { 
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: FOOD_COLOR,            
            ..default()
        },        
        ..default()
    })
    .insert((Food, Name::new("Food")))
    .insert(Position{
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
    })
    .insert(Size::square(0.5));
}



pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_food.run_if(on_timer(std::time::Duration::from_secs_f64(5.0)))); 
       
    }
}
