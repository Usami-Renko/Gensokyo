
use ash::vk;
use ash::vk::uint32_t;

use resources::image::{ ImageTiling, ImageLayout };
use pipeline::state::SampleCountType;

pub trait HaImageDescAbs {

    // image property.
    fn set_tiling(&mut self, tiling: ImageTiling);
    fn set_initial_layout(&mut self, layout: ImageLayout);
    fn set_samples(&mut self, count: SampleCountType, mip_levels: uint32_t, array_layers: uint32_t);
    fn set_share_queues(&mut self, queue_family_indices: Vec<uint32_t>);
}

pub trait HaImageViewDescAbs {

    // image view property.
    fn set_mapping_component(&mut self, r: vk::ComponentSwizzle, g: vk::ComponentSwizzle, b: vk::ComponentSwizzle, a: vk::ComponentSwizzle);

    /// Select the set of mipmap levels and array layers to be accessible to the view.
    ///
    /// base_mip_level is the first mipmap level accessible to the view.
    ///
    /// level_count is the number of mipmap levels (starting from base_mip_level) accessible to the view.
    ///
    /// base_array_layer is the first array layer accessible to the view.
    ///
    /// layer_count is the number of array layers (starting from baseArrayLayer) accessible to the view.
    fn set_subrange(&mut self, base_mip_level: uint32_t, level_count: uint32_t, base_array_layer: uint32_t, layer_count: uint32_t);
}
