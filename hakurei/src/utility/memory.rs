
use ash::vk;

#[inline]
pub fn bind_to_alignment(bind_value: vk::DeviceSize, alignment: vk::DeviceSize) -> vk::DeviceSize {
    if bind_value < alignment {
        alignment
    } else {
        bind_value - (bind_value % alignment) + alignment
    }
}

pub fn spaces_to_offsets(spaces: &Vec<vk::DeviceSize>) -> Vec<vk::DeviceSize> {

    let mut current: vk::DeviceSize = 0;
    let mut offsets = vec![];
    for &space in spaces.iter() {
        offsets.push(current);
        current += space;
    }

    offsets
}
