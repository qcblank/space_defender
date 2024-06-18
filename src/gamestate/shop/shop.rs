use super::components::{BuyButton, ShopItem, Upgrade};
use super::styles::SHOP_MENU_STYLE;
use crate::gamestate::main_menu::styles::*;
use crate::gamestate::main_menu::{PlayButton, QuitButton};
use crate::gamestate::shop::components::ShopMenu;

use bevy::prelude::*;

pub fn spawn_shop_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_shop_menu(&mut commands, &asset_server);
}

pub fn despawn_shop_menu(mut commands: Commands, shop_menu_query: Query<Entity, With<ShopMenu>>) {
    if let Ok(shop_menu_entity) = shop_menu_query.get_single() {
        commands.entity(shop_menu_entity).despawn_recursive();
    }
}

pub fn build_shop_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
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

            // ------ Buy firerate Button ------
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOUR.into(),
                        ..default()
                    },
                    BuyButton::with(ShopItem::with(Upgrade::Firerate, 10)),
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
