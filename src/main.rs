use bevy::prelude::*;
use bevy::window::*;

mod plugins;
use plugins::{
    camera::CameraPlugin,
    enemies::EnemyPlugin,
    towers::TowerPlugin,
};

fn main() {
    App::new()
        .add_plugins((CameraPlugin, EnemyPlugin, TowerPlugin))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "CSPFinal".into(),
                        resolution: (1920.0, 1080.0).into(),
                        // mode: WindowMode::BorderlessFullscreen,
                        window_theme: Some(WindowTheme::Dark),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .run();
}

