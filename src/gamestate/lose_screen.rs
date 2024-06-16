use crate::enemy::Enemy;
use crate::player::Player;
use crate::AppState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const MAX_ENEMIES: usize = 7;

#[derive(Component)]
struct LoseText;

pub fn number_of_enemies_check(
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if **app_state == AppState::Game {
        if enemy_query.iter().len() > MAX_ENEMIES {
            dbg!("You lose!");
            app_state_next_state.set(AppState::Lost)
        }
    }
}

pub fn lose_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Player, With<Player>>,
) {
    let window = window_query.get_single().unwrap();
    let player = player_query.get_single().unwrap();

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            format!("You Lose!\nScore: {}", player.get_score()),
            TextStyle {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 100.0,
                ..default()
            },
        ) // Set the justification of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(window.height() / 2.0),
            right: Val::Px(window.width() / 2.0),
            ..default()
        }),
        LoseText,
    ));
}
