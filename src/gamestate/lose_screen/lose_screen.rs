use super::components::{LoseScreen, ShopButton};
use crate::gamestate::game_loop::RoundStats;
use crate::gamestate::main_menu::styles::*;
use crate::Player;

use bevy::prelude::*;

pub fn update_player_score(
    mut player_query: Query<&mut Player, With<Player>>,
    mut round_stats: ResMut<RoundStats>,
) {
    let mut player = player_query.get_single_mut().unwrap();
    player.increment_score(round_stats.get_score());
    round_stats.reset_score();
}

pub fn spawn_lose_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Player, With<Player>>,
) {
    let player = player_query.get_single().unwrap();
    build_lose_screen(&mut commands, &asset_server, player.get_score());
}

pub fn despawn_lose_screen(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<LoseScreen>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_lose_screen(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    score: u64,
) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            LoseScreen {},
        ))
        .with_children(|parent| {
            // ------ Title ------
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                format!("You Lose!\nScore: {}", score),
                                get_title_text_style(&asset_server),
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });

            // ------ Shop Button ------
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOUR.into(),
                        ..default()
                    },
                    ShopButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Shop",
                                get_button_text_style(&asset_server),
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_menu_entity
}
