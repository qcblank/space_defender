use bevy::prelude::*;

use super::enemy::{Enemy, EnemySpawnCount};
use crate::gamestate::game_loop::components::ScoreText;
use crate::AppState;

const MAX_ENEMIES: usize = 2;

use super::player::Bullet;
use super::resources::RoundStats;

pub fn number_of_enemies_check(
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut spawn_enemy_count: ResMut<EnemySpawnCount>,
) {
    if **app_state == AppState::Game {
        if enemy_query.iter().len() > MAX_ENEMIES {
            spawn_enemy_count.amount = 0;
            app_state_next_state.set(AppState::Lost)
        }
    }
}

pub fn clear_screen(
    mut commands: Commands,
    bullet_query: Query<Entity, With<Bullet>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    for bullet in bullet_query.iter() {
        commands.entity(bullet).despawn_recursive();
    }
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn_recursive();
    }
}

pub fn spawn_game_loop_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_loop_ui(&mut commands, &asset_server);
}

pub fn despawn_game_loop_ui(
    mut commands: Commands,
    build_game_loop_ui_query: Query<Entity, With<ScoreText>>,
) {
    if let Ok(game_loop_ui_entity) = build_game_loop_ui_query.get_single() {
        commands.entity(game_loop_ui_entity).despawn_recursive();
    }
}

fn build_game_loop_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Round Score: ",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(if cfg!(feature = "default_font") {
                TextStyle {
                    font_size: 60.0,
                    color: Color::GOLD,
                    // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                    ..default()
                }
            } else {
                // "default_font" feature is unavailable, load a font to use instead.
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                }
            }),
        ]),
        ScoreText,
    ));
}

pub fn score_text_update_system(
    mut query: Query<&mut Text, With<ScoreText>>,
    round_stats: Res<RoundStats>,
) {
    for mut text in &mut query {
        text.sections[1].value = format!("{}", round_stats.score);
    }
}
