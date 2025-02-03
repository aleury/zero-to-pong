#![allow(dead_code, clippy::needless_pass_by_value)]
use bevy::prelude::*;
use rand::Rng;

const BALL_WIDTH: f32 = 25.0;
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 150.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_players, spawn_ball))
        .add_systems(Update, (move_paddle, move_ball, collision_detection))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Component)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
}

fn spawn_players(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(700.0, 500.0)),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
            ..default()
        },
    ));

    commands.spawn((
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(300.0, 0.0, 0.0)),
            ..default()
        },
    ));
}

fn move_paddle(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut paddles: Query<(&mut Transform, &Paddle)>,
) {
    for (mut pos, paddle) in &mut paddles {
        if input.pressed(paddle.move_up) {
            pos.translation.y += 200.0 * time.delta_seconds();
        }
        if input.pressed(paddle.move_down) {
            pos.translation.y -= 200.0 * time.delta_seconds();
        }
        pos.translation.y = pos.translation.y.clamp(-250.0 + 75.0, 250.0 - 75.0);
    }
}

#[derive(Debug, Component)]
struct Ball(Vec2);

fn spawn_ball(mut commands: Commands) {
    commands.spawn((
        Ball(Vec2::new(250.0, 0.0)),
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(BALL_WIDTH, BALL_WIDTH)),
                ..default()
            },
            ..default()
        },
    ));
}

fn move_ball(time: Res<Time>, mut balls: Query<(&mut Transform, &Ball)>) {
    let Ok((mut pos, ball)) = balls.get_single_mut() else {
        return;
    };
    pos.translation += ball.0.extend(0.0) * time.delta_seconds();
}

fn collision_detection(
    paddles: Query<&Transform, With<Paddle>>,
    mut balls: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
) {
    let Ok((mut ball, mut velocity)) = balls.get_single_mut() else {
        return;
    };

    if ball.translation.y.abs() + BALL_WIDTH / 2.0 > 250.0 {
        velocity.0.y *= -1.0;
    }

    if ball.translation.x.abs() + BALL_WIDTH / 2.0 > 350.0 {
        ball.translation = Vec3::ZERO;
    }

    for paddle in &paddles {
        if ball.translation.x - BALL_WIDTH / 2.0 < paddle.translation.x + PADDLE_WIDTH / 2.0
            && ball.translation.y - BALL_WIDTH / 2.0 < paddle.translation.y + PADDLE_HEIGHT / 2.0
            && ball.translation.x + BALL_WIDTH / 2.0 > paddle.translation.x - PADDLE_WIDTH / 2.0
            && ball.translation.y + BALL_WIDTH / 2.0 > paddle.translation.y - PADDLE_HEIGHT / 2.0
        {
            velocity.0 *= -1.0;
            velocity.0.y = rand::rng().random_range(-1.0..1.0) * 250.0;
        }
    }
}
