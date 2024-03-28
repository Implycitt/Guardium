use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::plugins::enemies::Enemy;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(Startup, add_tower); 
    }
}

#[derive(Component)]
pub struct TowerStats {
    level: usize,
    range: f32,
    damage: i32,
    speed: f32,
    pos: Vec2
}

#[derive(Component)]
pub struct TowerState {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Health {
    health: f32,
}

#[derive(Bundle)]
pub struct TowerBundle {
    pub stats: TowerStats,
    pub state: TowerState,
    pub health: Health,
}

impl TowerBundle {
    pub fn new() -> Self {
        Self {
            stats: TowerStats {
                level: 1,
                range: 100.0,
                damage: 10,
                speed: 10.0,
                pos: Vec2::new(0., 0.,),
            },
            state: TowerState {
                timer: Timer::from_seconds(1., TimerMode::Repeating)
            },
            health: Health {
                health: 1000.,
            }
        }
    }
}

fn add_tower(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            texture: asset_server.load("sprites/tower.png"),
            ..default()
        },
        TowerBundle::new(),
    ));
}

fn shoot_enemies(
    mut commands: Commands,
    mut tower_query: Query<(&Transform, &mut TowerState, &TowerStats)>,
    enemy_query: Query<&Transform, With<Enemy>>,
    time: Res<Time>,
) {
    for (transform, mut tower_state, tower_stats) in tower_query.iter_mut() {
        tower_state.timer.tick(time.delta());
        if !tower_state.timer.finished() {
            continue;
        }

        let mut in_range = enemy_query
            .iter()
            .filter(|(_, _, enemy_transform)| {
                let dist = enemy_transform
                    .translation
                    .truncate()
                    .distance(transform.translation.truncate());
                dist <= tower_stats.range
            });

        if let Some((enemy, _, _,)) = in_range.next() {
            let mut bullet_translation = transform.translation;
            // im not sure what this is supposed to do at all.
            bullet_translation.y += 24.0; 

            bullets.spawn_bullets(
                bullet_translation,
                enemy,
            )
        }
    }
}
