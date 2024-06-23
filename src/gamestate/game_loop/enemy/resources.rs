use bevy::prelude::*;

#[derive(Resource)]
pub struct EnemySpawnCount {
    pub amount: u64,
}

impl Default for EnemySpawnCount {
    fn default() -> Self {
        Self { amount: 0 }
    }
}
