use bevy::prelude::*;
use rand::Rng;

use crate::{GamePosition, GrowthEvent, PIX_AREA_SF};


pub struct GameApple;
impl Plugin for GameApple {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_apple);
        
    }
}

#[derive(Component)]
pub struct Apple();

pub fn spawn_apple(mut commands: Commands) {
    let x = rand::thread_rng().gen_range(0, 9);
    let y = rand::thread_rng().gen_range(0,9);

    
    commands.spawn((Sprite {
        color: Color::srgb(0.7, 0.1, 0.1),
        rect: Some(Rect::new(0.,0., (720/10) as f32, (720/10) as f32)),
        ..default()
    }, 
    Transform {
        translation: Vec3::new(x as f32 * PIX_AREA_SF, y as f32 * PIX_AREA_SF, 0.),
        ..default()
    }))
    .insert((Apple(), GamePosition{x , y}));

}   

// This has to be imported into apple since this has to be chained AFTER the event trigger to call instantly and not after 1 fixed update interval
pub fn apple_relocate(mut events: EventReader<GrowthEvent>, mut apples: Query<&mut GamePosition, With<Apple>>) {
    for event in events.read() {
        let mut apple = apples.single_mut();
        apple.x = rand::thread_rng().gen_range(0, 9);
        apple.y = rand::thread_rng().gen_range(0, 9);
    }
}