
use ash::vk;

use crate::core::GsDevice;

use crate::image::ImageTgtCI;
use crate::image::allocator::ImageAllotCI;
use crate::image::barrier::ImageBarrierCI;
use crate::image::view::ImageSubRange;

use crate::types::{ vkuint, vksint };
use crate::error::VkResult;

use std::cmp::max;

#[derive(Debug, Clone, Copy)]
pub enum MipmapMethod {
    /// `Disable` specifies program not to generate mip-map for the image.
    Disable,
    /// `StepBlit` specifies program to blit down the whole mip-chain from level n-1 to n.
    StepBlit,
    /// `BaseLevelBlit` specifies program to use the base image and blit down from that to all levels.
    BaseLevelBlit,
}

impl MipmapMethod {

    pub(super) fn is_support_by_device(&self, device: &GsDevice, image_ci: &ImageTgtCI) -> VkResult<bool> {

        match self {
            | MipmapMethod::Disable => Ok(true),
            | MipmapMethod::StepBlit
            | MipmapMethod::BaseLevelBlit => {

                // mip-chain generation requires support for blit source and destination.
                match image_ci.property.tiling {
                    | vk::ImageTiling::LINEAR => {
                        device.phys.formats.query_format_linear(
                            image_ci.specific.format,
                            vk::FormatFeatureFlags::BLIT_SRC | vk::FormatFeatureFlags::BLIT_DST
                        )
                    },
                    | vk::ImageTiling::OPTIMAL => {
                        device.phys.formats.query_format_optimal(
                            image_ci.specific.format,
                            vk::FormatFeatureFlags::BLIT_SRC | vk::FormatFeatureFlags::BLIT_DST
                        )
                    },
                    | _ => {
                        unreachable!("vk::ImageTiling should be LINEAR or OPTIMAL.")
                    },
                }
            },
        }
    }
}

pub(super) struct MipmapBlitInfo {

    pub image : vk::Image,
    pub src_layout: vk::ImageLayout,
    pub dst_layout: vk::ImageLayout,

    pub filter: vk::Filter,

    pub blit: vk::ImageBlit,
}

pub(super) fn blit_info(image_info: &mut ImageAllotCI, round: vkuint) -> MipmapBlitInfo {

    let image_dimension = &image_info.image_ci.specific.dimension;

    // image blit command.
    let image_blit = vk::ImageBlit {
        src_subresource: vk::ImageSubresourceLayers {
            aspect_mask: image_info.view_ci.subrange.0.aspect_mask,
            mip_level  : round - 1,
            base_array_layer: 0, // TODO: the base layer and layer count are not taken into account yet.
            layer_count     : 1,
        },
        src_offsets: [
            vk::Offset3D { x: 0, y: 0, z: 0 },
            vk::Offset3D {
                x: max((image_dimension.width  >> (round - 1)) as vksint, 1),
                y: max((image_dimension.height >> (round - 1)) as vksint, 1),
                z: 1,
            },
        ],
        dst_subresource: vk::ImageSubresourceLayers {
            aspect_mask: image_info.view_ci.subrange.0.aspect_mask,
            mip_level  : round,
            base_array_layer: 0,
            layer_count     : 1,
        },
        dst_offsets: [
            vk::Offset3D { x: 0, y: 0, z: 0 },
            vk::Offset3D {
                x: max((image_dimension.width  >> round) as vksint, 1),
                y: max((image_dimension.height >> round) as vksint, 1),
                z: 1,
            },
        ],
    };

    debug_assert_eq!(image_info.current_access, vk::AccessFlags::TRANSFER_WRITE);
    debug_assert_eq!(image_info.current_layout, vk::ImageLayout::TRANSFER_DST_OPTIMAL);

    MipmapBlitInfo {
        image: image_info.image.handle,
        src_layout: vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
        dst_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        filter: vk::Filter::LINEAR, // TODO: Make filter configurable.
        blit  : image_blit,
    }
}

pub(super) fn blit_src_barrier(info: &mut ImageAllotCI, subrange: ImageSubRange) -> ImageBarrierCI {

    debug_assert_eq!(info.current_access, vk::AccessFlags::TRANSFER_READ);
    debug_assert_eq!(info.current_layout, vk::ImageLayout::TRANSFER_SRC_OPTIMAL);

    let barrier = ImageBarrierCI::new(&info.image, subrange)
        .access_mask(vk::AccessFlags::empty(), vk::AccessFlags::TRANSFER_WRITE)
        .layout(vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .build();

    info.current_access = vk::AccessFlags::TRANSFER_WRITE;
    info.current_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;

    barrier
}

/// return the image barrier used to transfer image data from buffer to image.
pub(super) fn blit_dst_barrier(info: &mut ImageAllotCI, subrange: ImageSubRange) -> ImageBarrierCI {

    debug_assert_eq!(info.current_access, vk::AccessFlags::TRANSFER_WRITE);
    debug_assert_eq!(info.current_layout, vk::ImageLayout::TRANSFER_DST_OPTIMAL);

    let barrier = ImageBarrierCI::new(&info.image, subrange)
        .access_mask(vk::AccessFlags::TRANSFER_WRITE, vk::AccessFlags::TRANSFER_READ)
        .layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .build();

    info.current_access = vk::AccessFlags::TRANSFER_READ;
    info.current_layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;

    barrier
}

