pub mod components;
mod interactions;
pub mod main_menu;
pub mod styles;

pub use components::{PlayButton, QuitButton};
pub use interactions::{interact_with_play_button, interact_with_quit_button};
pub use main_menu::{despawn_main_menu, spawn_main_menu};
