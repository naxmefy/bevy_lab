mod player;
mod game_over;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game_over::GameOverPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerPlugin, GameOverPlugin))
        .run();
}

