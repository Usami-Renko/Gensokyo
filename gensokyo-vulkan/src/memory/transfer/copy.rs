
use ash::vk;

use crate::core::device::{ GsDevice, GsLogicalDevice };
use crate::core::device::queue::GsTransfer;

use crate::buffer::BufferCopiable;
use crate::image::ImageCopiable;
use crate::command::GsCommandRecorder;
use crate::memory::error::AllocatorError;

pub struct DataCopyer {

    transfer: GsTransfer,
    recorder: GsCommandRecorder,
}

impl DataCopyer {

    pub fn new(device: &GsDevice) -> Result<DataCopyer, AllocatorError> {

        let transfer = GsLogicalDevice::transfer(device);
        let command = transfer.command()?;
        let recorder = command.setup_record(device);

        let _ = recorder.begin_record(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)?;

        let copyer = DataCopyer {
            transfer, recorder,
        };

        Ok(copyer)
    }

    // TODO: Currently only support copy the whole buffer to another buffer.
    pub fn copy_buffer_to_buffer(&self, src: &impl BufferCopiable, dst: &impl BufferCopiable) -> &DataCopyer {

        let src = src.copy_info();
        let dst = dst.copy_info();

        // TODO: Only support one region.
        let copy_region = [
            vk::BufferCopy {
                // TODO: Only support copy buffer from beginning.
                src_offset: src.offset,
                dst_offset: dst.offset,
                size: src.size,
            },
        ];

        let _ = self.recorder.copy_buffer(src.handle, dst.handle, &copy_region);

        self
    }

    // TODO: Currently only support copy the whole buffer to the image.
    pub fn copy_buffer_to_image(&self, src: &impl BufferCopiable, dst: &impl ImageCopiable) -> &DataCopyer {

        let src = src.copy_info();
        let dst = dst.copy_info();

        // TODO: Only support one region.
        let copy_regions = [
            vk::BufferImageCopy {
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
            },
        ];

        let _ = self.recorder.copy_buffer_to_image(src.handle, dst.handle, dst.layout, &copy_regions);

        self
    }

    // TODO: Currently only support copy the whole image data to buffer.
    pub fn copy_image_to_buffer(&self, src: &impl ImageCopiable, dst: &impl BufferCopiable) -> &DataCopyer {

        let src = src.copy_info();
        let dst = dst.copy_info();

        let copy_regions = [
            vk::BufferImageCopy {
                buffer_offset: 0,
                buffer_row_length  : 0,
                buffer_image_height: 0,
                image_subresource: src.sub_resource_layers,
                image_offset: vk::Offset3D { x:0, y: 0, z: 0 },
                image_extent: src.extent,
            },
        ];

        let _ = self.recorder.copy_image_to_buffer(src.handle, src.layout, dst.handle, &copy_regions);

        self
    }

    // TODO: Currently only support copy the whole image data to another image.
    pub fn copy_image(&self, src: &impl ImageCopiable, dst: &impl ImageCopiable) -> &DataCopyer {

        let src = src.copy_info();
        let dst = dst.copy_info();

        let copy_regions = [
            vk::ImageCopy {
                src_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                dst_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                src_subresource: src.sub_resource_layers,
                dst_subresource: dst.sub_resource_layers,
                extent: src.extent,
            },
        ];

        let _ = self.recorder.copy_image(src.handle, src.layout, dst.handle, dst.layout, &copy_regions);

        self
    }

    pub fn done(&mut self) -> Result<(), AllocatorError> {

        let command = self.recorder.end_record()?;
        self.transfer.commit(command);
        self.transfer.excute()?;

        Ok(())
    }

    pub fn recorder(&self) -> &GsCommandRecorder {
        &self.recorder
    }
}
