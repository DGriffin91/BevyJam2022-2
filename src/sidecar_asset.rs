//! Implements loader for a custom asset type.

use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "8c4ceb9d-1d40-4942-8836-834151d4aff6"]
pub struct SidecarAsset {
    pub value: i32,
}

#[derive(Default)]
pub struct SidecarAssetLoader;

impl AssetLoader for SidecarAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = serde_json::from_slice::<SidecarAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["car"]
    }
}

pub struct SidecarAssetPlugin;
impl Plugin for SidecarAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<SidecarAsset>()
            .init_asset_loader::<SidecarAssetLoader>();
    }
}

/*
// Didn't get test to work, it doesn't see Assets<SidecarAsset>
#[cfg(test)]
mod tests {
    use bevy::{asset::AssetPlugin, ecs::system::SystemState};

    use super::*;

    #[derive(Default)]
    struct State {
        handle: Handle<SidecarAsset>,
    }

    #[test]
    fn test_sidecar_file() {
        let mut app = App::new();
        app.add_plugin(AssetPlugin::default())
            .add_plugins(MinimalPlugins)
            .init_resource::<State>()
            .add_plugin(SidecarAssetPlugin);

        let mut system_state: SystemState<(Res<AssetServer>, ResMut<State>)> =
            SystemState::new(&mut app.world);

        let (ass, mut state) = system_state.get_mut(&mut app.world);

        state.handle = ass.load("data/asset.car");

        app.update();

        let sidecar_assets = app
            .world
            .get_resource::<ResMut<Assets<SidecarAsset>>>()
            .unwrap();

        let state = app.world.get_resource::<Res<State>>().unwrap();
        let sidecar_asset = sidecar_assets.get(&state.handle).unwrap();

        info!("Sidecar asset loaded: {:?}", sidecar_asset);
    }
}
*/
