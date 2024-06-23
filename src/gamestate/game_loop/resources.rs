use bevy::prelude::*;

#[derive(Resource)]
pub struct RoundStats {
    pub score: u64,
}

impl Default for RoundStats {
    fn default() -> Self {
        Self { score: 0 }
    }
}
