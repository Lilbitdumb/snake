use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

use crate::{TAIL_COLOR,TAIL_DIMENSION,SNAKE_DIMENSION};

use crate::player::Player;
use crate::player::Direction;

#[derive(Component)]
pub struct TailParent; 

fn spawn_tail_parent(mut commands: Commands){
    commands.spawn((SpatialBundle::default(), TailParent, Name::new("Tail Parent")));
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component)]
pub struct Tail;

fn tail_spawn(
    mut commands: Commands,
    parent: Query<Entity, With <TailParent>>,
    player: Query<&Transform, With<Player>>,     
    mut query: Query<(&Direction, &Transform, &Player)>, 
    input: Res<ButtonInput<KeyCode>>,  
){
    if !input.just_pressed(KeyCode::Space){        
        return;    
    }

    let parent = parent.single();
    let mut tail_position = Vec3::new(0.0,0.0,0.0);        
    
    for (direction, transform, player) in query.iter_mut(){
        match direction {
            Direction::Up => {
                tail_position = transform.translation - Vec3::new(0.0,1.0,0.0) * SNAKE_DIMENSION/2.0;
            }
            Direction::Down => {
                ;
            }
            Direction::Left => {
                ;
            }
            Direction::Right => {
                ;
            }
        }
    }        
    commands.entity(parent).with_children(|commands|{
        commands.spawn(
            SpriteBundle {
                sprite: Sprite{
                    color: TAIL_COLOR,
                    custom_size: Some(Vec2::new(TAIL_DIMENSION,TAIL_DIMENSION)),                    
                    ..default()
                },
                transform: Transform{
                    translation: tail_position,
                    ..default()
                },                           
                ..default()
            },            
        );
    }); 
}

fn tail_movment(){

}


fn tail_collsion(){

}

pub struct TailPlugin;
impl Plugin for TailPlugin {
    fn build(&self, app:&mut App){
        app.add_systems(Startup, spawn_tail_parent)
        .add_systems(Update, tail_spawn)
        .register_type::<Tail>();
    }    
}
   
