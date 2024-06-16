use crate::enemy::enemy::{Enemy, ENEMY_SIZE};
use crate::player::shoot::Bullet;
use crate::player::Player;
use bevy::prelude::*;

pub fn enemy_hit(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut player_query: Query<&mut Player, With<Player>>,
) {
    let mut player = player_query.get_single_mut().unwrap();

    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let mut bullet_destroyed = false;
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            if bullet_transform
                .translation
                .distance(enemy_transform.translation)
                < ENEMY_SIZE / 2.0
            {
                commands.entity(bullet_entity).despawn_recursive();
                commands.entity(enemy_entity).despawn_recursive();
                bullet_destroyed = true;
                player.increment_score();
                dbg!(player.get_score());
                break;
            }
        }
        if bullet_destroyed {
            break;
        }
    }
}
