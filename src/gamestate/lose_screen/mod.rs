use bevy::prelude::*;

mod components;
mod interactions;
mod lose_screen;

pub use interactions::interact_with_shop_button;
pub use lose_screen::{despawn_lose_screen, spawn_lose_screen, update_player_score};

use super::AppState;

pub struct LoseScreenPlugin;

impl Plugin for LoseScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (interact_with_shop_button).run_if(in_state(AppState::Lost)),
        )
        .add_systems(
            OnEnter(AppState::Lost),
            (spawn_lose_screen, update_player_score),
        )
        .add_systems(OnExit(AppState::Lost), despawn_lose_screen);
    }
}
