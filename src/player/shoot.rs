use std::time::{Duration, Instant};

use super::{Player, PLAYER_HEIGHT, PLAYER_WIDTH};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;

const BULLET_SPEED: f32 = 650.;
const SHOOT_COOLDOWN: u64 = 500;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ShootStatus {
    #[default]
    Ready,
    Fired(Instant),
}

#[derive(Component)]
pub struct Bullet {
    speed: f32,
}

pub fn shoot(
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

pub fn bullet_movement(
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
