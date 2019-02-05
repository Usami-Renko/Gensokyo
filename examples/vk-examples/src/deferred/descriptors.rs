
use gsvk::prelude::descriptor::*;
use gsvk::prelude::pipeline::*;
use gsvk::prelude::api::*;
use gs::prelude::*;

use super::models::ModelResource;

pub struct DescriptorAssetAllocator {

    allocator: GsDescriptorAllocator,
}

pub struct DescriptorAssetDistributor {

    distributor: GsDescriptorDistributor,
}

impl DescriptorAssetAllocator {

    pub fn generate(initializer: &AssetInitializer) -> DescriptorAssetAllocator {

        let allocator = GsDescriptorAllocator::new(initializer);

        DescriptorAssetAllocator { allocator }
    }

    pub fn allocator(self) -> GsResult<DescriptorAssetDistributor> {

        let distributor = self.allocator.allocate()?;

        Ok(DescriptorAssetDistributor { distributor })
    }
}
