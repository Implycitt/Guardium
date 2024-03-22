use bevy::prelude::*;

#[derive(Component)]
struct tower {
    level: usize,
    range: f32,
    damage: i32,
    speed: f32,
    health: f32,
    coordinate: Coordinate,
}
