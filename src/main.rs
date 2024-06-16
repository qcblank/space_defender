use bevy::prelude::*;

mod camera;
mod enemy;
mod gamestate;
mod player;

use camera::spawn_camera;
use enemy::{enemy_hit, spawn_enemies, SpawnEnemyStatus};
use gamestate::{lose_screen, number_of_enemies_check, transition_to_game_state, AppState};
use player::{bullet_movement, player_movement, shoot, spawn_player, ShootStatus};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<ShootStatus>()
        .init_state::<SpawnEnemyStatus>()
        .init_state::<AppState>()
        .add_systems(Startup, (spawn_player, spawn_camera))
        .add_systems(
            Update,
            (
                player_movement,
                shoot,
                bullet_movement,
                spawn_enemies,
                enemy_hit,
                transition_to_game_state,
            ),
        )
        .add_systems(Last, number_of_enemies_check)
        .add_systems(OnEnter(AppState::Lost), lose_screen)
        .run();
}
