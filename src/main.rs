use player::PlayerPlugin;
use rand::prelude::*;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use tail::TailPlugin;

pub const MAP_SIZE: f32 = 500.0;
pub const SNAKE_COLOR: Color = Color::WHITE;
pub const TAIL_COLOR: Color = Color::GRAY;
pub const SNAKE_DIMENSION: f32 = 30.0;
pub const TAIL_DIMENSION: f32 = 20.0;
pub const BOUNDARY_DIMENSION: f32 = 10.0;
pub const FRAME_TIME: f64 = 0.1;
use crate::player::Player;
use crate::player::Direction;
use crate::tail::TailParent;
use crate::tail::Tail;

mod player;
mod tail;


fn setup(
    mut commands: Commands)
    {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin { min_width: MAP_SIZE, min_height: MAP_SIZE };      
    commands.spawn(camera);
   
    commands.spawn(        
        (SpriteBundle {
            sprite: Sprite{
                color: SNAKE_COLOR,
                custom_size: Some(Vec2::new(SNAKE_DIMENSION,SNAKE_DIMENSION)),
                ..default()
            },                  
            transform: Transform{
                translation: Vec3::new(
                thread_rng().gen_range((-(MAP_SIZE / 2.0) + (SNAKE_DIMENSION + BOUNDARY_DIMENSION + 50.0)) as f32 .. ((MAP_SIZE / 2.0) - (SNAKE_DIMENSION + BOUNDARY_DIMENSION + 50.0)) as f32),
                thread_rng().gen_range((-(MAP_SIZE / 2.0) + (SNAKE_DIMENSION + BOUNDARY_DIMENSION + 50.0)) as f32 .. ((MAP_SIZE / 2.0) - (SNAKE_DIMENSION + BOUNDARY_DIMENSION + 50.0)) as f32),
                0.0),
                ..default()
            }, 
            ..default()
            
        },         
                  
    )).insert((Player {speed: 100.0}, Name::new("Player")))
    .insert(Direction::Up);     
    
}  


fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins
        .set(ImagePlugin::default_nearest()) 
        .set(WindowPlugin{
            primary_window: Some(Window {
                title: "snake".into(),
                resolution:(MAP_SIZE,MAP_SIZE).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        })
      .build(),
    ) 
    .add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),)  
    .add_plugins((PlayerPlugin,TailPlugin)) 
    .add_systems(Startup, setup)              
    .run();      
}

