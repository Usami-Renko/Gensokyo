
use ash::vk;

use crate::core::GsDevice;

use crate::buffer::allocator::GsBufferAllocator;
use crate::buffer::allocator::types::BufferStorageType;
use crate::buffer::instance::GsImgsrcBuffer;
use crate::buffer::GsBufferRepository;

use crate::image::barrier::ImageBarrierCI;
use crate::image::storage::ImageSource;
use crate::image::enums::{ ImagePipelineStage, ImageInstanceType };
use crate::image::instance::traits::ImageBarrierBundleAbs;
use crate::image::mipmap::MipmapMethod;
use crate::image::view::ImageSubRange;
use crate::image::allocator::ImageAllotCI;
use crate::memory::transfer::DataCopyer;

use crate::error::{ VkResult, VkError };
use crate::command::{ GsCmdRecorder, GsCmdTransferApi };
use crate::utils::allot::{ GsAllocatorApi, GsAllotIntoDistributor };
use crate::utils::allot::{ GsDistributeApi, GsDistIntoRepository };
use crate::utils::phantom::{ Staging, Transfer };

pub struct SampleImageBarrierBundle {

    image_type: ImageInstanceType,
    info_indices: Vec<usize>,
    dst_stage: ImagePipelineStage,

    staging_repository: Option<GsBufferRepository<Staging>>,
}

impl ImageBarrierBundleAbs for SampleImageBarrierBundle {

    fn make_barrier_transform(&mut self, device: &GsDevice, copyer: &DataCopyer, infos: &mut Vec<ImageAllotCI>) -> VkResult<()> {

        let recorder = copyer.recorder();

        // 1.create staging buffer and memories.
        let (mut staging_repository, buffer_blocks) = self.create_staging_repository(device, infos)?;
        // 2.send textures to the staging buffer.
        self.upload_staging_data(&mut staging_repository, &buffer_blocks, infos)?;

        // 3.upload image data from buffers to images.
        self.upload_image_data(copyer, infos, &buffer_blocks);

        // 4.generate mipmap for each image if needed.
        use crate::image::instance::base::mipmap::generate_mipmaps;
        generate_mipmaps(recorder, &self.image_type, &self.info_indices, infos);

        // 5.make image barrier transition for shader reading.
        self.prepare_shader_read(recorder, infos);

        // done. Keep the staging buffer data until the command buffer executes.
        self.staging_repository = Some(staging_repository);

        Ok(())
    }
}

impl SampleImageBarrierBundle {

    pub fn new(dst_stage: ImagePipelineStage, image_type: ImageInstanceType, indices: Vec<usize>) -> SampleImageBarrierBundle {
        SampleImageBarrierBundle {
            image_type,
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

            match infos[info_index].backend.storage.source {
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

    fn upload_image_data(&self, copyer: &DataCopyer, infos: &mut [ImageAllotCI], src_blocks: &[GsImgsrcBuffer]) {

        match self.image_type {
            | ImageInstanceType::CombinedImageSampler { .. }
            | ImageInstanceType::SampledImage { .. } => {
                use crate::image::instance::base::upload::upload_2d_image_data;
                upload_2d_image_data(copyer, &self.info_indices, infos, src_blocks);
            },
            | ImageInstanceType::CubeMapImage { .. } => {
                use crate::image::instance::cubemap::upload::upload_cube_image_data;
                upload_cube_image_data(copyer, &self.info_indices, infos, src_blocks);
            },
            | ImageInstanceType::DepthStencilImage { .. }
            | ImageInstanceType::DepthStencilAttachment => {
                unreachable!("Depth Stencil Images shouldn't be uploaded any data.")
            },
        }
    }

    fn prepare_shader_read(&self, recorder: &GsCmdRecorder<Transfer>, infos: &mut Vec<ImageAllotCI>) {

        let final_barriers = self.info_indices.iter().map(|&index| {

            let image_info = &mut infos[index];
            // make all mip levels readable in shader.
            // TODO: Consider the base layer and layer count.
            let all_mip_levels = image_info.backend.view_ci.subrange.clone()
                .with_mip_level(0, image_info.backend.image_ci.property.mip_levels);

            shader_read_barrier(image_info, all_mip_levels)

        }).collect();

        recorder.image_pipeline_barrier(
            vk::PipelineStageFlags::TRANSFER, // src stage
            self.dst_stage.into(),            // dst stage
            vk::DependencyFlags::empty(),
            final_barriers
        );
    }
}

/// return the image barrier used to make image prepare the be sampled from shader.
fn shader_read_barrier(info: &mut ImageAllotCI, subrange: ImageSubRange) -> ImageBarrierCI {

    match info.backend.image_ci.property.mipmap {
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
