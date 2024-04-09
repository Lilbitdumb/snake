use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use snake::{
    init_snake,
    snake::{spawn_segment, Position, SnakePlugin},
    MAP_SIZE,
};

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
        .add_plugins((SnakePlugin))
        .add_systems(Startup, (setup, init_snake))
        .add_systems(Update, test_spawn_segments)
        .run();
}

pub fn setup(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}

fn test_spawn_segments(commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    spawn_segment(commands, Position::default());
}
