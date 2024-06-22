use std::time::{Duration, Instant};

use super::{Bullet, Player, PLAYER_SPRITE_HEIGHT};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;

const BULLET_SPEED: f32 = 650.;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ShootStatus {
    #[default]
    Ready,
    Fired(Instant),
}

pub fn shoot(
    mut commands: Commands,
    shoot_state: Res<State<ShootStatus>>,
    mut shoot_state_next_state: ResMut<NextState<ShootStatus>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<(&Player, &Transform), With<Player>>,
) {
    let (player, player_transform) = player_query.get_single().unwrap();
    match **shoot_state {
        ShootStatus::Ready => {
            if keyboard_input.pressed(KeyCode::Space) {
                dbg!("pew pew!");

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Rectangle::new(5., 12.5)).into(),
                        material: materials.add(ColorMaterial::from(Color::CRIMSON)),
                        transform: Transform::from_translation(Vec3::new(
                            player_transform.translation.x,
                            player_transform.translation.y + PLAYER_SPRITE_HEIGHT / 2.0,
                            0.,
                        )),
                        ..default()
                    },
                    Bullet::with_speed(BULLET_SPEED),
                ));
                shoot_state_next_state.set(ShootStatus::Fired(Instant::now()))
            }
        }
        ShootStatus::Fired(time_fired) => {
            if Instant::now().duration_since(time_fired)
                > Duration::from_millis(player.get_shot_cooldown())
            {
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
        transform.translation += Vec3::Y * bullet.get_speed() * time.delta_seconds();
        if transform.translation.y > window.height() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
