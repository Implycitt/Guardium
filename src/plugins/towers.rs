use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
       app.add_systems(Startup, add_tower); 
    }
}

#[derive(Component)]
pub struct Tower {
    level: usize,
    range: f32,
    damage: i32,
    speed: f32,
    pos: Vec2
}

#[derive(Component)]
pub struct Health {
    health: f32,
}

fn add_tower(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let tower = Tower {
        level: 1,
        range: 10.,
        damage: 100,
        speed: 2.,
        pos: Vec2::new(0., 0.), 
    };

    commands.spawn(
        SpriteBundle {
            transform: Transform::from_xyz(tower.pos.x, tower.pos.y, 0.),
            texture: asset_server.load("sprites/tower.png"),
            ..default()
        }
    );
}
