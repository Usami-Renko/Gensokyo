
use ash::vk;

use crate::core::GsDevice;

use crate::types::format::{ Format, TexBCCompressFormat, TexASTCLdrCompressFormat, TexETCCompressFormat };
use crate::types::VK_TRUE;

#[derive(Debug, Clone)]
pub enum GsImageFormat {

    Uncompressed(Format),
    CompressionBC(TexBCCompressFormat),
    CompressionASTCLdr(TexASTCLdrCompressFormat),
    CompressionETC2(TexETCCompressFormat),
}

impl Default for GsImageFormat {

    fn default() -> GsImageFormat {
        GsImageFormat::Uncompressed(Format::RGBA8_UNORM)
    }
}

impl GsImageFormat {

    pub(crate) fn is_support_by_device(&self, device: &GsDevice) -> bool {

        match self {
            | GsImageFormat::Uncompressed(_) => {
                true
            },
            | GsImageFormat::CompressionBC(_) => {
                device.phys.features.enable_features().texture_compression_bc == VK_TRUE
            },
            | GsImageFormat::CompressionASTCLdr(_) => {
                device.phys.features.enable_features().texture_compression_astc_ldr == VK_TRUE
            },
            | GsImageFormat::CompressionETC2(_) => {
                device.phys.features.enable_features().texture_compression_etc2 == VK_TRUE
            },
        }
    }
}

impl From<GsImageFormat> for vk::Format {

    fn from(f: GsImageFormat) -> vk::Format {
        match f {
            | GsImageFormat::Uncompressed(f) => f.into(),
            | GsImageFormat::CompressionBC(f) => f.into(),
            | GsImageFormat::CompressionASTCLdr(f) => f.into(),
            | GsImageFormat::CompressionETC2(f) => f.into(),
        }
    }
}

impl From<GsImageFormat> for Format {

    fn from(f: GsImageFormat) -> Format {
        match f {
            | GsImageFormat::Uncompressed(f) => f.into(),
            | GsImageFormat::CompressionBC(f) => f.into(),
            | GsImageFormat::CompressionASTCLdr(f) => f.into(),
            | GsImageFormat::CompressionETC2(f) => f.into(),
        }
    }
}
