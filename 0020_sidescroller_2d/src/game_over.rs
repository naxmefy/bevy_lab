use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverText;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_game_over_view);
    }
}

fn setup_game_over_view(mut commands: Commands) {
    // commands.spawn(TextBundle {
    //     text: Text {
    //         value: "Game Over".to_string(),
    //         font: commands.resource::<Handle<Font>>().unwrap().clone(),
    //         style: TextStyle {
    //             font_size: 100.0,
    //             color: Color::rgb(1.0, 0.0, 0.0),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // })
    // .with(GameOverText);
}