
use ash::vk;

use crate::core::GsDevice;
use crate::core::device::GsLogicalDevice;
use crate::core::device::queue::GsTransfer;

use crate::buffer::BufferCopyInfo;
use crate::image::ImageCopyInfo;
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

    // TODO: Currently only support copy the whole buffer to another buffer.
    pub fn copy_buffer_to_buffer(&self, src: BufferCopyInfo, dst: BufferCopyInfo) -> &DataCopyer {

        // TODO: Only support one region.
        let copy_region = [
            vk::BufferCopy {
                // TODO: Only support copy buffer from beginning.
                src_offset: src.offset,
                dst_offset: dst.offset,
                size: src.size,
            },
        ];

        let _ = self.recorder.copy_buf2buf(src.handle, dst.handle, &copy_region);

        self
    }

    // TODO: Currently only support copy the whole buffer to the image.
    pub fn copy_buffer_to_image(&self, src: BufferCopyInfo, dst: ImageCopyInfo) -> &DataCopyer {

        // TODO: Only support one region.
        let copy_regions = [
            vk::BufferImageCopy {
                // the image data must start at the beginning of buffer to be copied from.
                buffer_offset: 0,
                // TODO: the following options are not configurable.
                // Specifying 0 for both indicates that the pixels are simply tightly packed.
                buffer_row_length  : 0,
                buffer_image_height: 0,
                image_subresource: dst.sub_resource_layers.0,
                // imageOffset selects the initial x, y, z offsets in texels of the sub-region of the source or destination image data.
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                // imageExtent is the size in texels of the image to copy in width, height and depth.
                image_extent: dst.extent,
            },
        ];

        let _ = self.recorder.copy_buf2img(src.handle, dst.handle, dst.layout, &copy_regions);

        self
    }

    // TODO: Currently only support copy the whole image data to buffer.
    pub fn copy_image_to_buffer(&self, src: ImageCopyInfo, dst: BufferCopyInfo) -> &DataCopyer {

        let copy_regions = [
            vk::BufferImageCopy {
                buffer_offset: 0,
                buffer_row_length  : 0,
                buffer_image_height: 0,
                image_subresource: src.sub_resource_layers.0,
                image_offset: vk::Offset3D { x:0, y: 0, z: 0 },
                image_extent: src.extent,
            },
        ];

        let _ = self.recorder.copy_img2buf(src.handle, src.layout, dst.handle, &copy_regions);

        self
    }

    // TODO: Currently only support copy the whole image data to another image.
    pub fn copy_image(&self, src: ImageCopyInfo, dst: ImageCopyInfo) -> &DataCopyer {

        let copy_regions = [
            vk::ImageCopy {
                src_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                dst_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                src_subresource: src.sub_resource_layers.0,
                dst_subresource: dst.sub_resource_layers.0,
                extent: src.extent,
            },
        ];

        let _ = self.recorder.copy_img2img(src.handle, src.layout, dst.handle, dst.layout, &copy_regions);

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
