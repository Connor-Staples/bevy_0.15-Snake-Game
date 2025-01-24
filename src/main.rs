mod apple;
mod snake;

use core::time;
use std::thread::sleep;

use apple::{spawn_apple, GameApple};

use snake::*; 

use bevy::{prelude::*, window::WindowResolution};

const GAME_PIX_SIZE: f32 = 720.; 
const GAME_AREA: i32 = 10;
const PIX_AREA_SF: f32 = GAME_PIX_SIZE / GAME_AREA as f32;
fn main() {
    {
        // set the WPGU_BACKEND environment variable to "dx12"
        //  https://github.com/bevyengine/bevy/issues/3406
        std::env::set_var("WGPU_BACKEND", "dx12");
    }

    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(GAME_PIX_SIZE, GAME_PIX_SIZE),
            title: "Snake Game".to_string(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(Time::<Fixed>::from_seconds(0.5))
    
    .add_plugins((GameApple, GameSnake))
    
    .add_systems(Startup, setup_camera)
    .add_systems(Update, update_screen)
    .add_event::<RestartGame>()
    .run();
}
#[derive(Event)]
pub struct RestartGame;

#[derive(Component)]
pub struct GamePosition {
    x: i32,
    y: i32,
}
#[derive(Component)]
pub struct ScoreUI;

fn setup_camera(mut commands: Commands) {
    //translation since we are using the first (positive) quadrant only
    commands.spawn((Camera2d, Transform {
        translation: Vec3::new((GAME_PIX_SIZE * 0.5) - PIX_AREA_SF * 0.5,(GAME_PIX_SIZE * 0.5) - PIX_AREA_SF * 0.5,0.),
        ..default()
    }));


    // UI here
    commands.spawn((Node {
        //Fill the entire window now its like working with CSS, Pain :(
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        ..default()
    })).with_children(|commands| {
        commands.spawn((
            Text::new("Score: 0"),
            TextFont {
                font_size: 50.,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                position_type: PositionType::Absolute,
                // Pads right and left, hence it centers
                margin: UiRect {
                    right: Val::Auto,
                    left: Val::Auto,
                    bottom: Val::Px(0.),
                    top: Val::Px(0.),
                },
                
                ..default()
            },
            Name::new("Score"),
            ScoreUI,
        ));
    });

}

// this gets called in snake
pub fn update_score_ui(mut text: Query<&mut Text, With<ScoreUI>>, score: Res<Score>) {
    for mut t in text.iter_mut() {
        let text = format!("Score: {}", score.0);
        t.0 = text;
    }

}

//updates the transforms of all entities with a GamePosition
fn update_screen(mut transforms: Query<(&mut Transform, &mut GamePosition), With<GamePosition>>, mut body: Query<&mut Body, With<GamePosition>>, mut body_transforms: Query<&mut Transform, (With<SegmentMarker>, Without<GamePosition>)>) {

    for i in transforms.iter_mut() {
        let mut transform = i.0;
        let position = i.1;

        transform.translation.x = position.x as f32 * PIX_AREA_SF;
        transform.translation.y = position.y as f32 * PIX_AREA_SF;
    }

    let mut body_t: Vec<Mut<Transform>> = body_transforms.iter_mut().collect();
    let b = body.single_mut();

    for i in 0..body_t.len() {
        body_t[i].translation.x = b.body_pos[i].0 as f32 * PIX_AREA_SF;
        body_t[i].translation.y = b.body_pos[i].1 as f32 * PIX_AREA_SF;
    }
}

pub fn terminate() {
    sleep(time::Duration::from_secs(5));
    println!("You Lost!")
}

pub fn restart_game(mut events: EventReader<RestartGame>, mut entities: Query<Entity, With<SegmentMarker>>,mut commands: Commands, mut snakes: Query<(&mut GamePosition, &mut Body), With<Snake>>, mut score: ResMut<Score>) {
    for event in events.read() {
        
        score.0 = 0;
        //despawn snake body segments
        for entity in entities.iter_mut() {
            if commands.get_entity(entity).is_some() {
                commands.entity(entity).despawn();
            }
        }

        //reset snake to spawn location
        let mut snake = snakes.single_mut();
        let mut pos = snake.0;
        pos.x = 4;
        pos.y = 4;

        snake.1.body_pos = vec![];
        
    }
}