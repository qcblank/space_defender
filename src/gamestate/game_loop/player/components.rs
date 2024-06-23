use bevy::prelude::*;

const SHOT_COOLDOWN: u64 = 500;

#[derive(Component)]
pub struct Player {
    pub score: u64,
    pub shot_cooldown: u64,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            score: 0,
            shot_cooldown: SHOT_COOLDOWN,
        }
    }
}

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
}

impl Bullet {
    pub fn with_speed(speed: f32) -> Self {
        Bullet { speed }
    }
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
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
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    pub fn new(timer: Timer) -> Self {
        Self(timer)
    }
}
