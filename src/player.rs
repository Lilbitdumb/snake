use bevy::{ecs::query, input::keyboard::KeyCode};
use bevy::{prelude::*, sprite, transform};
use bevy_inspector_egui::InspectorOptions;
use rand::{thread_rng, Rng};

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component)]
pub struct Player{   
   pub speed: f32
   
}

#[derive(Component)]
pub enum  Direction{
    Up,
    Down,
    Left,
    Right, 
}

fn direction_detection(
    mut query: Query<&mut Direction, With<Player>>,  
    input: Res<ButtonInput<KeyCode>>,   
) {
    for mut direction in query.iter_mut(){        
        if input.pressed(KeyCode::KeyW){
            *direction = Direction::Up;
        }
        if input.pressed(KeyCode::KeyS){
            *direction = Direction::Down;   
        }
        if input.pressed(KeyCode::KeyD){
            *direction = Direction::Right;
        }
        if input.pressed(KeyCode::KeyA){
            *direction = Direction::Left;  
        }       

    }
}

fn player_movement(
    mut query: Query<(&Direction, &mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (direction, mut transform, player) in query.iter_mut() {
        let movement_speed = player.speed; 
        match direction {
            Direction::Up => {
                transform.translation.y += movement_speed * time.delta_seconds();
            }
            Direction::Down => {
                transform.translation.y -= movement_speed * time.delta_seconds();
            }
            Direction::Left => {
                transform.translation.x -= movement_speed * time.delta_seconds();
            }
            Direction::Right => {
                transform.translation.x += movement_speed * time.delta_seconds();
            }
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app:&mut App){
        app.add_systems(Update, (direction_detection,player_movement))
        .register_type::<Player>();
    }    
}

    