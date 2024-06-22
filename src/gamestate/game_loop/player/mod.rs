pub mod components;
pub mod movement;
pub mod player;
pub mod shoot;

pub use components::{Bullet, Player};
pub use movement::player_movement;
pub use player::{spawn_player, PLAYER_HEIGHT, PLAYER_SPEED, PLAYER_WIDTH};
pub use shoot::{bullet_movement, shoot, ShootStatus};
