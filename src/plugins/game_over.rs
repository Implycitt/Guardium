use bevy::prelude::*;
// use bevy::ui::prelude::*;

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
}

