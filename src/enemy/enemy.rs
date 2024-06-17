use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::Enemy;
use crate::gamestate::AppState;

pub const ENEMY_SIZE: f32 = 64.0; // This is the enemy sprite size.

const ENEMY_SPAWN_COOLDOWN: u64 = 2000;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SpawnEnemyStatus {
    #[default]
    Ready,
    Cooldown(Instant),
}

pub fn spawn_enemies(
    mut commands: Commands,
    app_state: Res<State<AppState>>,
    spawn_enemy_state: Res<State<SpawnEnemyStatus>>,
    mut spawn_enemy_state_next_state: ResMut<NextState<SpawnEnemyStatus>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();

    if **app_state == AppState::Game {
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
}
