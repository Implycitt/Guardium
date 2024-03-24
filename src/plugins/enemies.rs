use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use rand::prelude::*;
use rand::Rng;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies);
    }
} 

#[derive(Component)]
pub struct Enemy {}

pub const NUMBER_OF_ENEMIES: usize = 20;

fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {

        let random_x = rand::thread_rng().gen_range(-window.width()/2.0..window.width()/2.0); 
        let random_y = rand::thread_rng().gen_range(-window.height()/2.0..window.height()/2.0);

        commands.spawn(
            SpriteBundle {
                // enemies spawn outside view sometimes which might be what we want later on since
                // they will move towards the tower.
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball.png"),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(30., 30.)),
                    ..default()
                },
                ..default()
            },
        );
    }
}
