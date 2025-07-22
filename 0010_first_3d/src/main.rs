use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_movement)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    // Kamera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Spieler
    commands.spawn((
        PbrBundle {
            mesh: bevy::prelude::shape::Cube { size: 1.0 }.into(),
            material: Color::rgb(0.0, 0.5, 1.0).into(),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
    ));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();
    let speed = 5.0 * time.delta_seconds();

    if keyboard_input.pressed(KeyCode::W) {
        transform.translation.z -= speed;
    }
    if keyboard_input.pressed(KeyCode::S) {
        transform.translation.z += speed;
    }
    if keyboard_input.pressed(KeyCode::A) {
        transform.translation.x -= speed;
    }
    if keyboard_input.pressed(KeyCode::D) {
        transform.translation.x += speed;
    }
}
