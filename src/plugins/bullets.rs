use bevy::prelude::*;

use crate::plugins::enemies::Enemy;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        todo!();
    } 
}

#[derive(Component)]
pub struct Bullet;

fn spawn_bullets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    translation: Vec3,
) -> Entity {
    todo!();
} 

fn update_bullets(
    mut idk: Commands,
) {
    todo!();
}

