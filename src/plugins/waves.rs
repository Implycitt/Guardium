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
        app.init_resource::<Waves>()
            .init_resource::<WaveState>()
            .add_systems(OnExit(GameState::GameOver), reset);
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

#[derive(Clone, Resource, Debug)]
pub struct Wave {
    pub num: usize,
    pub interval: f32,
    pub delay: f32,
}

impl Default for Wave {
    fn default() -> Self {
        Wave {
            num: 0,
            interval: 3.0,
            delay: 30.0,
        }
    }
}

#[derive(Resource, Debug)]
pub struct WaveState {
    pub delay_timer: Timer,
    pub spawn_timer: Timer,
    pub remaining: usize,
}

impl Default for WaveState {
    fn default() -> Self {
        Self {
            delay_timer: Timer::from_seconds(1.0, TimerMode::Once),
            spawn_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            remaining: 0,
        }
    }
}

impl From<&Wave> for WaveState {
    fn from(value: &Wave) -> Self {
        Self {
            delay_timer: Timer::from_seconds(value.delay, TimerMode::Once),
            spawn_timer: Timer::from_seconds(value.delay, TimerMode::Repeating),
            remaining: value.num,
        }
    }
}

fn reset(
    mut commands: Commands,
    mut waves: ResMut<Waves> 
) {
    waves.current = 0;
}

