use bevy::prelude::*;

use crate::plugins::{
    towers::TowerStats,
    towers::TowerHealth,
};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_loss.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
    MainMenu,
    Loading,
    GameOver,
}

fn check_loss(
    mut towers: Query<&TowerHealth>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for health in towers.iter_mut() {
        if health.health <= 0 {
            next_state.set(GameState::GameOver);
        }
    }
}
