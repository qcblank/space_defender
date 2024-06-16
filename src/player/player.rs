use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;

pub const PLAYER_SPEED: f32 = 500.;
pub const PLAYER_WIDTH: f32 = 40.;
pub const PLAYER_HEIGHT: f32 = 60.;

#[derive(Component)]
pub struct Player {
    score: u32,
}

impl Player {
    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn increment_score(&mut self) {
        self.score += 1;
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Triangle2d::new(
                    Vec2::new(0., 0.),
                    Vec2::new(20., 60.),
                    Vec2::new(40., 0.),
                ))
                .into(),
            material: materials.add(ColorMaterial::from(Color::GREEN)),
            transform: Transform::from_xyz(window.width() / 2., 50., 0.1),
            ..default()
        },
        Player { score: 0 },
    ));
}
