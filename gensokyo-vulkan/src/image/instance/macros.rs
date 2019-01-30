
macro_rules! impl_image_desc_info_abs {
    ($ImageInstanceInfo:ty) => {

        // image property.
        impl ImageTgtCIAbs for $ImageInstanceInfo {

            fn with_tiling(mut self, tiling: vk::ImageTiling) -> $ImageInstanceInfo {

                self.image_ci.property.tiling = tiling;
                self
            }

            fn with_initial_layout(mut self, layout: vk::ImageLayout) -> $ImageInstanceInfo {

                self.image_ci.property.initial_layout = layout;
                self
            }

            fn with_samples(mut self, count: vk::SampleCountFlags, mip_levels: vkuint, array_layers: vkuint) -> $ImageInstanceInfo {

                self.image_ci.property.sample_count = count;
                self.image_ci.property.mip_levels   = mip_levels;
                self.image_ci.property.array_layers = array_layers;
                self
            }

            fn with_share_queues(mut self, queue_family_indices: Vec<vkuint>) -> $ImageInstanceInfo {

                self.image_ci.specific.share_queue_families(Some(queue_family_indices));
                self
            }
        }

        impl ImageViewCIAbs for $ImageInstanceInfo {

            // image view property.
            fn with_mapping_component(mut self, r: vk::ComponentSwizzle, g: vk::ComponentSwizzle, b: vk::ComponentSwizzle, a: vk::ComponentSwizzle) -> $ImageInstanceInfo {
                self.view_ci.components = vk::ComponentMapping {
                    r, g, b, a,
                };

                self
            }

            fn with_subrange(mut self, base_mip_level: vkuint, level_count: vkuint, base_array_layer: vkuint, layer_count: vkuint) -> $ImageInstanceInfo {

                self.view_ci.subrange = vk::ImageSubresourceRange {
                    aspect_mask: self.view_ci.subrange.aspect_mask,
                    base_mip_level, level_count, base_array_layer, layer_count,
                };

                self
            }
        }


    };
}
