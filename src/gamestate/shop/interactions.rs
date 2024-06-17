use super::components::BuyButton;
use crate::gamestate::main_menu::styles::{
    HOVERED_BUTTON_COLOUR, NORMAL_BUTTON_COLOUR, PRESSED_BUTTON_COLOUR,
};
use crate::gamestate::AppState;

use bevy::prelude::*;

pub fn interact_with_buy_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BuyButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_colour)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_colour = PRESSED_BUTTON_COLOUR.into();
                app_state_next_state.set(AppState::Game);
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
