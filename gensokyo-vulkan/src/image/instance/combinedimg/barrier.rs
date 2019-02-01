
use ash::vk;

use crate::core::GsDevice;

use crate::buffer::allocator::GsBufferAllocator;
use crate::buffer::allocator::types::BufferStorageType;
use crate::buffer::instance::GsImgsrcBuffer;
use crate::buffer::BufferCopiable;
use crate::buffer::GsBufferRepository;

use crate::image::barrier::ImageBarrierCI;
use crate::image::storage::ImageSource;
use crate::image::enums::ImagePipelineStage;
use crate::image::instance::traits::ImageBarrierBundleAbs;
use crate::image::instance::combinedimg::mipmap::MipmapMethod;
use crate::image::traits::ImageCopiable;
use crate::image::view::ImageSubRange;
use crate::image::utils::ImageCopySubrange;
use crate::image::allocator::ImageAllotCI;
use crate::memory::transfer::DataCopyer;

use crate::error::{ VkResult, VkError };
use crate::command::GsCmdTransferApi;
use crate::utils::allot::{ GsAllocatorApi, GsAllotIntoDistributor };
use crate::utils::allot::{ GsDistributeApi, GsDistIntoRepository };
use crate::utils::phantom::Staging;

pub struct SampleImageBarrierBundle {

    info_indices: Vec<usize>,
    dst_stage: ImagePipelineStage,

    staging_repository: Option<GsBufferRepository<Staging>>,
}

impl ImageBarrierBundleAbs for SampleImageBarrierBundle {

    fn make_barrier_transform(&mut self, device: &GsDevice, copyer: &DataCopyer, infos: &mut Vec<ImageAllotCI>) -> VkResult<()> {

        // 1.create staging buffer and memories.
        let (mut staging_repository, buffer_blocks) = self.create_staging_repository(device, infos)?;
        // 2.send textures to the staging buffer.
        self.upload_staging_data(&mut staging_repository, &buffer_blocks, infos)?;

        // 3.upload image data from buffers to images.
        self.upload_image_data(copyer, infos, &buffer_blocks);

        // 4.generate mipmap for each image if needed.
        // visit http://cpp-rendering.io/mipmap-generation/ for detail.
        self.generate_mipmaps(copyer, infos);

        // 5.make image barrier transition for shader reading.
        self.prepare_shader_read(copyer, infos);

        // done. Keep the staging buffer data until the command buffer executes.
        self.staging_repository = Some(staging_repository);

        Ok(())
    }
}

impl SampleImageBarrierBundle {

    pub fn new(dst_stage: ImagePipelineStage, indices: Vec<usize>) -> SampleImageBarrierBundle {
        SampleImageBarrierBundle {

            info_indices: indices, dst_stage,
            staging_repository: None,
        }
    }

    fn create_staging_repository(&mut self, device: &GsDevice, infos: &Vec<ImageAllotCI>) -> VkResult<(GsBufferRepository<Staging>, Vec<GsImgsrcBuffer>)> {

        let staging_buffer_count = self.info_indices.len();
        let mut staging_indices = Vec::with_capacity(staging_buffer_count);

        let mut staging_allocator = GsBufferAllocator::create(device, BufferStorageType::STAGING);

        for &index in self.info_indices.iter() {
            let img_info = GsImgsrcBuffer::new(infos[index].space);
            let buffer_index = staging_allocator.assign(img_info)?;
            staging_indices.push(buffer_index);
        }

        let distributor = staging_allocator.allocate()?;

        let staging_buffers: Vec<GsImgsrcBuffer> = staging_indices.into_iter()
            .map(|index| distributor.acquire(index))
            .collect();

        Ok((distributor.into_repository(), staging_buffers))
    }

    fn upload_staging_data(&self, staging_repository: &mut GsBufferRepository<Staging>, img_data_blocks: &[GsImgsrcBuffer], infos: &Vec<ImageAllotCI>) -> VkResult<()> {

        let mut uploader = staging_repository.data_uploader()?;

        for (&info_index, img_block) in self.info_indices.iter().zip(img_data_blocks.iter()) {

            match infos[info_index].storage.source {
                | ImageSource::UploadData(ref source) => {
                    uploader.upload(img_block, &source.data)?;
                },
                | _ => {
                    return Err(VkError::other("The data of sample image is missing."))
                },
            }
        }

        uploader.finish()?;

        Ok(())
    }

    fn upload_image_data(&self, copyer: &DataCopyer, infos: &mut Vec<ImageAllotCI>, src_blocks: &[GsImgsrcBuffer]) {

        let recorder = copyer.recorder();

        // make image barrier transition for the coming data transfer.
        let transfer_dst_barriers = self.info_indices.iter().map(|&index| {

            let image_info = &mut infos[index];
            // copy to the base mip level of image.
            // TODO: Consider the base layer and layer count.
            let base_mip_level = image_info.view_ci.subrange.clone()
                .with_mip_level(0, 1); // base mip level is at 0.

            transfer_dst_barrier(image_info, base_mip_level)

        }).collect();

        recorder.image_pipeline_barrier(
            vk::PipelineStageFlags::TOP_OF_PIPE, // src stage
            vk::PipelineStageFlags::TRANSFER,    // dst stage
            vk::DependencyFlags::empty(), // dependencies specifying how execution and memory dependencies are formed.
            transfer_dst_barriers
        );

        // copy buffer to base mipmap level image.
        for (i, &index) in self.info_indices.iter().enumerate() {
            // copy to the base mip level.
            let copy_dst_range = ImageCopySubrange::base_copy(&infos[index].view_ci.subrange);
            // copy the whole buffer to the base mip level of image.
            copyer.copy_buffer_to_image(src_blocks[i].copy_whole(), infos[index].copy_range(copy_dst_range));
        }

        // make image barrier transition to transfer src for base mip level(level 0).
        let transfer_src_barriers: Vec<ImageBarrierCI> = self.info_indices.iter().filter_map(|&index| {

            let image_info = &mut infos[index];
            match image_info.image_ci.property.mipmap {
                | MipmapMethod::Disable => None,
                | MipmapMethod::StepBlit
                | MipmapMethod::BaseLevelBlit => {
                    let base_mip_level = image_info.view_ci.subrange.clone()
                        .with_mip_level(0, 1); // base mip level is at 0.

                    let barrier = transfer_src_barrier(image_info, base_mip_level);
                    Some(barrier)
                },
            }
        }).collect();

        if transfer_src_barriers.is_empty() == false {
            recorder.image_pipeline_barrier(
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::DependencyFlags::empty(),
                transfer_src_barriers
            );
        }
    }

    fn generate_mipmaps(&self, copyer: &DataCopyer, infos: &mut Vec<ImageAllotCI>) {

        use crate::image::instance::combinedimg::mipmap;
        use std::collections::HashSet;

        let recorder = copyer.recorder();

        let mut candidate_indices: HashSet<usize> = self.info_indices.iter().cloned().collect();
        let mut current_level = 1; // mipmap generation start from level 1.

        while candidate_indices.is_empty() == false {

            let mut indices_remove = Vec::new();

            let candidate_count = candidate_indices.len();
            let mut blit_prepare_barriers = Vec::with_capacity(candidate_count);
            let mut blit_finish_barriers = Vec::with_capacity(candidate_count);
            let mut blit_infos = Vec::with_capacity(candidate_count);

            candidate_indices.iter().for_each(|&index| {

                let image_info = &mut infos[index];

                match image_info.image_ci.property.mipmap {
                    | MipmapMethod::StepBlit => {

                        // barrier before image blit.
                        let subrange = image_info.view_ci.subrange.clone()
                            .with_mip_level(current_level, 1);

                        let prepare_barrier = mipmap::blit_src_barrier(image_info, subrange.clone());
                        blit_prepare_barriers.push(prepare_barrier);

                        // image blit information.
                        let blit = mipmap::blit_info(image_info, current_level);
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

                if image_info.image_ci.property.mip_levels == current_level + 1 {
                    indices_remove.push(index);
                }
            });


            if blit_prepare_barriers.is_empty() == false {
                // Transition current mip level to transfer destination.
                recorder.image_pipeline_barrier(
                    // TODO: consider change TOP_OF_PIPE to TRANSFER.
                    vk::PipelineStageFlags::TOP_OF_PIPE, // src stage
                    vk::PipelineStageFlags::TRANSFER,    // dst stage
                    vk::DependencyFlags::empty(),
                    blit_prepare_barriers,
                );

                for blit in blit_infos.into_iter() {
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
                    blit_finish_barriers,
                );
            }

            for image_to_remove in indices_remove.iter() {
                candidate_indices.remove(image_to_remove);
            }

            current_level += 1;
        }

    }

    fn prepare_shader_read(&self, copyer: &DataCopyer, infos: &mut Vec<ImageAllotCI>) {

        let final_barriers = self.info_indices.iter().map(|&index| {

            let image_info = &mut infos[index];
            // make all mip levels readable in shader.
            // TODO: Consider the base layer and layer count.
            let all_mip_levels = image_info.view_ci.subrange.clone()
                .with_mip_level(0, image_info.image_ci.property.mip_levels);

            shader_read_barrier(image_info, all_mip_levels)

        }).collect();

        copyer.recorder().image_pipeline_barrier(
            vk::PipelineStageFlags::TRANSFER, // src stage
            self.dst_stage.into(),            // dst stage
            vk::DependencyFlags::empty(),
            final_barriers
        );
    }
}

/// return the image barrier used to transfer image data from buffer to image.
fn transfer_dst_barrier(info: &mut ImageAllotCI, subrange: ImageSubRange) -> ImageBarrierCI {

    let barrier = ImageBarrierCI::new(&info.image, subrange)
        .access_mask(info.current_access, vk::AccessFlags::TRANSFER_WRITE)
        .layout(info.current_layout, vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .build();

    info.current_access = vk::AccessFlags::TRANSFER_WRITE;
    info.current_layout = vk::ImageLayout::TRANSFER_DST_OPTIMAL;

    barrier
}

/// return the image barrier used to transfer image data from buffer to image.
fn transfer_src_barrier(info: &mut ImageAllotCI, subrange: ImageSubRange) -> ImageBarrierCI {

    let barrier = ImageBarrierCI::new(&info.image, subrange)
        .access_mask(info.current_access, vk::AccessFlags::TRANSFER_READ)
        .layout(info.current_layout, vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .build();

    info.current_access = vk::AccessFlags::TRANSFER_READ;
    info.current_layout = vk::ImageLayout::TRANSFER_SRC_OPTIMAL;

    barrier
}

/// return the image barrier used to make image prepare the be sampled from shader.
fn shader_read_barrier(info: &mut ImageAllotCI, subrange: ImageSubRange) -> ImageBarrierCI {

    match info.image_ci.property.mipmap {
        | MipmapMethod::Disable => {
            debug_assert_eq!(info.current_access, vk::AccessFlags::TRANSFER_WRITE);
            debug_assert_eq!(info.current_layout, vk::ImageLayout::TRANSFER_DST_OPTIMAL);
        },
        | MipmapMethod::StepBlit
        | MipmapMethod::BaseLevelBlit => {
            debug_assert_eq!(info.current_access, vk::AccessFlags::TRANSFER_READ);
            debug_assert_eq!(info.current_layout, vk::ImageLayout::TRANSFER_SRC_OPTIMAL);
        },
    }

    let barrier = ImageBarrierCI::new(&info.image, subrange)
        .access_mask(info.current_access, vk::AccessFlags::SHADER_READ)
        .layout(info.current_layout, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        .build();

    info.current_access = vk::AccessFlags::SHADER_READ;
    info.current_layout = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;

    barrier
}
