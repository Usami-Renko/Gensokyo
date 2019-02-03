
use ash::vk;

use crate::image::entity::ImageEntity;
use crate::image::copy::{ ImageFullCopyInfo, ImageRangesCopyInfo, ImageCopyRange };
use crate::image::instance::traits::ImageInstanceInfoDesc;

use crate::types::{ vkuint, vkDim3D };

pub trait ImageHandleEntity: Sized {

    fn handle(&self) -> vk::Image;
}

pub trait ImageInstance<I>: ImageCopiable {

    fn build(img: I, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> Self where Self: Sized;
}

pub trait ImageCopiable: Sized {

    /// Copy all layers of a specific mipmap of an image.
    fn full_copy_mipmap(&self, copy_mip_level: vkuint) -> ImageFullCopyInfo;

    fn full_copy_mipmap_layer_ranges(&self, copy_mip_level: vkuint) -> ImageRangesCopyInfo {

        use std::cmp::max;

        let full_range = self.full_copy_mipmap(copy_mip_level);

        let layer_count = full_range.sub_resource_layers.layer_count;

        let mut ranges = Vec::with_capacity(layer_count as usize);
        for layer in 0..layer_count {

            let copy_range = ImageCopyRange {
                // all layer should share the same dimension.
                extent: vkDim3D {
                    width : max(full_range.extent.width  >> copy_mip_level, 1),
                    height: max(full_range.extent.height >> copy_mip_level, 1),
                    depth : 1, // copy one layer each time.
                },
                sub_resource_layers: vk::ImageSubresourceLayers {
                    aspect_mask      : full_range.sub_resource_layers.aspect_mask,
                    mip_level        : copy_mip_level,
                    base_array_layer : layer,
                    layer_count      : 1,
                },
            };
            ranges.push(copy_range);
        }

        ImageRangesCopyInfo {
            handle: full_range.handle,
            layout: full_range.layout,
            ranges,
        }
    }
}
