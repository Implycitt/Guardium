use std::f32::consts::PI;

use bevy::{
    prelude::*,
    math::{vec2, vec3},
    utils::Instant,
};

use crate::plugins::{
    enemies::Enemy,
    state::GameState,
    resources::CursorPosition,
};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
       app.add_event::<PlayerEnemyCollisionEvent>()
            .add_systems(Startup, add_tower.run_if(in_state(GameState::Playing)))
            .add_systems(Update, (manual_shoot_enemies, manual_update_bullets, despawn_bullets, handle_collision_events)
                .run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct TowerStats {
    level: usize,
    range: f32,
    damage: u32,
    speed: f32,
    pos: Vec2
}

#[derive(Component)]
pub struct TowerState {
    pub timer: Timer,
}

#[derive(Component)]
pub struct TowerHealth {
    pub health: i32,
}

#[derive(Bundle)]
pub struct TowerBundle {
    pub stats: TowerStats,
    pub state: TowerState,
    pub health: TowerHealth,
}

#[derive(Component)]
pub struct Bullet {
    speed: f32,
    target: Entity,
    damage: i32,
}

#[derive(Component)]
pub struct ManualBullet;

#[derive(Component)]
struct BulletDirection(Vec3);

#[derive(Component)]
struct SpawnInstant(Instant);

#[derive(Event)]
pub struct PlayerEnemyCollisionEvent;

// works however need to add a timer between attacks
fn handle_collision_events(
    mut tower_query: Query<&mut TowerHealth, With<TowerHealth>>,
    mut events: EventReader<PlayerEnemyCollisionEvent>,
) {
    if tower_query.is_empty() {
        return;
    }

    let mut health = tower_query.single_mut();
    for _ in events.read() {
        health.health -= 50;
    }
}

impl TowerBundle {
    pub fn new() -> Self {
        Self {
            stats: TowerStats {
                level: 1,
                range: 500.0,
                damage: 10,
                speed: 10.0,
                pos: Vec2::new(0., 0.,),
            },
            state: TowerState {
                timer: Timer::from_seconds(0.5, TimerMode::Repeating)
            },
            health: TowerHealth {
                health: 1000,
            }
        }
    }
}

fn add_tower(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            // tower asset not loading for now
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
            speed: 1000.,
            damage: 5,
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
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    time: Res<Time>,
) {
    for (transform, mut tower_state, tower_stats) in tower_query.iter_mut() {

        tower_state.timer.tick(time.delta());

        if !tower_state.timer.finished() {
            continue;
        }

        let range_squared = tower_stats.range * tower_stats.range;
        let pos = transform.translation.truncate();

        for (entity, enemy_transform) in &enemy_query {
            let enemy_pos = enemy_transform.translation.truncate();
            if pos.distance_squared(enemy_pos) > range_squared {
                continue;
            } 
            // if you remove the break below you have a tower that shoots
            // all enemies; this could be used in the future ?
            let texture = asset_server.load("sprites/bullet.png");
            spawn_bullets(&mut commands, texture, entity);
            break; 
        }
    }
}

fn update_bullets(
    mut commands: Commands,
    mut target_query: Query<(&mut Enemy, &Transform), (With<Enemy>, Without<Bullet>)>,
    mut query: Query<(Entity, &mut Transform, &mut Bullet)>,
    time: Res<Time>
) {
    for (mut entity, mut transform, mut bullet) in query.iter_mut() {
        let Ok((mut enemy, target_transform)) = target_query.get_mut(bullet.target)
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
        } else {
            enemy.health -= bullet.damage;
        }
    }
}

fn manual_shoot_enemies(
    mut commands: Commands,
    mut tower_query: Query<(&Transform, &mut TowerState), With<TowerStats>>,
    asset_server: Res<AssetServer>,
    cursor_pos: Res<CursorPosition>,
    time: Res<Time>,
) {
    if tower_query.is_empty() {
        return;
    }

    let (tower_transform, mut tower_timer) = tower_query.single_mut();
    let tower_pos = tower_transform.translation.truncate();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => tower_pos,
    };

    tower_timer.timer.tick(time.delta());

    if tower_timer.timer.elapsed_secs() >= 0.3 {
        tower_timer.timer.reset();

        let dir = Vec3::new(cursor_pos.x, cursor_pos.y, 0.0);

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/bullet.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(10., 10.)),
                    ..default()
                },
                transform: Transform::from_translation(vec3(0.0, 10.0, 0.0)),
                ..default()
            },
            ManualBullet,
            BulletDirection(dir),
            SpawnInstant(Instant::now()),
        ));
    }
}

fn manual_update_bullets(
    mut bullet_query: Query<(&mut Transform, &BulletDirection), With<ManualBullet>> 
) {
    if bullet_query.is_empty() {
        return;
    }

    for (mut t, dir) in bullet_query.iter_mut() {
        t.translation += dir.0.normalize() * Vec3::splat(5.0);
    }
}

fn despawn_bullets(
    mut commands: Commands,
    bullet_query: Query<(&SpawnInstant, Entity), With<ManualBullet>>,
) {
    for (instant, e) in bullet_query.iter() {
        if instant.0.elapsed().as_secs_f32() > 2.0 {
            commands.entity(e).despawn();
        }
    }
}

