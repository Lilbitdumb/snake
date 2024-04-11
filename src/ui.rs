use bevy::prelude::*;

use snake::snake::Score;

#[derive(Component)]
pub struct ScoreText;

fn spawn_game_ui(mut commands: Commands){
    commands.spawn((
        TextBundle{
            text: Text::from_section(
                "Score!",
                TextStyle{
                    font_size: 32.0,
                    ..default()
                },
            ),
            ..default()
        },
        ScoreText,
    ));
}

fn update_game_ui(
    mut texts: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>
){
    for mut text in &mut texts{
        text.sections[0].value = format!("Score:{}",score.0);
    }
}


pub struct GameUI;
impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_game_ui)
            .add_systems(Update, update_game_ui);
    }
}