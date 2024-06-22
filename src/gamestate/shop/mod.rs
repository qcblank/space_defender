use bevy::prelude::*;

pub mod components;
pub mod interactions;
pub mod shop;
mod styles;

use crate::AppState;
pub use interactions::interact_with_buy_button;
pub use shop::{despawn_shop_menu, spawn_shop_menu};

use super::main_menu::{interact_with_play_button, interact_with_quit_button};

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app
            // Button interactions
            .add_systems(
                Update,
                (
                    interact_with_buy_button,
                    interact_with_play_button,
                    interact_with_quit_button,
                ),
            )
            // OnEnter State Systems
            .add_systems(OnEnter(AppState::Shop), spawn_shop_menu)
            // OnExit State Systems
            .add_systems(OnExit(AppState::Shop), despawn_shop_menu);
    }
}
