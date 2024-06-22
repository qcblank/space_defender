use super::components::{BuyButton, PlayerScore, ShopItem, Upgrade};
use super::styles::SHOP_MENU_STYLE;
use crate::gamestate::main_menu::styles::*;
use crate::gamestate::main_menu::{PlayButton, QuitButton};
use crate::gamestate::shop::components::ShopMenu;
use crate::Player;

use bevy::prelude::*;

const SHOT_COOLDOWN_PRICE: u64 = 2;

pub fn spawn_shop_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Player, With<Player>>,
) {
    let player = player_query.get_single().unwrap();
    build_shop_menu(&mut commands, &asset_server, player.get_score());
}

pub fn despawn_shop_menu(mut commands: Commands, shop_menu_query: Query<Entity, With<ShopMenu>>) {
    if let Ok(shop_menu_entity) = shop_menu_query.get_single() {
        commands.entity(shop_menu_entity).despawn_recursive();
    }
}

pub fn build_shop_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_score: u64,
) -> Entity {
    let shop_menu_entity = commands
        .spawn((
            NodeBundle {
                style: SHOP_MENU_STYLE,
                ..default()
            },
            ShopMenu {},
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
                                "-- Shop --",
                                get_title_text_style(&asset_server),
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });

            // ------ Player Score ------
            parent
                .spawn(NodeBundle {
                    style: LABEL_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Text
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    format!("Player score: {}", player_score),
                                    get_label_text_style(&asset_server),
                                )],
                                ..default()
                            },
                            ..default()
                        },
                        PlayerScore {},
                    ));
                });

            // ------ Buy firerate Button ------
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOUR.into(),
                        ..default()
                    },
                    BuyButton::with(ShopItem::with(Upgrade::Firerate, SHOT_COOLDOWN_PRICE)),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "firerate",
                                get_button_text_style(&asset_server),
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });

            // ------ Play Button ------
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOUR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                get_button_text_style(&asset_server),
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });

            // ------ Quit Button ------
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOUR.into(),
                        ..default()
                    },
                    QuitButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                get_button_text_style(&asset_server),
                            )],
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    shop_menu_entity
}
