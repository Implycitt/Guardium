use bevy::prelude::*;
use bevy_nine_slice_ui::NineSliceUiTexture;

use crate::plugins::loading::LoadingAssets;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiAssets>()
            .add_systems(Update, button_style);
    }
}

#[derive(Resource)]
pub struct UiAssets {
    pub button: Handle<Image>,
}

impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let button = asset_server.load("ui/button.png");

        let mut loading_assets = world.resource_mut::<LoadingAssets>();

        loading_assets.0.push(button.clone().into());

        UiAssets {
            button,
        }

    }
}

pub fn button_style(
    mut interaction_query: Query<
        (&Interaction, &mut NineSliceUiTexture), (Changed<Interaction>, With<Button>)>,
    assets: Res<UiAssets>,
) {
    // actually I have to do this and find pictures for each button and the interactions
    for (interaction, mut texture) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *texture = NineSliceUiTexture::from_image(assets.button.clone());
            }
            Interaction::Hovered => {
                *texture = NineSliceUiTexture::from_image(assets.button.clone());
            }
            Interaction::None => {
                *texture = NineSliceUiTexture::from_image(assets.button.clone());
            }
        }
    }
}
