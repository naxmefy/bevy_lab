mod hello;
mod people;

use bevy::prelude::*;
use crate::hello::{HelloPlugin, HelloPlugin2};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .add_plugins(HelloPlugin2)
        .run();
}
