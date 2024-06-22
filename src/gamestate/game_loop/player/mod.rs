pub mod animation;
pub mod components;
pub mod movement;
pub mod player;
pub mod shoot;

pub use animation::animate_sprite;
pub use components::{Bullet, Player};
pub use movement::player_movement;
pub use player::{spawn_player, PLAYER_SPEED, PLAYER_SPRITE_HEIGHT, PLAYER_SPRITE_WIDTH};
pub use shoot::{bullet_movement, shoot, ShootStatus};