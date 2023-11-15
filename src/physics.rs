use bevy::prelude::*;
use crate::bird;
use bird::Bird;

const GRAVITY: f32 = 100.5;

#[derive(Component)]
pub struct Velocity(pub Vec2);

fn bird_fall(
    mut query: Query<&mut Transform, With<Bird>>,
    time: Res<Time>,
) {
    let mut transform = query.get_single_mut().unwrap();
    transform.translation.y -= GRAVITY * time.delta_seconds();
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, bird_fall);
    }
}
