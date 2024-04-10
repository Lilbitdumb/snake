use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        reflect::ReflectComponent,
        system::{Commands, ResMut, Resource},
    }, prelude::*, reflect::Reflect, sprite::{Sprite, SpriteBundle}, time::common_conditions::on_timer, transform::commands, window::PrimaryWindow
};
use bevy_inspector_egui::InspectorOptions;

use crate::{ARENA_HEIGHT, ARENA_WIDTH, SNAKE_COLOR, TAIL_COLOR};
use crate::food::Food;

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(Component)]
pub struct SnakeHead {
    pub direction: Direction,
}

#[derive(Reflect)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Reflect)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Resource, Default, Deref, DerefMut)]
struct SnakeSegments(Vec<Entity>);

#[derive(Resource, Default)]
struct LastTailPosition(Option<Position>);

#[derive(Event)]
struct GrowthEvent;


fn init_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert((SnakeSegment, Name::new("SnakeHead")))
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ]);
}

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: TAIL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(position)
        .insert(Size::square(0.65))
        .insert(SnakeSegment)
        .id()
}

fn direction_detection(mut query: Query<&mut SnakeHead>, input: Res<ButtonInput<KeyCode>>) {
    if let Some(mut snake_head) = query.iter_mut().next() {
        match &snake_head.direction {
            Direction::Up | Direction::Down => {
                if input.just_pressed(KeyCode::KeyA) {
                    snake_head.direction = Direction::Left;
                } else if input.just_pressed(KeyCode::KeyD) {
                    snake_head.direction = Direction::Right;
                }
            }
            Direction::Left | Direction::Right => {
                if input.just_pressed(KeyCode::KeyW) {
                    snake_head.direction = Direction::Up;
                } else if input.just_pressed(KeyCode::KeyS) {
                    snake_head.direction = Direction::Down;
                }
            }
        }
    }
}

fn snake_movement(
    mut head: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_pos: ResMut<LastTailPosition>,
    segments: ResMut<SnakeSegments>,
) {
    if let Some((head_entity, head)) = head.iter_mut().next() {
        let segment_positions = segments
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();

        *last_tail_pos = LastTailPosition(Some(*segment_positions.last().unwrap()));

        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };

        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, entity)| {
                *positions.get_mut(*entity).unwrap() = *pos;
            });
    }
}

fn size_scaling(
    mut q: Query<(&Size, &mut Transform)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = if let Ok(window) = window.get_single() {
        window.clone()
    } else {
        return;
    };
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(
    window: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Position, &mut Transform)>,
) {
    let window = if let Ok(window) = window.get_single() {
        window.clone()
    } else {
        info!("No window found");
        return;
    };
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn snake_eating(
    mut commands: Commands,
    mut event: EventWriter<GrowthEvent>,
    food_position: Query<(Entity, &Position), With<Food>>,
    head_position: Query<&Position, With<SnakeHead>>,
){
    for head_pos in head_position.iter(){
        for (ent, food_pos) in food_position.iter(){
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                event.send(GrowthEvent);
            }
        }
    }
}


fn snake_growth(
    commands: Commands,
    last_tail_pos: ResMut<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>
){
    if growth_reader.read().next().is_some(){
        segments.push(spawn_segment(commands,last_tail_pos.0.unwrap()))
    }
}

pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_snake)
            .insert_resource(SnakeSegments::default())
            .insert_resource(LastTailPosition::default())
            .add_systems(Update, direction_detection)
            .add_systems(
                Update,
                snake_movement.run_if(on_timer(std::time::Duration::from_millis(150))),
            )
            .add_systems(Update, snake_eating.run_if(on_timer(std::time::Duration::from_millis(150))).after(snake_movement))            
            .add_systems(Update,snake_growth.after(snake_eating))
            .add_systems(PostUpdate, (size_scaling, position_translation))
            .add_event::<GrowthEvent>()
            .register_type::<SnakeHead>()
            .register_type::<Direction>()
            .register_type::<Size>()
            .register_type::<Position>();
    }
}
