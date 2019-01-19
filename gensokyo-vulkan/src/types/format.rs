
use ash::vk;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct GsFormat(pub(crate) vk::Format);

impl GsFormat {
    pub const UNDEFINED  : GsFormat = GsFormat(vk::Format::UNDEFINED);
    pub const RGBA8_UNORM: GsFormat = GsFormat(vk::Format::R8G8B8A8_UNORM);
    pub const D32_SFLOAT : GsFormat = GsFormat(vk::Format::D32_SFLOAT);
    pub const D24_UNORM_S8_UINT: GsFormat = GsFormat(vk::Format::D24_UNORM_S8_UINT);
    pub const D32_SFLOAT_S8_UINT: GsFormat = GsFormat(vk::Format::D32_SFLOAT_S8_UINT);

    pub(crate) fn any(f: vk::Format) -> GsFormat {
        GsFormat(f)
    }
}
