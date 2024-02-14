use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

const PADDLE_SPEED: f32 = 400.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(LastScored {player: 1.0})
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_paddles)
        .add_startup_system(spawn_ball)
        .add_system(move_ball)
        .add_system(move_paddles)
        .add_system(confine_paddle_movement.after(move_paddles))
        .add_system(ball_bouncing)
        .run();
}

#[derive(Resource)]
pub struct LastScored {
    player: f32,
}

#[derive(Component)]
pub struct Paddle1 {}

#[derive(Component)]
pub struct Paddle2 {}

#[derive(Component)]
pub struct Ball {
    direction: Vec2,
    speed: f32,
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let w = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(w.width() / 2.0, w.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_paddles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let w = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(w.width() / 20.0, w.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/paddle.png"),
            ..default()
        },
        Paddle1 {},
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(w.width() / 1.05, w.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/paddle.png"),
            ..default()
        },
        Paddle2 {},
    ));
}

pub fn spawn_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    last_scored: Res<LastScored>,
) {
    let w = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(w.width() / 2.0, w.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball.png"),
            ..default()
        },
        Ball {
            direction: Vec2::new(random::<f32>() * last_scored.player, random::<f32>() * 2.0 - 1.0).normalize(), 
            speed: 200.0
        },
    ));
}

pub fn move_ball(
    mut ball_query: Query<(&mut Transform, &Ball)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, ball)) = ball_query.get_single_mut() {
        let direction = Vec3::new(ball.direction.x, ball.direction.y, 0.0);
        transform.translation += direction * ball.speed * time.delta_seconds()
    }
}

pub fn move_paddles(
    keyboard_input: Res<Input<KeyCode>>,
    mut paddle_query1: Query<&mut Transform, With<Paddle1>>,
    mut paddle_query2: Query<&mut Transform, (With<Paddle2>, Without<Paddle1>)>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = paddle_query1.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W){
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S){
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        transform.translation += direction * PADDLE_SPEED * time.delta_seconds();
    }

    if let Ok(mut transform) = paddle_query2.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Up){
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down){
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        transform.translation += direction * PADDLE_SPEED * time.delta_seconds();
    }
}

pub fn confine_paddle_movement(
    mut paddle_query1: Query<&mut Transform, With<Paddle1>>,
    mut paddle_query2: Query<&mut Transform, (With<Paddle2>, Without<Paddle1>)>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    let half_paddle_size = 90.0;
    let y_min = 0.0 + half_paddle_size;
    let y_max = window.height() - half_paddle_size;

    if let Ok(mut transform) = paddle_query1.get_single_mut(){
        let mut translation = transform.translation;

        if translation.y < y_min {
            translation.y = y_min;
        }
        if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }

    if let Ok(mut transform) = paddle_query2.get_single_mut(){
        let mut translation = transform.translation;

        if translation.y < y_min {
            translation.y = y_min;
        }
        if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn ball_bouncing(
    mut ball_query: Query<(&Transform, Entity, &mut Ball)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();

    let half_ball_size = 10.0;
    let x_min = 0.0 + half_ball_size;
    let x_max = window.width() - half_ball_size;
    let y_min = 0.0 + half_ball_size;
    let y_max = window.height() - half_ball_size;

    if let Ok((transform, ball_entity, mut ball)) = ball_query.get_single_mut(){
        let translation = transform.translation;

        if translation.x < x_min {
            println!("Player 2 Scored!");
            commands.entity(ball_entity).despawn();
        }
        if translation.x > x_max {
            println!("Player 1 Scored!");
            commands.entity(ball_entity).despawn();
        }
        if translation.y > y_max || translation.y < y_min {
            ball.direction.y *= -1.0;
        }
    }

    // add ball bouncing off paddles

    /*
    when player scores, depawn ball, put text on screen player scored, countdown, ball spawns again. 
     */
}