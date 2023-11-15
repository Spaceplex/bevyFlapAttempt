use bevy::prelude::*;
mod bird;
mod physics;
mod input;
mod gamedata;
mod gamestate;

use physics::*;
use bird::*;
use gamedata::*;
use gamestate::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(Startup, setup)
        .insert_resource(GameState::Menu)
        .insert_resource(Score(0))
        .run();
}

fn setup(mut commands: Commands){
    //camera
    commands.spawn(Camera2dBundle::default());
}

