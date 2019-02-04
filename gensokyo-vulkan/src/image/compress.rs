
use crate::core::GsDevice;

use crate::image::target::ImageTgtCI;

use crate::types::VK_TRUE;

#[derive(Debug, Clone, Copy)]
pub enum ImageCompressType {
    Uncompressed,
    CompressionBC,
    CompressionASTCLdr,
    CompressionETC2,
}

impl ImageCompressType {

    pub(crate) fn is_support_by_device(&self, device: &GsDevice, image_ci: &ImageTgtCI) -> bool {

        match self {
            | ImageCompressType::Uncompressed => {
                true
            },
            | ImageCompressType::CompressionBC => {
                device.phys.features.enable_features().texture_compression_bc == VK_TRUE
            },
            | ImageCompressType::CompressionASTCLdr => {
                device.phys.features.enable_features().texture_compression_astc_ldr == VK_TRUE
            },
            | ImageCompressType::CompressionETC2 => {
                device.phys.features.enable_features().texture_compression_etc2 == VK_TRUE
            },
        }
    }
}
