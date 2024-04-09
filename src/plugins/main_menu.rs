use bevy::prelude::*;

use crate::plugins::{
    state::GameState,
    util::cleanup,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Mainmenu), setup_menu)
           .add_systems(OnExit(GameState::MainMenu), despawn_menu);
    }
}

#[derive(Component)]
struct MainMenuItem;

fn setup_menu(
    commands: &mut Commands,
) {
    todo!();
}

fn despawn_menu(
    commands: &mut Commands,
    menu_items: Query<Entity, With<MainMenuItem>>,
) {
    for e in menu_items.iter() {
        commands.entity(e).despawn_recursive();
    }
}

