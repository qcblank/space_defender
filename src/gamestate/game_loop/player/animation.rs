use bevy::prelude::*;

use super::{
    components::{AnimationIndices, AnimationTimer},
    Player,
};

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas), With<Player>>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = indices.next(atlas.index);
        }
    }
}
