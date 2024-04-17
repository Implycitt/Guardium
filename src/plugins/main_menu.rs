use bevy::prelude::*;
use bevy::render::camera::*;

use crate::plugins::{
    state::GameState,
    util::cleanup,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
    }
}

