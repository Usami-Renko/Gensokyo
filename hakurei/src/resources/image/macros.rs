
macro_rules! impl_image_desc_info_abs {
    ($ImageBranchInfo:ty) => {

        // image property.
        impl HaImageDescAbs for $ImageBranchInfo {

            fn set_tiling(&mut self, tiling: ImageTiling) {
                self.image_desc.tiling = tiling;
            }

            fn set_initial_layout(&mut self, layout: ImageLayout) {
                self.image_desc.initial_layout = layout;
            }

            fn set_samples(&mut self, count: SampleCountType, mip_levels: vkint, array_layers: vkint) {
                self.image_desc.sample_count = count;
                self.image_desc.mip_levels   = mip_levels;
                self.image_desc.array_layers = array_layers;
            }

            fn set_share_queues(&mut self, queue_family_indices: Vec<vkint>) {
                self.image_desc.sharing = SharingMode::Concurrent;
                self.image_desc.queue_family_indices = queue_family_indices;
            }
        }

        impl HaImageViewDescAbs for $ImageBranchInfo {

            // image view property.
            fn set_mapping_component(&mut self, r: ComponentSwizzle, g: ComponentSwizzle, b: ComponentSwizzle, a: ComponentSwizzle) {
                self.view_desc.components = (r, g, b, a);
            }

            fn set_subrange(&mut self, base_mip_level: vkint, level_count: vkint, base_array_layer: vkint, layer_count: vkint) {
                self.view_desc.subrange.set(base_mip_level, level_count, base_array_layer, layer_count);
            }
        }
    };
}
