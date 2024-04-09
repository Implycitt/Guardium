use bevy::{
    prelude::*,
    asset::UntypedAssetId,
};

use bevy_nine_slice_ui::NineSliceUiTexture;
use bevy_pipelines_ready::PipelinesReadyPlugin;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PipelinesReadyPlugin)
            .init_resource::<LoadingAssets>();
    }
}

#[derive(Default, Resource)]
pub struct LoadingAssets(pub Vec<UntypedAssetId>);
