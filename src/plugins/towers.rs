use bevy::prelude::*;

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
                range: 300.0,
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
    asset_server: Res<AssetServer>,
) {
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
            speed: 100.,
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

// somewhere in this code more than one bullet at a time is being spawned
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

            let texture = asset_server.load("sprites/bullet.png");

            spawn_bullets(&mut commands, texture, entity);

        }
    }
}

fn update_bullets(
    mut commands: Commands,
    mut target_query: Query<&Transform, Without<Bullet>>,
    mut query: Query<(Entity, &mut Transform, &mut Bullet)>,
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

        let damage = 50.0;
        if step < dist {
            let dir = (target_pos - bullet_pos).normalize_or_zero();
            transform.translation += (dir * step).extend(0.);
        } else {
            //health.sub(damage);
            continue;
        }
    }
}
