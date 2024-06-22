use bevy::prelude::*;

mod camera;
mod gamestate;

use camera::spawn_camera;
use gamestate::game_loop::enemy::{EnemySpawnCount, SpawnEnemyStatus};
use gamestate::game_loop::player::{spawn_player, Bullet, Player, ShootStatus};
use gamestate::game_loop::{GameLoopPlugin, RoundStats};
use gamestate::lose_screen::LoseScreenPlugin;
use gamestate::main_menu::MainMenuPlugin;
use gamestate::shop::ShopPlugin;
use gamestate::AppState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MainMenuPlugin)
        .add_plugins(ShopPlugin)
        .add_plugins(GameLoopPlugin)
        .add_plugins(LoseScreenPlugin)
        .init_state::<ShootStatus>()
        .init_state::<SpawnEnemyStatus>()
        .init_state::<AppState>()
        .init_resource::<EnemySpawnCount>()
        .init_resource::<RoundStats>()
        .add_systems(Startup, (spawn_camera, spawn_player))
        .run();
}
