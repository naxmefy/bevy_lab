use bevy::prelude::*;

pub struct PersonPlugin;

impl Plugin for PersonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, update_people);
    }
}

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(String);

impl Name {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}