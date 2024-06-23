use bevy::prelude::*;

use super::Explosion;
use crate::gamestate::game_loop::player::{AnimationIndices, AnimationTimer};

pub fn explode(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlas,
        ),
        With<Explosion>,
    >,
) {
    for (explosion_entity, indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = indices.next(atlas.index);
            if atlas.index == indices.first {
                commands.entity(explosion_entity).despawn_recursive();
            }
        }
    }
}
