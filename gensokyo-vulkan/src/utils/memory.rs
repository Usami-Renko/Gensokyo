
use crate::types::vkbytes;

#[inline]
pub fn bound_to_alignment(bound_value: vkbytes, alignment: vkbytes) -> vkbytes {

    // Implementation 1.
    // if bound_value % alignment == 0 {
    //     bind_value
    // } else if bound_value < alignment {
    //     alignment
    // } else {
    //     bound_value - (bound_value % alignment) + alignment
    // }

    // Implementation 2.
    // `!` operator will make 1 to 0 or make 0 to 1 for each bit for any integer type.
    (bound_value + alignment - 1) & !(alignment - 1)
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
