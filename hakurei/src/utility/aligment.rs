
use ash::vk;

#[inline]
pub fn bind_to_alignment(bind_value: vk::DeviceSize, alignment: vk::DeviceSize) -> vk::DeviceSize {
    if bind_value < alignment {
        alignment
    } else {
        bind_value - (bind_value % alignment) + alignment
    }
}