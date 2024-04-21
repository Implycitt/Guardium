use bevy::{
    prelude::*,
    time::common_conditions::on_timer
};

use rand::prelude::*;
use rand::Rng;

use std::time::Duration;

use crate::plugins::{
    towers::TowerStats,
    towers::TowerHealth,
    state::GameState,
    waves::{ 
        WavePlugin,
        Waves,
        WaveState,
    }
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(TIME_BETWEEN_WAVES))),
                update_enemy,
                check_death,
            ).run_if(in_state(GameState::Playing)),
        );
    }
} 

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: 50,
        }
    }
}

#[derive(Component, Default)]
pub enum EnemyState {
    #[default]
    Moving,
    Attacking,
}

#[derive(Component)]
pub struct AttackTimer {
    pub timer: Timer,
}

impl Default for AttackTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating)
        }
    }
}

pub struct EnemyBundle {
    enemy: Enemy,
    timer: AttackTimer,
}

impl EnemyBundle {
    pub fn new() -> Self {
        Self {
            enemy: Enemy::default(),
            timer: AttackTimer::default(),
        }
    }
}

// this stuff is going to be shoved into the waves module
pub const NUMBER_OF_ENEMIES: usize = 10;
// small constant between each spawn to see the code in effect. to change.
pub const TIME_BETWEEN_WAVES: f32 = 5.0;

fn spawn_enemies(
    mut wave_state: ResMut<WaveState>,
    mut waves: ResMut<Waves>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let Some(current_wave) = waves.current() else {
        println!("Not working");
        return;
    };

    println!("{:?}", waves.current());
    for _ in 0..NUMBER_OF_ENEMIES {

        // get enemies to spawn in circle around tower: Get a random angle
        let r = 500.0;
        let theta: f32 = rand::thread_rng().gen_range(0.0..360.0);

        // convert polar to rectangular 
        let x: f32 = r * theta.cos();
        let y: f32 = r * theta.sin();

        commands.spawn((
            Enemy {
                health: 50,
            },
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
    let speed = 0.8;
    for mut transform in enemy_query.iter_mut() {

        let dir = (player_pos - transform.translation).normalize();

        transform.translation += dir * speed;

    }

}

fn check_death(
    enemy_query: Query<(Entity, &Enemy), With<Enemy>>,
    mut commands: Commands,
) {
    for (entity, enemy) in &enemy_query {
        if !enemy.health <= 0 {
            continue;
        }
        commands.entity(entity).despawn();
    }
}

