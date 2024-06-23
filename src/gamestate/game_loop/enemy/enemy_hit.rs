use super::{Enemy, Explosion, ENEMY_SIZE};
use crate::{
    gamestate::game_loop::{
        player::{AnimationIndices, AnimationTimer},
        RoundStats,
    },
    Bullet,
};

use bevy::prelude::*;

const EXPLOSION_SPRITE_WIDTH: f32 = 64.;
const EXPLOSION_SPRITE_HEIGHT: f32 = 64.;

pub fn enemy_hit(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut round_stats: ResMut<RoundStats>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let mut bullet_destroyed = false;
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            if bullet_transform
                .translation
                .distance(enemy_transform.translation)
                < ENEMY_SIZE / 2.0
            {
                commands.entity(bullet_entity).despawn_recursive();

                {
                    let texture = asset_server.load("sprites/explosion.png");
                    let layout = TextureAtlasLayout::from_grid(
                        Vec2::new(EXPLOSION_SPRITE_WIDTH, EXPLOSION_SPRITE_HEIGHT),
                        4,
                        1,
                        None,
                        None,
                    );
                    let texture_atlas_layout = texture_atlas_layouts.add(layout);
                    let animation_indices = AnimationIndices::new(0, 3);
                    commands.spawn((
                        (
                            SpriteSheetBundle {
                                texture,
                                atlas: TextureAtlas {
                                    layout: texture_atlas_layout,
                                    index: animation_indices.first,
                                },
                                transform: Transform::from_xyz(
                                    enemy_transform.translation.x,
                                    enemy_transform.translation.y,
                                    0.2,
                                ),
                                ..default()
                            },
                            animation_indices,
                            AnimationTimer::new(Timer::from_seconds(0.1, TimerMode::Repeating)),
                        ),
                        Explosion,
                    ));
                }

                commands.entity(enemy_entity).despawn_recursive();
                bullet_destroyed = true;
                round_stats.score += 1;
                break;
            }
        }
        if bullet_destroyed {
            break;
        }
    }
}
