use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game_over::GameOverText;

#[derive(Component)]
pub struct Player;

const PLAYER_SPEED: f32 = 300.0;
const JUMP_FORCE: f32 = 500.0;
const GRAVITY: f32 = -9.81 * 100.0;
const FALL_LIMIT: f32 = -200.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Startup, setup_player_camera);
    }
}

fn setup_player_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.8, 0.2),
                custom_size: Some(Vec2::new(32.0, 64.0)),
                ..default()
            },
            transform: Transform::from_xyz(-100.0, 50.0, 0.0),
            ..default()
        },
    ));
}

fn player_movement(keyboard_input: Res<ButtonInput<KeyCode>>) {
    // Seitwärtsbewegung
    let mut movement = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA) {
        movement -= PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        movement += PLAYER_SPEED;
    }
    // velocity.linvel.x = movement;

    // Springen nur, wenn der Spieler auf dem Boden ist
    // if keyboard_input.just_pressed(KeyCode::Space) && velocity.linvel.y.abs() < 0.1 {
        // velocity.linvel.y = JUMP_FORCE;
    // }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
        }
    }
}

fn check_fall(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if player_transform.translation.y < FALL_LIMIT {
            // Spieler ist gefallen, Game Over anzeigen
            player_transform.translation = Vec3::new(-100.0, 50.0, 0.0); // Zurücksetzen zur Startposition
            // if let Ok(mut game_over_visibility) = game_over_query.get_single_mut() {
            //     game_over_visibility.is_visible = true;
            // }
        }
    }
}
