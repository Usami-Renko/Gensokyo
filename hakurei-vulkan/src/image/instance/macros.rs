
macro_rules! impl_image_desc_info_abs {
    ($ImageInstanceInfo:ty) => {

        // image property.
        impl HaImageDescAbs for $ImageInstanceInfo {

            fn with_tiling(&mut self, tiling: vk::ImageTiling) {

                self.image_desc.property.tiling = tiling;
            }

            fn with_initial_layout(&mut self, layout: vk::ImageLayout) {

                self.image_desc.property.initial_layout = layout;
            }

            fn with_samples(&mut self, count: vk::SampleCountFlags, mip_levels: vkuint, array_layers: vkuint) {

                self.image_desc.property.sample_count = count;
                self.image_desc.property.mip_levels   = mip_levels;
                self.image_desc.property.array_layers = array_layers;
            }

            fn with_share_queues(&mut self, queue_family_indices: Vec<vkuint>) {

                self.image_desc.specific.share_queue_families(Some(queue_family_indices));
            }
        }

        impl HaImageViewDescAbs for $ImageInstanceInfo {

            // image view property.
            fn with_mapping_component(&mut self, r: vk::ComponentSwizzle, g: vk::ComponentSwizzle, b: vk::ComponentSwizzle, a: vk::ComponentSwizzle) {
                self.view_desc.components = vk::ComponentMapping {
                    r, g, b, a,
                };
            }

            fn with_subrange(&mut self, base_mip_level: vkuint, level_count: vkuint, base_array_layer: vkuint, layer_count: vkuint) {

                self.view_desc.subrange = vk::ImageSubresourceRange {
                    aspect_mask: self.view_desc.subrange.aspect_mask,
                    base_mip_level, level_count, base_array_layer, layer_count,
                };
            }
        }


    };
}
