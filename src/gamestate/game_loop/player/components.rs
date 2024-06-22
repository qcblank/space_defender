use bevy::prelude::*;

const SHOT_COOLDOWN: u64 = 500;

#[derive(Component)]
pub struct Player {
    score: u64,
    shot_cooldown: u64,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            score: 0,
            shot_cooldown: SHOT_COOLDOWN,
        }
    }
}

impl Player {
    pub fn get_score(&self) -> u64 {
        self.score
    }

    pub fn increment_score(&mut self, value: u64) {
        self.score += value;
    }

    pub fn decrement_score(&mut self, value: u64) {
        self.score -= value;
    }

    pub fn get_shot_cooldown(&self) -> u64 {
        self.shot_cooldown
    }

    pub fn decrease_shot_cooldown(&mut self, value: u64) {
        self.shot_cooldown -= value;
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

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

impl AnimationIndices {
    pub fn new(first: usize, last: usize) -> Self {
        Self { first, last }
    }

    pub fn next(&self, current_index: usize) -> usize {
        if current_index == self.last {
            self.first
        } else {
            current_index + 1
        }
    }

    pub fn get_first(&self) -> usize {
        self.first
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
}