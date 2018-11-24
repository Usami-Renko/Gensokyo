
use ash::vk;

pub fn image_subrange_to_layers(subrange: &vk::ImageSubresourceRange) -> vk::ImageSubresourceLayers {
    vk::ImageSubresourceLayers {
        aspect_mask      : subrange.aspect_mask,
        mip_level        : subrange.base_mip_level,
        base_array_layer : subrange.base_array_layer,
        layer_count      : subrange.layer_count,
    }
}
