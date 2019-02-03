
use ash::vk;

use crate::image::mipmap::{ MipmapMethod, MipmapBlitInfo };
use crate::image::allocator::ImageAllotCI;
use crate::image::enums::ImageInstanceType;
use crate::image::barrier::ImageBarrierCI;

use crate::command::{ GsCmdRecorder, GsCmdTransferApi };
use crate::utils::phantom::Transfer;
use crate::types::vkuint;

use std::collections::HashSet;

// visit http://cpp-rendering.io/mipmap-generation/ for detail.
pub(super) fn generate_mipmaps(recorder: &GsCmdRecorder<Transfer>, image_type: &ImageInstanceType, info_indices: &Vec<usize>, infos: &mut Vec<ImageAllotCI>) {

    use crate::image::mipmap;

    let mut candidate_indices: HashSet<usize> = info_indices.iter().cloned().collect();
    let mut current_level = 1; // mipmap generation start from level 1.
    let blit_layer_count = blit_layer_count(image_type);

    while candidate_indices.is_empty() == false {

        let mut indices_remove = Vec::new();

        let candidate_count = candidate_indices.len();
        let mut blit_prepare_barriers = Vec::with_capacity(candidate_count);
        let mut blit_finish_barriers = Vec::with_capacity(candidate_count);
        let mut blit_infos = Vec::with_capacity(candidate_count);

        candidate_indices.iter().for_each(|&index| {

            let image_info = &mut infos[index];

            match image_info.backend.image_ci.property.mipmap {
                | MipmapMethod::StepBlit => {

                    // barrier before image blit.
                    let subrange = image_info.backend.view_ci.subrange.clone()
                        .with_mip_level(current_level, 1);

                    let prepare_barrier = mipmap::blit_src_barrier(image_info, subrange.clone());
                    blit_prepare_barriers.push(prepare_barrier);

                    // image blit information.
                    let blit = mipmap::blit_info(image_info, current_level, blit_layer_count);
                    blit_infos.push(blit);

                    // barrier after image blit.
                    let finish_barrier = mipmap::blit_dst_barrier(image_info, subrange);
                    blit_finish_barriers.push(finish_barrier);
                },
                | MipmapMethod::BaseLevelBlit => {
                    unimplemented!()
                },
                | MipmapMethod::Disable => {
                    indices_remove.push(index);
                },
            };

            if image_info.backend.image_ci.property.mip_levels == current_level + 1 {
                indices_remove.push(index);
            }
        });


        blit_image(recorder, blit_infos, blit_prepare_barriers, blit_finish_barriers);

        for image_to_remove in indices_remove.iter() {
            candidate_indices.remove(image_to_remove);
        }

        current_level += 1;
    }
}

pub(super) fn blit_image(recorder: &GsCmdRecorder<Transfer>, blit_commands: Vec<MipmapBlitInfo>, prepare_transition: Vec<ImageBarrierCI>, finish_transition: Vec<ImageBarrierCI>) {

    if prepare_transition.is_empty() == false {

        // Transition current mip level to transfer destination.
        recorder.image_pipeline_barrier(
            // TODO: consider change TOP_OF_PIPE to TRANSFER.
            vk::PipelineStageFlags::TOP_OF_PIPE, // src stage
            vk::PipelineStageFlags::TRANSFER,    // dst stage
            vk::DependencyFlags::empty(),
            prepare_transition,
        );

        for blit in blit_commands.into_iter() {
            recorder.blit_image(
                blit.image, blit.src_layout,
                blit.image, blit.dst_layout,
                &[blit.blit],
                blit.filter
            );
        }

        recorder.image_pipeline_barrier(
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            finish_transition,
        );
    }
}

pub(super) fn blit_layer_count(image_type: &ImageInstanceType) -> vkuint {

    match image_type {
        | ImageInstanceType::CombinedImageSampler { .. }
        | ImageInstanceType::SampledImage { .. } => {
            1
        },
        | ImageInstanceType::CubeMapImage { .. } => {
            6
        },
        | ImageInstanceType::DepthStencilImage { .. }
        | ImageInstanceType::DepthStencilAttachment => {
            unreachable!("Depth Stencil Images don't use mipmap generation.")
        },
    }
}
