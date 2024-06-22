use bevy::prelude::*;

pub mod components;
mod interactions;
pub mod main_menu;
pub mod styles;

pub use components::{PlayButton, QuitButton};
pub use interactions::{interact_with_play_button, interact_with_quit_button};
pub use main_menu::{despawn_main_menu, spawn_main_menu};

use super::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (interact_with_play_button, interact_with_quit_button)
                .run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
