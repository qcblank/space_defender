use super::components::{BuyButton, PlayerScore};
use crate::gamestate::main_menu::styles::{
    HOVERED_BUTTON_COLOUR, NORMAL_BUTTON_COLOUR, PRESSED_BUTTON_COLOUR,
};
use crate::Player;

use bevy::prelude::*;

pub fn interact_with_buy_button(
    mut player_query: Query<&mut Player, With<Player>>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &BuyButton),
        (Changed<Interaction>, With<BuyButton>),
    >,
    mut player_score_display_query: Query<&mut Text, With<PlayerScore>>,
) {
    let mut player = player_query.get_single_mut().unwrap();
    if let Ok((interaction, mut background_colour, buy_button)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_colour = PRESSED_BUTTON_COLOUR.into();
                let item_price = buy_button.get_item().get_price();
                if player.get_score() >= item_price {
                    dbg!(player.get_score() - item_price);
                    player.decrement_score(item_price);
                    player.decrease_shot_cooldown(25);
                };

                let mut player_score_display = player_score_display_query.get_single_mut().unwrap();

                player_score_display.sections[0].value =
                    format!("Player score: {}", player.get_score());
            }
            Interaction::Hovered => {
                *background_colour = HOVERED_BUTTON_COLOUR.into();
            }
            Interaction::None => {
                *background_colour = NORMAL_BUTTON_COLOUR.into();
            }
        }
    }
}
