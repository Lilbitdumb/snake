use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use snake::{food, snake::SnakePlugin, MAP_SIZE};
use food::FoodPlugin;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "snake".into(),
                        resolution: (MAP_SIZE, MAP_SIZE).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        ))
        .add_systems(Startup, setup)
        .add_plugins((SnakePlugin,FoodPlugin))
        .run();
}

pub fn setup(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}
