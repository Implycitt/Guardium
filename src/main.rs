use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    // Circle mesh
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(10.)).into(),
        // 4. Put something bright in a dark environment to see the effect
        material: materials.add(Color::rgb(7.5, 0.0, 7.5)),
        // transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
        // the idea of the line above is just to move or transform the Circle
        // to the left 200 pixels
        ..default()
    });
}

fn move_player(keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::W) {
        // move player, todo tomorrow.
    }
}
