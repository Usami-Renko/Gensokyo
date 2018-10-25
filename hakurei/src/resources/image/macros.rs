
macro_rules! impl_image_desc_info_abs {
    ($ImageBranchInfo:ty) => {

        impl HaImageDescAbs for $ImageBranchInfo {

            fn set_tiling(&mut self, tiling: ImageTiling) {
                self.image_desc.tiling = tiling.value();
            }
            fn set_initial_layout(&mut self, layout: ImageLayout) {
                self.image_desc.initial_layout = layout.value();
            }
            fn set_samples(&mut self, count: SampleCountType, mip_levels: uint32_t, array_layers: uint32_t) {
                self.image_desc.sample_count = count.value();
                self.image_desc.mip_levels   = mip_levels;
                self.image_desc.array_layers = array_layers;
            }
            fn set_share_queues(&mut self, queue_family_indices: Vec<uint32_t>) {
                self.image_desc.sharing = vk::SharingMode::Concurrent;
                self.image_desc.queue_family_indices = queue_family_indices;
            }
        }

        impl HaImageViewDescAbs for $ImageBranchInfo {

            // image view property.
            fn set_mapping_component(&mut self, r: vk::ComponentSwizzle, g: vk::ComponentSwizzle, b: vk::ComponentSwizzle, a: vk::ComponentSwizzle) {
                self.view_desc.components = vk::ComponentMapping { r, g, b, a };
            }
            fn set_subrange(&mut self, base_mip_level: uint32_t, level_count: uint32_t, base_array_layer: uint32_t, layer_count: uint32_t) {

                self.view_desc.subrange.base_mip_level   = base_mip_level;
                self.view_desc.subrange.level_count      = level_count;
                self.view_desc.subrange.base_array_layer = base_array_layer;
                self.view_desc.subrange.layer_count      = layer_count;
            }
        }
    };
}

macro_rules! impl_image_branch_abs {
    ($ImageBranch:ty) => {
        impl HaImageBranchAbs for $ImageBranch {

            fn view_index(&self) -> usize {
                self.item.view_index
            }
            fn fill_handles(&mut self, image: vk::Image, view: vk::ImageView) {
                self.item.set_handles(image, view);
            }
        }
    };
}
