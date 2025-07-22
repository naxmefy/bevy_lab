use bevy::prelude::*;
use crate::people::{Name, Person, PersonPlugin};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PersonPlugin);
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Update, (greet_people).chain());
    }
}

pub struct HelloPlugin2;

impl Plugin for HelloPlugin2 {
    fn build(&self, app: &mut App) {
        app.add_plugins(PersonPlugin);
        app.insert_resource(GreetTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
        app.add_systems(Update, (greet_people).chain());
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello 2 {}!", name.as_str());
        }
    }
}