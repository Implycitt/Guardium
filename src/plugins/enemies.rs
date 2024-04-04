use bevy::{prelude::*, time::common_conditions::on_timer};

use rand::prelude::*;
use rand::Rng;

use std::time::Duration;

use crate::plugins::{
    towers::TowerStats,
    hitpoints::HitPoints,
};

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    pub health: HitPoints,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: HitPoints::default(),
        }
    }
}

pub const NUMBER_OF_ENEMIES: usize = 5;
// small constant between each spawn to see the code in effect. to change.
pub const TIME_BETWEEN_WAVES: f32 = 2.0;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(TIME_BETWEEN_WAVES))),
                update_enemy,
            )
        );
    }
} 

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for _ in 0..NUMBER_OF_ENEMIES {

        // get enemies to spawn in circle around tower: Get a random angle
        let r = 500.0;
        let theta: f32 = rand::thread_rng().gen_range(0.0..360.0);

        // convert polar to rectangular 
        let x: f32 = r * theta.cos();
        let y: f32 = r * theta.sin();

        commands.spawn((
            Enemy::default(),
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/ball.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(30., 30.)),
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn update_enemy(
    player_query: Query<&Transform, With<TowerStats>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<TowerStats>)>
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }
    let player_pos = player_query.single().translation;
    let speed = 0.2;
    for mut transform in enemy_query.iter_mut() {
        let dir = (player_pos - transform.translation).normalize();
        transform.translation += dir * speed;
    }

}
