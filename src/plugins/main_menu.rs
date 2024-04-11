use bevy::prelude::*;
use bevy::render::camera::*;

use crate::plugins::{
    state::GameState,
    util::cleanup,
    loading::{GlobalFont, LoadingAssets}
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_menu_assets)
            .add_systems(OnEnter(GameState::MainMenu), setup_menu);
    }
}

#[derive(Resource, Default)]
struct MainMenuAssets {
    texture: Handle<Image>,
}

#[derive(Component, Default)]
struct MainMenu;

fn load_menu_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    let texture = asset_server.load::<Image>("ui/button.png");
    loading_assets.add(texture.clone());

    commands.insert_resource(MainMenuAssets { texture });
}

fn setup_menu(
    commands: &mut Commands,
    global_font: Res<GlobalFont>,
    main_menu_assets: Res<MainMenuAssets>,
) {
    commands.spawn((
        Camera2dBundle::default(),
        MainMenu,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: main_menu_assets.texture.clone().into(),
                style: Style {
                    height: Val::Vh(100.0),
                    ..Default::default()
                },
                ..Default::default()
            });
            parent.spawn(TextBundle {
                style: Style {
                    bottom: Val::Percent(10.0),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                text: Text::from_section(
                    "Press SPACE to start",
                    TextStyle {
                        font: global_font.0.clone(),
                        font_size: 36.0,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });
        });
}

fn cleanup_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
) {
    for i in query.iter() {
        commands.entity(i).despawn_recursive();
    }
}
