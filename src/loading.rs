use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<FontAssets>()
            .with_collection::<TextureAssets>()
            .continue_to_state(GameState::Menu)
            .build(app);
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/ferrum.otf")]
    pub roboto: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/title.png")]
    pub title_texture: Handle<Image>,
    #[asset(path = "textures/button.png")]
    pub button_texture: Handle<Image>,
    #[asset(path = "textures/stage.png")]
    pub stage_texture: Handle<Image>,
    #[asset(path = "textures/player-64x64.png")]
    pub player_texture_64: Handle<Image>,
    #[asset(path = "textures/reflector.png")]
    pub reflector_texture: Handle<Image>,
    #[asset(path = "textures/enemy.png")]
    pub alarm_texture: Handle<Image>,
    #[asset(path = "textures/grandfather_moon.png")]
    pub grandfather_moon: Handle<Image>,
    #[asset(path = "textures/grandfather_sun.png")]
    pub grandfather_sun: Handle<Image>,
}
