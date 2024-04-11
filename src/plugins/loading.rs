use bevy::{
    prelude::*,
    asset::UntypedAssetId,
};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingAssets>();
    }
}

#[derive(Default, Resource)]
pub struct LoadingAssets {
    loading: Vec<UntypedAssetId>,
}

impl LoadingAssets {
    pub fn add(&mut self, asset: impl Into<UntypedAssetId>) {
        self.loading.push(asset.into())
    }
}

#[derive(Default, Resource)]
pub struct GlobalFont(pub Handle<Font>); 


