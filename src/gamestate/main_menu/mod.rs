mod interactions;
pub mod main_menu;
mod styles;

pub use interactions::{interact_with_play_button, interact_with_quit_button};
pub use main_menu::{despawn_main_menu, spawn_main_menu};
