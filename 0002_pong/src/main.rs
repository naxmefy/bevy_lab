use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_players, spawn_ball));
    app.add_systems(Update, (move_paddle, move_ball));
    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode
}

fn spawn_players(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(700., 500.)),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-300., 0., 0.)),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(10., 150.)),
            ..Default::default()
        },
        ..Default::default()
    }, Paddle {
        move_up: KeyCode::KeyW,
        move_down: KeyCode::KeyS,
    }));

    commands.spawn((SpriteBundle {
        transform: Transform::from_translation(Vec3::new(300., 0., 0.)),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(10., 150.)),
            ..Default::default()
        },
        ..Default::default()
    }, Paddle {
        move_up: KeyCode::ArrowUp,
        move_down: KeyCode::ArrowDown,
    }));
}

fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut pos, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            pos.translation.y += 200. * time.delta_seconds();
        }

        if input.pressed(settings.move_down) {
            pos.translation.y -= 200. * time.delta_seconds();
        }

        pos.translation.y = pos.translation.y.clamp(-250. + 75., 250. - 75.);
    }
}

#[derive(Component)]
struct Ball(Vec2);

fn spawn_ball(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(25., 25.)),
            ..Default::default()
        },
        ..Default::default()
    }, Ball(Vec2::new(-100., 0.))));
}

fn move_ball(
    mut balls: Query<(&mut Transform, &Ball)>,
    time: Res<Time>,
) {
    for (mut pos, ball) in &mut balls {
        pos.translation += ball.0.extend(0.) * time.delta_seconds();
    }
}

fn ball_collide(
    mut balls: Query<(&mut Transform, &Ball)>,
    paddles: Query<&Transform, With<Paddle>>
) {
    for (pos, mut ball) in &mut balls {
        for paddle in &paddles {
            
        }
    }
}
