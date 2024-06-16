use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;

const PLAYER_SPEED: f32 = 500.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<ShootStatus>()
        .add_systems(Startup, (spawn_player, spawn_camera))
        .add_systems(Update, (player_movement, shoot))
        .run();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 10.0),
        ..default()
    });
}

#[derive(Component)]
struct Player {}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Triangle2d::new(
                    Vec2::new(0., 0.),
                    Vec2::new(20., 60.),
                    Vec2::new(40., 0.),
                ))
                .into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_xyz(window.width() / 2., 50., 0.1),
            ..default()
        },
        Player {},
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum ShootStatus {
    #[default]
    Ready,
    Fired(Instant),
}

const SHOOT_COOLDOWN: u64 = 2000;

fn shoot(
    shoot_state: Res<State<ShootStatus>>,
    mut shoot_state_next_state: ResMut<NextState<ShootStatus>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut _player_query: Query<&mut Transform, With<Player>>,
) {
    match **shoot_state {
        ShootStatus::Ready => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                dbg!("pew pew!");
                shoot_state_next_state.set(ShootStatus::Fired(Instant::now()))
            }
        }
        ShootStatus::Fired(time_fired) => {
            if Instant::now().duration_since(time_fired) > Duration::from_millis(SHOOT_COOLDOWN) {
                shoot_state_next_state.set(ShootStatus::Ready)
            }
        }
    }
}
