use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{tdtiled, TDState};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(TDState::Load).continue_to_state(TDState::Playing),
        );
        app.add_collection_to_loading_state::<_, MapHandles>(TDState::Load);
    }
}

#[derive(AssetCollection, Resource)]
pub struct MapHandles {
    #[asset(path = "map_h.tmx")]
    pub map_h: Handle<tdtiled::TiledMap>,
}

