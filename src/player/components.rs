use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    score: u32,
}

#[derive(Component)]
pub struct Bullet {
    speed: f32,
}
