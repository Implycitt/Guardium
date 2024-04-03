use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::plugins::enemies::Enemy;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(Startup, add_tower);
       app.add_systems(Update, (shoot_enemies, update_bullets));
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

#[derive(Component)]
pub struct Bullet {
    speed: f32,
    target: Entity,
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

pub fn spawn_bullets(
    commands: &mut Commands,
    tex: Handle<Image>,
    targ: Entity,
) {
    commands.spawn((
        Bullet{
            target: targ,
            speed: 30.,
        },
        SpriteBundle {
            texture: tex,
            sprite: Sprite {
                custom_size: Some(Vec2::new(10., 10.)),
                ..default()
            },
            ..default()
        },
    ));
} 

fn shoot_enemies(
    mut commands: Commands,
    mut tower_query: Query<(&Transform, &mut TowerState, &TowerStats)>,
    asset_server: Res<AssetServer>,
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
            .filter(|enemy_transform| {
                let dist = enemy_transform.translation.truncate().distance(transform.translation.truncate());
                dist <= tower_stats.range
            });

        if let Some(enemy) = in_range.next() {
           let mut bullet_translation = transform.translation; 
        }

        let enemy = in_range.next();

        let texture = asset_server.load("sprites/bullet.png");

        spawn_bullets(&mut commands, texture, enemy);
    }
}

fn update_bullets(
    mut commands: Commands,
    target_query: Query<&Transform, Without<Bullet>>,
    query: Query<(Entity, &mut Transform, &mut Bullet)>,
    time: Res<Time>
) {
    for (entity, mut transform, mut bullet) in query.iter_mut() {
        let Ok(target_transform) = target_query.get_mut(bullet.target)
        else {
            commands.entity(entity).despawn_recursive();
            continue;
        };

        let target_pos = target_transform.translation.truncate();
        let bullet_pos = transform.translation.truncate();

        let dist = bullet_pos.distance(target_pos);

        let delta = time.delta_seconds();
        let step = bullet.speed * delta;

        if step < dist {
            let dir = (target_pos - bullet_pos).normalize_or_zero();
            transform.translation += (dir * step).extend(0.);

            continue;
        }
    }
}
