use bevy::prelude::*;

mod camera;
mod enemy;
mod gamestate;
mod player;

use camera::spawn_camera;
use enemy::{enemy_hit, spawn_enemies, SpawnEnemyStatus};
use gamestate::main_menu::{
    despawn_main_menu, interact_with_play_button, interact_with_quit_button, spawn_main_menu,
};
use gamestate::{
    clear_screen, lose_screen, number_of_enemies_check, transition_to_game_state, AppState,
};
use player::{bullet_movement, player_movement, shoot, spawn_player, ShootStatus};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<ShootStatus>()
        .init_state::<SpawnEnemyStatus>()
        .init_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (
                transition_to_game_state,
                interact_with_play_button,
                interact_with_quit_button,
            )
                .run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            Update,
            (
                player_movement,
                shoot,
                bullet_movement,
                spawn_enemies,
                enemy_hit,
            )
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(Last, number_of_enemies_check)
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
        .add_systems(OnEnter(AppState::Game), spawn_player)
        .add_systems(OnEnter(AppState::Lost), (lose_screen, clear_screen))
        .run();
}
