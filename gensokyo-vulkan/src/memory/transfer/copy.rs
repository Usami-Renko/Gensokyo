
use ash::vk;

use crate::core::GsDevice;
use crate::core::device::GsLogicalDevice;
use crate::core::device::queue::GsTransfer;

use crate::buffer::{ BufferFullCopyInfo, BufferCopyRanges };
use crate::image::{ ImageFullCopyInfo, ImageRangesCopyInfo };
use crate::command::{ GsCmdRecorder, GsCmdTransferApi };
use crate::error::VkResult;
use crate::utils::phantom::Transfer;

pub struct DataCopyer {

    transfer: GsTransfer,
    recorder: GsCmdRecorder<Transfer>,
}

impl DataCopyer {

    // Implement TryFrom instead of this new func.
    pub fn new(device: &GsDevice) -> VkResult<DataCopyer> {

        let transfer = GsLogicalDevice::transfer(device)?;
        let command = transfer.command()?;
        let recorder = GsCmdRecorder::create_copy(device, command);

        let _ = recorder.begin_record(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)?;

        let copyer = DataCopyer { transfer, recorder };
        Ok(copyer)
    }

    /// Copy the whole buffer to another buffer. Both buffer must be the same size.
    pub fn copy_buffer_to_buffer(&self, src: BufferFullCopyInfo, dst: BufferFullCopyInfo) -> &DataCopyer {

        debug_assert_eq!(src.size, dst.size, "The size of both buffers must be the same during copy.");

        let copy_region = vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size: src.size,
        };

        let _ = self.recorder.copy_buf2buf(src.handle, dst.handle, &[copy_region]);

        self
    }

    /// Copy the whole buffer to an image. The size of buffer and the image must be the same.
    pub fn copy_buffer_to_image(&self, src: BufferFullCopyInfo, dst: ImageFullCopyInfo) -> &DataCopyer {

        let copy_region = vk::BufferImageCopy {
            // the image data must start at the beginning of buffer to be copied from.
            buffer_offset: 0,
            // TODO: the following options are not configurable.
            // Specifying 0 for both indicates that the pixels are simply tightly packed.
            buffer_row_length  : 0,
            buffer_image_height: 0,
            image_subresource: dst.sub_resource_layers,
            // imageOffset selects the initial x, y, z offsets in texels of the sub-region of the source or destination image data.
            image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
            // imageExtent is the size in texels of the image to copy in width, height and depth.
            image_extent: dst.extent,
        };

        let _ = self.recorder.copy_buf2img(src.handle, dst.handle, dst.layout, &[copy_region]);

        self
    }

    pub fn copy_buffer_to_image_ranges(&self, src: BufferCopyRanges, dst: ImageRangesCopyInfo) -> &DataCopyer {

        debug_assert_eq!(src.subrange_count(), dst.ranges.len());

        let copy_regions: Vec<vk::BufferImageCopy> = dst.ranges.iter().zip(src.offsets)
            .map(|(dst_range, offset)| {

            vk::BufferImageCopy {
                // the image data must start at the beginning of buffer to be copied from.
                buffer_offset: offset,
                // TODO: the following options are not configurable.
                // Specifying 0 for both indicates that the pixels are simply tightly packed.
                buffer_row_length  : 0,
                buffer_image_height: 0,
                image_subresource: dst_range.sub_resource_layers,
                // imageOffset selects the initial x, y, z offsets in texels of the sub-region of the source or destination image data.
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                // imageExtent is the size in texels of the image to copy in width, height and depth.
                image_extent: dst_range.extent,
            }
        }).collect();

        let _ = self.recorder.copy_buf2img(src.handle, dst.handle, dst.layout, &copy_regions);

        self
    }

    /// Copy the whole image data to a buffer. The size of image and the buffer must be the same.
    pub fn copy_image_to_buffer(&self, src: ImageFullCopyInfo, dst: BufferFullCopyInfo) -> &DataCopyer {

        let copy_region = vk::BufferImageCopy {
            buffer_offset: 0,
            buffer_row_length  : 0,
            buffer_image_height: 0,
            image_subresource: src.sub_resource_layers,
            image_offset: vk::Offset3D { x:0, y: 0, z: 0 },
            image_extent: src.extent,
        };

        let _ = self.recorder.copy_img2buf(src.handle, src.layout, dst.handle, &[copy_region]);

        self
    }

    /// Copy the whole image data to another image. Both the images must be the same size.
    pub fn copy_image(&self, src: ImageFullCopyInfo, dst: ImageFullCopyInfo) -> &DataCopyer {

        let copy_region = vk::ImageCopy {
            src_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
            dst_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
            src_subresource: src.sub_resource_layers,
            dst_subresource: dst.sub_resource_layers,
            extent: src.extent,
        };

        let _ = self.recorder.copy_img2img(src.handle, src.layout, dst.handle, dst.layout, &[copy_region]);

        self
    }

    pub fn done(&mut self) -> VkResult<()> {

        let command = self.recorder.end_record()?;
        self.transfer.commit(command);
        self.transfer.execute()?;

        Ok(())
    }

    #[inline]
    pub fn recorder(&self) -> &GsCmdRecorder<Transfer> {
        &self.recorder
    }
}
