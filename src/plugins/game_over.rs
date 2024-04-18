use bevy::prelude::*;

use crate::plugins::{
    util::cleanup,
    state::GameState,
};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup);
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct ColorText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "hello\nbevy!",
            TextStyle {
                font_size: 100.0,
                ..default()
            },
        ) 
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        ColorText,
    ));
}

