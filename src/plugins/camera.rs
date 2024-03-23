use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::rgb(0., 5., 0.)),
            ..Default::default()
        },
        ..Default::default()
    };
    commands.spawn(camera);
}
