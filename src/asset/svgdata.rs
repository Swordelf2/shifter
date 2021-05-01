use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::reflect::TypeUuid;
use bevy::utils::BoxedFuture;

/// Asset type, which holds additional svg data for an object
#[derive(TypeUuid)]
#[uuid = "737b3336-aa6f-11eb-bcbc-0242ac130002"]
pub struct SvgData {}

#[derive(Default)]
pub struct SvgDataLoader;

impl AssetLoader for SvgDataLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            println!("LOADING {:?}", bytes);
            // TODO: impl
            unimplemented!()
            /*
            load_context.set_default_asset(LoadedAsset::new(SvgData { ... }));
            Ok(())
                */
        })
    }

    fn extensions(&self) -> &[&str] {
        &["svg"]
    }
}
