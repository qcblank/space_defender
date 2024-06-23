use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::{AnimationIndices, AnimationTimer};
use super::Player;

pub const PLAYER_SPEED: f32 = 500.;
pub const PLAYER_SPRITE_WIDTH: f32 = 64.;
pub const PLAYER_SPRITE_HEIGHT: f32 = 64.;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let window = window_query.get_single().unwrap();
    let texture = asset_server.load("sprites/player.png");
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(PLAYER_SPRITE_WIDTH, PLAYER_SPRITE_HEIGHT),
        3,
        1,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices::new(0, 2);
    commands.spawn((
        (
            SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform::from_xyz(window.width() / 2., 60., 0.1),
                ..default()
            },
            animation_indices,
            AnimationTimer::new(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ),
        Player::default(),
    ));
}
