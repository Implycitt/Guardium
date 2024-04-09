use bevy::prelude::*;

use crate::plugins::{
    state::GameState,
    util::cleanup,
    ui::UiAssets,
};

use bevy_nine_slice_ui::NineSliceUiTexture;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_menu)
            .add_systems(Update, play_button.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup::<MainMenuScene>);
    }
}

#[derive(Component)]
struct MainMenuScene;

#[derive(Component)]
struct PlayButton;

fn setup_menu(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(45.0),
        margin: UiRect::all(Val::Px(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 18.0,
        color: Color::rgb(0.9, 0.9, 0.9),
        ..default()
    };

    let play_button = commands.spawn((
        ButtonBundle {
            style: button_style.clone(),
            ..default()
        },
        NineSliceUiTexture::from_image(ui_assets.button.clone()),
        PlayButton,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section("Play", button_text_style.clone()));
    })
    .id();
}

fn play_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

