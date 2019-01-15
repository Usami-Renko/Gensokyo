
use gsvk::core::physical::GsPhysicalDevice;
use gsvk::types::vkuint;

use std::collections::HashMap;
use std::marker::PhantomData;

// ------------------------------------------------------------------------------------
type ReferenceIndex = usize;
type   StorageIndex = usize;

pub(super) struct GsglTFAssetLib<Asset, AssetData>
    where Asset: GsglTFAsset<AssetData> {

    phantom_type: PhantomData<AssetData>,
    indices: HashMap<ReferenceIndex, StorageIndex>,
    assets: Vec<Asset>,
}

impl<Asset, AssetData> Default for GsglTFAssetLib<Asset, AssetData>
    where Asset: GsglTFAsset<AssetData> {

    fn default() -> GsglTFAssetLib<Asset, AssetData> {
        GsglTFAssetLib {
            phantom_type: PhantomData,
            indices: HashMap::new(),
            assets: Vec::new(),
        }
    }
}

impl<Asset, AssetData> GsglTFAssetLib<Asset, AssetData>
    where Asset: GsglTFAsset<AssetData> {

    pub fn load<Document>(&mut self, doc: Document, ref_index: ReferenceIndex) -> StorageIndex
        where Asset: From<Document> {

        if let Some(store_index) = self.indices.get(&ref_index) {

            store_index.clone()
        } else {
            let asset = Asset::from(doc);

            let store_index = self.assets.len();
            self.indices.insert(ref_index, store_index);
            self.assets.push(asset);

            store_index
        }
    }

    pub fn into_data(self) -> Vec<AssetData> {

        self.assets.into_iter()
            .map(|asset| asset.into_data())
            .collect()
    }
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub(super) trait GsglTFAsset<AssetData>: Sized + Default {
    const ASSET_NAME: &'static str;

    fn into_data(self) -> AssetData;
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub(crate) struct GsglTFPhyLimits {

    pub max_push_constant_size: vkuint,
}

impl<'a> From<&'a GsPhysicalDevice> for GsglTFPhyLimits {

    fn from(phy: &'a GsPhysicalDevice) -> GsglTFPhyLimits {
        GsglTFPhyLimits {
            max_push_constant_size: phy.limits().max_push_constants_size,
        }
    }
}
// ------------------------------------------------------------------------------------
