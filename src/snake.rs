use bevy::prelude::*;
use crate::{terminate, GamePosition, PIX_AREA_SF, update_score_ui, RestartGame, restart_game};
use crate::apple::{Apple, apple_relocate};

pub struct GameSnake;
impl Plugin for GameSnake {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake)
        .add_systems(Update, direction_detection)
        .add_systems(FixedUpdate, (snake_move, collision_detection, increase_body, spawn_segment, apple_relocate, update_score_ui,restart_game).chain())
        .add_event::<GrowthEvent>()
        .insert_resource(Score(0))
        .insert_resource(SegSpawnLoc{x:3,y:4});
    }
}
#[derive(Resource)]
pub struct Score(pub i32);

#[derive(Resource)] 
struct SegSpawnLoc {
    x: i32,
    y: i32,
}
#[derive(Event)]
pub struct GrowthEvent();

#[derive(Component)]
pub struct Snake();
#[derive(Component)]
struct Direction(Directions);

//Track body segment positions in order
#[derive(Component)]
pub struct Body {
    pub body_pos: Vec<(i32,i32)>,
}

#[derive(Component)]
pub struct SegmentMarker();
enum Directions {
    Right,
    Left, 
    Up,
    Down,
}

pub fn spawn_snake(mut commands: Commands) {

    commands.spawn((Sprite {
        rect: Some(Rect::new(0.,0., PIX_AREA_SF as f32, PIX_AREA_SF as f32)),
        color: Color::srgb(0.1, 0.7, 0.1),
        ..default()
    }, 
    Transform {
        translation: Vec3::new(4 as f32 * PIX_AREA_SF, 4 as f32 * PIX_AREA_SF, 1.),
        ..default()
    }))
    .insert((Snake(), GamePosition{x: 4, y:4}, Direction(Directions::Right), Body{body_pos: vec![]}));

}

fn spawn_segment(mut commands: Commands, mut events: EventReader<GrowthEvent>, spawn_loc: Res<SegSpawnLoc>) {
    for event in events.read() {

        commands.spawn((Sprite {
            rect: Some(Rect::new(0.,0., PIX_AREA_SF as f32, PIX_AREA_SF as f32)),
            color: Color::srgb(0.1, 0.4, 0.1),
            ..default()
        }, 
        Transform {
            translation: Vec3::new(spawn_loc.x as f32 * PIX_AREA_SF, spawn_loc.y as f32 * PIX_AREA_SF, 1.),
            ..default()
        }))
        .insert(SegmentMarker());
    }
}

fn direction_detection(keyboard_input: Res<ButtonInput<KeyCode>>, mut snakes: Query<&mut Direction, With<Snake>>) {
    let mut direction = snakes.single_mut();

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.0 = Directions::Up;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.0 = Directions::Down;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.0 = Directions::Right;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.0 = Directions::Left;
    }
}

fn snake_move(mut snakes: Query<(&mut GamePosition,&Direction), With<Snake>>, mut bodies: Query<&mut Body, With<Snake>>, mut SegSpawnLoc: ResMut<SegSpawnLoc>, mut restart: EventWriter<RestartGame>) {
    
    //There is 1 snake (hopefully)
    let snake = snakes.single_mut();

    let direction = snake.1;
    
    let mut pos = snake.0;

    let mut body = bodies.single_mut();

    let mut restart_required = false;

    
    if !body.body_pos.is_empty() {
        
        let last_index = body.body_pos.len() - 1;

        let x = body.body_pos[last_index].0;
        let y = body.body_pos[last_index].1;
        SegSpawnLoc.x = x;
        SegSpawnLoc.y = y;
        for i in (1..body.body_pos.len()).rev() {
            body.body_pos[i] = body.body_pos[i-1]
        }
        body.body_pos[0].0 = pos.x;
        body.body_pos[0].1 = pos.y;
    } else {
        SegSpawnLoc.x = pos.x;
        SegSpawnLoc.y = pos.y;
    }
    
    match direction.0 {
        Directions::Right => {
            if pos.x + 1 >= 10 {
                restart_required = true;
            } else {
                pos.x += 1;
            }
            
        }
        Directions::Left => {
            if pos.x - 1 < 0 {
                restart_required = true;
            } else {
                pos.x -= 1;
            }
           
        }
        Directions::Up => {
            if pos.y + 1 >= 10 {
                restart_required = true;
            } else {
                pos.y += 1;
            }
            
        }
        Directions::Down => {
            if pos.y - 1 < 0 {
                restart_required = true;
            } else {
                pos.y -= 1;
            }
        }
        
    }

    for p in &body.body_pos {
        if p.0 == pos.x && p.1 == pos.y {
            restart_required = true;
        }
    }

    if restart_required {
        restart.send(RestartGame);
    }
}

fn increase_body(mut growth_event: EventReader<GrowthEvent>,mut snakes: Query<&mut Body, With<Snake>>, loc: Res<SegSpawnLoc>) {
    for event in growth_event.read() {
        let mut snake = snakes.single_mut();
        snake.body_pos.push((loc.x, loc.y));
    }
}

//I think this method of parameter layout is ugly but when parameters > 3 its kind of nessecary
fn collision_detection(
    mut score: ResMut<Score>, 
    snakes: Query<&GamePosition, With<Snake>>, 
    apples: Query<&GamePosition, (Without<Snake>, With<Apple>)>,
    mut growth_event: EventWriter<GrowthEvent>
    ) {

    let snake = snakes.single();
    let apple = apples.single();

    if snake.x == apple.x && snake.y == apple.y {
        score.0 += 1;
        growth_event.send(GrowthEvent());
        //snake on apple :D
    }
}
