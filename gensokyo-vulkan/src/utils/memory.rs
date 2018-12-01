
use types::vkbytes;

#[inline]
pub fn bind_to_alignment(bind_value: vkbytes, alignment: vkbytes) -> vkbytes {
    if bind_value < alignment {
        alignment
    } else {
        bind_value - (bind_value % alignment) + alignment
    }
}

pub fn spaces_to_offsets(spaces: &Vec<vkbytes>) -> Vec<vkbytes> {

    let mut current: vkbytes = 0;
    let mut offsets = vec![];

    for &space in spaces.iter() {
        offsets.push(current);
        current += space;
    }

    offsets
}
