/*
    TODO:
    1. SPAWN BALL WHEN RUNNING STATE ENTERS
    2. PAUSE EVERYTHING WHEN PAUSED
    3. ADD WAY TO QUIT GAME
    4. ADD WAY TO SAVE SCORES
    5. COUNTDOWN WHEN PLAYER SCORES, RESPAWNS BALL
    6. ADD MENU + SCORE + COUNTDOWN USER INTERFACE
*/

use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

const PADDLE_SPEED: f32 = 400.0;
const PADDLE_WIDTH: f32 = 30.0;
const PADDLE_HEIGHT: f32 = 180.0;
const BALL_SIZE: f32 = 10.0;
const BALL_SPEED_INCREASE_FACTOR: f32 = 1.25;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(LastScored {player: 1.0})
        .insert_resource(Scores {player1: 0, player2: 0})
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_paddles)
        .add_system(spawn_ball.in_schedule(OnEnter(GameState::Running)).before(ball_bouncing))
        .add_systems(
            (
                move_ball, 
                move_paddles,
                confine_paddle_movement.after(move_paddles),
                ball_bouncing,
            ).in_set(OnUpdate(GameState::Running))
        )
        .add_system(countdown.in_schedule(OnEnter(GameState::Scored)))
        .add_system(execute_that_silly_little_ball)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Scored,
    Paused,
    Menu,
}

#[derive(Resource)]
pub struct LastScored {
    player: f32,
}

#[derive(Resource)]
pub struct Scores {
    player1: u32,
    player2: u32,
}

#[derive(Component)]
pub struct ScheduledForExecution {}

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

pub fn countdown(
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let mut timer = 0.1;

    while timer > 0.0 {
        timer -= time.delta_seconds();
    }

    game_state.set(GameState::Running);
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

    let half_paddle_size = PADDLE_HEIGHT / 2.0;
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
    mut ball_query: Query<(&mut Transform, Entity, &mut Ball), (Without<ScheduledForExecution>, Without<Paddle1>, Without<Paddle2>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    paddle_query1: Query<&Transform, With<Paddle1>>,
    paddle_query2: Query<&Transform, (With<Paddle2>, Without<Paddle1>)>,
    mut scores: ResMut<Scores>,
    mut last_scored: ResMut<LastScored>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let window = window_query.get_single().unwrap();

    let half_ball_size = BALL_SIZE / 2.0;
    let x_min = 0.0 + half_ball_size;
    let x_max = window.width() - half_ball_size;
    let y_min = 0.0 + half_ball_size;
    let y_max = window.height() - half_ball_size;

    if let Ok((mut transform, ball_entity, mut ball)) = ball_query.get_single_mut(){
        let translation = transform.translation;

        if translation.x < x_min {
            transform.translation = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);
            println!("Player 2 Scored!");
            scores.player2 += 1;
            println!("Scores - Player 1: {} - Player 2: {}", scores.player1, scores.player2);
            last_scored.player = 1.0;
            commands.entity(ball_entity).insert(ScheduledForExecution{});
            game_state.set(GameState::Scored);
        }
        if translation.x > x_max {
            transform.translation = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);
            println!("Player 1 Scored!");
            scores.player1 += 1;
            println!("Scores - Player 1: {} - Player 2: {}", scores.player1, scores.player2);
            last_scored.player = -1.0;
            commands.entity(ball_entity).insert(ScheduledForExecution{});
            game_state.set(GameState::Scored);
        }
        if translation.y > y_max || translation.y < y_min {
            ball.direction.y *= -1.0;
        }

        let half_paddle_width = PADDLE_WIDTH / 2.0;
        let half_paddle_height = PADDLE_HEIGHT / 2.0;

        if let Ok(transform1) = paddle_query1.get_single() {
            let translation1 = transform1.translation;
            if (translation.x - half_ball_size <= translation1.x + half_paddle_width && translation.x - half_ball_size >= translation1.x - half_paddle_width)
            && (translation.y + half_ball_size >= translation1.y - half_paddle_height && translation.y - half_ball_size <= translation1.y + half_paddle_height){
                ball.direction.x *= -1.0;
                ball.speed *= BALL_SPEED_INCREASE_FACTOR;
            }
        }
        if let Ok(transform2) = paddle_query2.get_single() {
            let translation2 = transform2.translation;
            if (translation.x + half_ball_size >= translation2.x - half_paddle_width && translation.x + half_ball_size <= translation2.x + half_paddle_width)
            && (translation.y + half_ball_size >= translation2.y - half_paddle_height && translation.y - half_ball_size <= translation2.y + half_paddle_height){
                ball.direction.x *= -1.0;
                ball.speed *= BALL_SPEED_INCREASE_FACTOR;
            }
        }
    }
    /*
    when player scores, despawn ball, put text on screen player scored, countdown, ball spawns again. 
     */
}

pub fn execute_that_silly_little_ball(
    mut commands: Commands,
    ball_query: Query<Entity, (With<Ball>, With<ScheduledForExecution>)>
) {
    if let Ok(ball_entity) = ball_query.get_single() {
        commands.entity(ball_entity).despawn();
    }
}