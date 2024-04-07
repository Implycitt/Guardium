use bevy::prelude::*;

use crate::plugins::{
    state::GameState,
    util::cleanup,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), (setup_menu, init_background))
            .add_systems(Update, play_button.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), (cleanup::<MainMenuScene>, cleanup_background));
    }
}

#[derive(Component)]
struct MainMenuScene;

#[derive(Component)]
struct PlayButton;

fn setup_menu() {
    todo!();
}

fn play_button() {
    todo!();
}

fn init_background() {
    todo!();
}

fn cleanup_background() {
    todo!();
}

