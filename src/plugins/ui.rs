use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resrouce::<UIAssets>()
            .add_systems(Update, button_style);
    }
}

// idk about this yet
pub struct UIAssets;
