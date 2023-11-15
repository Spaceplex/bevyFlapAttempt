use bevy::{prelude::*, input::InputSystem};

use crate::Velocity;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Bird;


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird);
    }
}

fn spawn_bird(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load("bird.png"),
        transform: Transform {
            translation: Vec3::ZERO,
            ..default()
        },
        ..default()
    },
    Bird,
    Velocity(Vec2::new(0.0, 0.0)),
    ));
} 

fn jump(
    mut player: Query<(&mut Transform, &mut Velocity), With<Bird>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    ) {

}
