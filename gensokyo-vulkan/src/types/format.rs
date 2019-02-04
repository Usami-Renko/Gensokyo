
use ash::vk;

macro_rules! impl_format_convert {
    ($raw_format:ident, $new_format:ident) => {
        impl From<$raw_format> for $new_format {

            fn from(f: $raw_format) -> $new_format {
                Format(f)
            }
        }

        impl From<$new_format> for $raw_format {

            fn from(f: $new_format) -> $raw_format {
                f.0
            }
        }
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Format(vk::Format);

impl Format {
    pub const UNDEFINED          : Format = Format(vk::Format::UNDEFINED);
    pub const RGBA8_UNORM        : Format = Format(vk::Format::R8G8B8A8_UNORM);
    pub const D32_SFLOAT         : Format = Format(vk::Format::D32_SFLOAT);
    pub const D24_UNORM_S8_UINT  : Format = Format(vk::Format::D24_UNORM_S8_UINT);
    pub const D32_SFLOAT_S8_UINT : Format = Format(vk::Format::D32_SFLOAT_S8_UINT);

    pub(crate) fn any(f: vk::Format) -> Format {
        Format(f)
    }
}

impl_format_convert!(vk::Format, Format);

pub struct TexBCCompressFormat(pub(crate) vk::Format);

impl TexBCCompressFormat {


    pub const BC1_RGB_UNORM  : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC1_RGB_UNORM_BLOCK);
    pub const BC1_RGBA_UNORM : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC1_RGBA_UNORM_BLOCK);
    pub const BC1_RGB_SRGB   : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC1_RGB_SRGB_BLOCK);
    pub const BC1_RGBA_SRGB  : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC1_RGBA_SRGB_BLOCK);

    pub const BC2_UNORM      : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC2_UNORM_BLOCK);
    pub const BC2_SRGB       : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC2_SRGB_BLOCK);

    pub const BC3_UNORM      : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC3_UNORM_BLOCK);
    pub const BC3_SRGB       : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC3_SRGB_BLOCK);

    pub const BC4_UNORM      : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC4_UNORM_BLOCK);
    pub const BC4_SNORM      : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC4_SNORM_BLOCK);

    pub const BC5_UNORM      : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC5_UNORM_BLOCK);
    pub const BC5_SNROM      : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC5_SNORM_BLOCK);

    pub const BC6H_UFLOAT    : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC6H_UFLOAT_BLOCK);
    pub const BC6H_SFLOAT    : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC6H_SFLOAT_BLOCK);

    pub const BC7_SRGB       : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC7_SRGB_BLOCK);
    pub const BC7_UNORM      : TexBCCompressFormat = TexBCCompressFormat(vk::Format::BC7_UNORM_BLOCK);
}

impl From<TexBCCompressFormat> for Format {

    fn from(v: TexBCCompressFormat) -> Format {
        Format(v.0)
    }
}

impl From<TexBCCompressFormat> for vk::Format {

    fn from(f: TexBCCompressFormat) -> vk::Format {
        f.0
    }
}

pub struct TexASTCLdrCompressFormat(pub(crate) vk::Format);

impl TexASTCLdrCompressFormat {

    pub const ASTC_4X4_UNORM   : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_4X4_UNORM_BLOCK);
    pub const ASTC_5X4_UNORM   : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_5X4_UNORM_BLOCK);
    pub const ASTC_5X5_UNORM   : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_5X5_UNORM_BLOCK);
    pub const ASTC_6X5_UNORM   : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_6X5_UNORM_BLOCK);
    pub const ASTC_6X6_UNORM   : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_6X6_UNORM_BLOCK);
    pub const ASTC_8X5_UNORM   : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_8X5_UNORM_BLOCK);
    pub const ASTC_8X6_UNORM   : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_8X6_UNORM_BLOCK);
    pub const ASTC_8X8_UNORM   : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_8X8_UNORM_BLOCK);
    pub const ASTC_10X5_UNORM  : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_10X5_UNORM_BLOCK);
    pub const ASTC_10X6_UNORM  : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_10X6_UNORM_BLOCK);
    pub const ASTC_10X8_UNORM  : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_10X8_UNORM_BLOCK);
    pub const ASTC_10X10_UNORM : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_10X10_UNORM_BLOCK);
    pub const ASTC_12X10_UNORM : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_12X10_UNORM_BLOCK);
    pub const ASTC_12X12_UNORM : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_12X12_UNORM_BLOCK);

    pub const ASTC_4X4_SRGB    : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_4X4_SRGB_BLOCK);
    pub const ASTC_5X4_SRGB    : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_5X4_SRGB_BLOCK);
    pub const ASTC_5X5_SRGB    : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_5X5_SRGB_BLOCK);
    pub const ASTC_6X5_SRGB    : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_6X5_SRGB_BLOCK);
    pub const ASTC_6X6_SRGB    : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_6X6_SRGB_BLOCK);
    pub const ASTC_8X5_SRGB    : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_8X5_SRGB_BLOCK);
    pub const ASTC_8X6_SRGB    : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_8X6_SRGB_BLOCK);
    pub const ASTC_10X5_SRGB   : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_10X5_SRGB_BLOCK);
    pub const ASTC_10X6_SRGB   : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_10X6_SRGB_BLOCK);
    pub const ASTC_10X8_SRGB   : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_10X8_SRGB_BLOCK);
    pub const ASTC_10X10_SRGB  : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_10X10_SRGB_BLOCK);
    pub const ASTC_12X10_SRGB  : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_12X10_SRGB_BLOCK);
    pub const ASTC_12X12_SRGB  : TexASTCLdrCompressFormat = TexASTCLdrCompressFormat(vk::Format::ASTC_12X12_SRGB_BLOCK);
}

impl From<TexASTCLdrCompressFormat> for Format {

    fn from(v: TexASTCLdrCompressFormat) -> Format {
        Format(v.0)
    }
}

impl From<TexASTCLdrCompressFormat> for vk::Format {

    fn from(f: TexASTCLdrCompressFormat) -> vk::Format {
        f.0
    }
}

pub struct TexETCCompressFormat(pub(crate) vk::Format);

impl TexETCCompressFormat {
    
    pub const ETC2_RGB8_UNORM   : TexETCCompressFormat = TexETCCompressFormat(vk::Format::ETC2_R8G8B8_UNORM_BLOCK);
    pub const ETC2_RGB8A1_UNORM : TexETCCompressFormat = TexETCCompressFormat(vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK);
    pub const ETC2_RGBA8_UNORM  : TexETCCompressFormat = TexETCCompressFormat(vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK);
    
    pub const ETC2_RGB8_SRGB    : TexETCCompressFormat = TexETCCompressFormat(vk::Format::ETC2_R8G8B8_SRGB_BLOCK);
    pub const ETC2_RGB8A1_SRGB  : TexETCCompressFormat = TexETCCompressFormat(vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK);
    pub const ETC2_RGBA8_SRGB   : TexETCCompressFormat = TexETCCompressFormat(vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK);

    pub const EAC_R11_UNORM     : TexETCCompressFormat = TexETCCompressFormat(vk::Format::EAC_R11_UNORM_BLOCK);
    pub const EAC_R11_SNORM     : TexETCCompressFormat = TexETCCompressFormat(vk::Format::EAC_R11_SNORM_BLOCK);

    pub const EAC_R11G11_UNORM  : TexETCCompressFormat = TexETCCompressFormat(vk::Format::EAC_R11G11_UNORM_BLOCK);
    pub const EAC_R11G11_SNORM  : TexETCCompressFormat = TexETCCompressFormat(vk::Format::EAC_R11G11_SNORM_BLOCK);
}

impl From<TexETCCompressFormat> for Format {

    fn from(v: TexETCCompressFormat) -> Format {
        Format(v.0)
    }
}

impl From<TexETCCompressFormat> for vk::Format {

    fn from(f: TexETCCompressFormat) -> vk::Format {
        f.0
    }
}
