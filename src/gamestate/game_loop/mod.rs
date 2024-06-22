use bevy::prelude::*;

mod components;
mod game_loop;
mod resources;

use crate::enemy::{enemy_hit, spawn_enemies};
use crate::player::{bullet_movement, player_movement, shoot};
pub use game_loop::{
    clear_screen, despawn_game_loop_ui, number_of_enemies_check, score_text_update_system,
    spawn_game_loop_ui,
};
pub use resources::RoundStats;

use super::AppState;

pub struct GameLoopPlugin;

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_movement,
                shoot,
                bullet_movement,
                spawn_enemies,
                enemy_hit,
                score_text_update_system,
                number_of_enemies_check,
            )
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(Last, number_of_enemies_check)
        .add_systems(OnEnter(AppState::Game), spawn_game_loop_ui)
        .add_systems(OnExit(AppState::Game), (despawn_game_loop_ui, clear_screen));
    }
}
