use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

const PLAYER_SPEED: f32 = 500.;
const PLAYER_WIDTH: f32 = 40.;
const PLAYER_HEIGHT: f32 = 60.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<ShootStatus>()
        .init_state::<SpawnEnemyStatus>()
        .add_systems(Startup, (spawn_player, spawn_camera))
        .add_systems(
            Update,
            (
                player_movement,
                shoot,
                bullet_movement,
                spawn_enemies,
                enemy_hit,
            ),
        )
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

#[derive(Component)]
struct Bullet {
    speed: f32,
}

const BULLET_SPEED: f32 = 650.;

const SHOOT_COOLDOWN: u64 = 500;

fn shoot(
    mut commands: Commands,
    shoot_state: Res<State<ShootStatus>>,
    mut shoot_state_next_state: ResMut<NextState<ShootStatus>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, With<Player>>,
) {
    match **shoot_state {
        ShootStatus::Ready => {
            if keyboard_input.pressed(KeyCode::Space) {
                dbg!("pew pew!");

                let player_transform = player_query.get_single().unwrap();

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Rectangle::new(5., 12.5)).into(),
                        material: materials.add(ColorMaterial::from(Color::CRIMSON)),
                        transform: Transform::from_translation(Vec3::new(
                            player_transform.translation.x + PLAYER_WIDTH / 2.0,
                            player_transform.translation.y + PLAYER_HEIGHT / 2.0,
                            0.,
                        )),
                        ..default()
                    },
                    Bullet {
                        speed: BULLET_SPEED,
                    },
                ));
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

fn bullet_movement(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut bullet_query: Query<(Entity, &mut Transform, &Bullet), With<Bullet>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();
    for (entity, mut transform, bullet) in bullet_query.iter_mut() {
        transform.translation += Vec3::Y * bullet.speed * time.delta_seconds();
        if transform.translation.y > window.height() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component)]
struct Enemy {}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum SpawnEnemyStatus {
    #[default]
    Ready,
    Cooldown(Instant),
}

const ENEMY_SPAWN_COOLDOWN: u64 = 2000;
const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.

fn spawn_enemies(
    mut commands: Commands,
    spawn_enemy_state: Res<State<SpawnEnemyStatus>>,
    mut spawn_enemy_state_next_state: ResMut<NextState<SpawnEnemyStatus>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();

    match **spawn_enemy_state {
        SpawnEnemyStatus::Ready => {
            dbg!("boo!");

            let random_x = ENEMY_SIZE / 2.0 + random::<f32>() * (window.width() - ENEMY_SIZE);

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Circle::new(ENEMY_SIZE / 2.)).into(),
                    material: materials.add(ColorMaterial::from(Color::CRIMSON)),
                    transform: Transform::from_translation(Vec3::new(
                        random_x,
                        window.height() * 0.75,
                        0.,
                    )),
                    ..default()
                },
                Enemy {},
            ));

            spawn_enemy_state_next_state.set(SpawnEnemyStatus::Cooldown(Instant::now()))
        }
        SpawnEnemyStatus::Cooldown(last_spawned) => {
            if Instant::now().duration_since(last_spawned)
                > Duration::from_millis(ENEMY_SPAWN_COOLDOWN)
            {
                spawn_enemy_state_next_state.set(SpawnEnemyStatus::Ready)
            }
        }
    }
}

fn enemy_hit(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let mut bullet_destroyed = false;
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            if bullet_transform
                .translation
                .distance(enemy_transform.translation)
                < ENEMY_SIZE / 2.0
            {
                commands.entity(bullet_entity).despawn_recursive();
                commands.entity(enemy_entity).despawn_recursive();
            }
            bullet_destroyed = true;
            break;
        }
        if bullet_destroyed {
            break;
        }
    }
}
