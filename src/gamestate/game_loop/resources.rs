use bevy::prelude::*;

#[derive(Resource)]
pub struct RoundStats {
    score: u64,
}

impl Default for RoundStats {
    fn default() -> Self {
        Self { score: 0 }
    }
}

impl RoundStats {
    pub fn increment_score(&mut self) {
        self.score += 1;
    }

    pub fn get_score(&self) -> u64 {
        self.score
    }

    pub fn reset_score(&mut self) {
        self.score = 0
    }
}
