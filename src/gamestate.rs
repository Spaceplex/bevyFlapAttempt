use bevy::prelude::Resource;

#[derive(Resource)]
pub enum GameState {
    Menu,
    Playing,
    Dead,
}

