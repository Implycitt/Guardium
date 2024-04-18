use bevy::{
    prelude::*,
    utils::HashMap,
};

use crate::plugins::{
    enemies::EnemyBundle,
    state::GameState,
};

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
    }
}

#[derive(Resource, Default)]
pub struct Waves {
    pub waves: Vec<Wave>,
    pub current: usize,
}

impl Waves {
    pub fn current(&self) -> Option<&Wave> {
        self.waves.get(self.current)
    }
    pub fn advance(&mut self) -> Option<&Wave> {
        self.current += 1;
        self.current()
    }
}

#[derive(Clone)]
pub struct Wave {
    pub num: usize,
    pub interval: f32,
    pub delay: f32,
}

#[derive(Component)]
pub struct WaveState {
    pub delay_timer: Timer,
    pub spawn_timer: Timer,
    pub remaining: usize,
}
