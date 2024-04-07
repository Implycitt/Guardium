use bevy::prelude::*;

use crate::plugins::{
    util::cleanup,
    state::GameState,
};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), init);
        app.add_systems(OnExit(GameState::GameOver), cleanup::<GameOverScene>);
        app.add_systems(Update, menu_button.run_if(in_state(GameState::GameOver)));
    }
}

#[derive(Component)]
struct GameOverScene;

#[derive(Component)]
struct MenuButton;

fn init(
    mut commands: Commands,
) {
    todo!();
}

fn menu_button() {
    todo!();
}
