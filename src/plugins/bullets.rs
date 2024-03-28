use bevy::prelude::*;

use crate::plugins::enemies::Enemy;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_bullets);
    } 
}

#[derive(Component)]
pub struct Bullet;

fn spawn_bullets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    translation: Vec3,
) {
    commands.spawn((
        Bullet,
        SpriteBundle {
            transform: Transform::from_translation(translation),
            texture: asset_server.load("sprites/bullet.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(10., 10.)),
                ..default()
            },
            ..default()
        },
    ));
} 

fn update_bullets(
    mut idk: Commands,
) {
    todo!();
}

