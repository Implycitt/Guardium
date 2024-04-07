use bevy::prelude::*;

use crate::plugins::{
    towers::TowerStats,
    towers::TowerHealth,
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Playing,
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
