use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    score: u32,
}

impl Player {
    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }

    pub fn decrement_score(&mut self, value: u32) {
        self.score -= value;
    }
}

#[derive(Component)]
pub struct Bullet {
    speed: f32,
}

impl Bullet {
    pub fn with_speed(speed: f32) -> Self {
        Bullet { speed }
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }
}
