use bevy::prelude::*;

mod camera;
mod enemy;
mod gamestate;
mod player;

use camera::spawn_camera;
use enemy::{enemy_hit, spawn_enemies, SpawnEnemyStatus};
use gamestate::lose_screen::{despawn_lose_screen, interact_with_shop_button, spawn_lose_screen};
use gamestate::main_menu::{
    despawn_main_menu, interact_with_play_button, interact_with_quit_button, spawn_main_menu,
};
use gamestate::shop::{despawn_shop_menu, interact_with_buy_button, spawn_shop_menu};
use gamestate::{clear_screen, number_of_enemies_check, transition_to_game_state, AppState};
use player::{bullet_movement, player_movement, shoot, spawn_player, ShootStatus};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<ShootStatus>()
        .init_state::<SpawnEnemyStatus>()
        .init_state::<AppState>()
        .add_systems(Startup, (spawn_camera, spawn_player))
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
        .add_systems(
            Update,
            (interact_with_shop_button).run_if(in_state(AppState::Lost)),
        )
        .add_systems(
            Update,
            (
                interact_with_buy_button,
                interact_with_play_button,
                interact_with_quit_button,
            )
                .run_if(in_state(AppState::Shop)),
        )
        .add_systems(Last, number_of_enemies_check)
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
        .add_systems(OnEnter(AppState::Lost), (spawn_lose_screen, clear_screen))
        .add_systems(OnExit(AppState::Lost), despawn_lose_screen)
        .add_systems(OnEnter(AppState::Shop), spawn_shop_menu)
        .add_systems(OnExit(AppState::Shop), despawn_shop_menu)
        .run();
}
