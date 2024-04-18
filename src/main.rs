use bevy::{
    core::FrameCount,
    prelude::*,
    window::*,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod plugins;

use plugins::{
    camera::CameraPlugin,
    enemies::EnemyPlugin,
    towers::TowerPlugin,
    state::{GameState, StatePlugin},
    resources::ResourcesPlugin,
    collisions::CollisionPlugin,
    game_over::GameOverPlugin,
};

fn main() {
    App::new()
        .init_state::<GameState>()
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
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(
            Update,
            (
                make_visible,
            ),
        )
        .add_plugins((CameraPlugin, EnemyPlugin, TowerPlugin, ResourcesPlugin, CollisionPlugin, StatePlugin, GameOverPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

fn make_visible(
    mut window: Query<&mut Window>, 
    frames: Res<FrameCount>
) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
    }
}
