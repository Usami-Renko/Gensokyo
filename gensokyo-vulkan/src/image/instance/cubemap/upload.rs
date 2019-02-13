
use ash::vk;

use crate::image::mipmap::MipmapMethod;
use crate::image::allocator::ImageAllotCI;
use crate::image::copy::ImageCopiable;
use crate::image::barrier::ImageBarrierCI;

use crate::buffer::BufferCopiable;
use crate::buffer::instance::GsImgsrcBuffer;

use crate::memory::transfer::DataCopyer;
use crate::command::GsCmdTransferApi;

pub(in crate::image::instance)
fn upload_cube_image_data(copyer: &DataCopyer, info_indices: &[usize], infos: &mut [ImageAllotCI], src_blocks: &[GsImgsrcBuffer]) {

    transfer_prepare_transition(copyer, info_indices, infos);

    for (i, &index) in info_indices.iter().enumerate() {

        const CUBEMAP_BUFFER_RANGE_COUNT: usize = 6;

        let src_ranges = src_blocks[i].copy_split_ranges(CUBEMAP_BUFFER_RANGE_COUNT);
        // just copy to the base mipmap level.
        let dst_ranges = infos[index].full_copy_mipmap_layer_ranges(0);

        copyer.copy_buffer_to_image_ranges(src_ranges, dst_ranges);
    }

    transfer_finish_transition(copyer, info_indices, infos);
}


// make image barrier transition for the coming data transfer.
fn transfer_prepare_transition(copyer: &DataCopyer, info_indices: &[usize], infos: &mut [ImageAllotCI]) {

    // debug_assert_eq!(image_info.backend.image_ci.property.array_layers, 6);

    let transfer_dst_barriers: Vec<ImageBarrierCI> = info_indices.iter().map(|&index| {

        let image_info = &mut infos[index];
        // copy to the base mip level of image.
        let base_mip_level = image_info.backend.view_ci.subrange.clone()
            .with_layer(0, image_info.backend.image_ci.property.array_layers)
            .with_mip_level(0, 1); // base mip level is at 0.

        use crate::image::barrier::transfer_dst_barrier;
        transfer_dst_barrier(image_info, base_mip_level)

    }).collect();

    if transfer_dst_barriers.is_empty() == false {
        copyer.recorder().image_pipeline_barrier(
            vk::PipelineStageFlags::TOP_OF_PIPE, // src stage
            vk::PipelineStageFlags::TRANSFER,    // dst stage
            vk::DependencyFlags::empty(), // dependencies specifying how execution and memory dependencies are formed.
            transfer_dst_barriers
        );
    }
}

// make image barrier transition to transfer src for base mip level(level 0).
fn transfer_finish_transition(copyer: &DataCopyer, info_indices: &[usize], infos: &mut [ImageAllotCI]) {

    let transfer_src_barriers: Vec<ImageBarrierCI> = info_indices.iter().filter_map(|&index| {

        let image_info = &mut infos[index];
        match image_info.backend.image_ci.property.mipmap {
            | MipmapMethod::Disable => None,
            | MipmapMethod::StepBlit
            | MipmapMethod::BaseLevelBlit => {
                let base_mip_level = image_info.backend.view_ci.subrange.clone()
                    .with_layer(0, image_info.backend.image_ci.property.array_layers)
                    .with_mip_level(0, 1); // base mip level is at 0.

                use crate::image::barrier::transfer_src_barrier;
                let barrier = transfer_src_barrier(image_info, base_mip_level);
                Some(barrier)
            },
        }
    }).collect();

    if transfer_src_barriers.is_empty() == false {
        copyer.recorder().image_pipeline_barrier(
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::DependencyFlags::empty(),
            transfer_src_barriers
        );
    }
}
