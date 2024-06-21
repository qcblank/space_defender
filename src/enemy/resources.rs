use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemySpawnCount {
    amount: u64,
}

impl Default for EnemySpawnCount {
    fn default() -> Self {
        Self { amount: 0 }
    }
}

impl EnemySpawnCount {
    pub fn increment(&mut self) {
        self.amount += 1;
    }

    pub fn get_count(&mut self) -> u64 {
        self.amount
    }

    pub fn reset(&mut self) {
        self.amount = 0
    }
}
