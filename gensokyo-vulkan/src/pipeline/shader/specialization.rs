
use ash::vk;

#[allow(dead_code)]
struct GsShaderSpecialize<T>
    where
        T: SpecializeData {

    data: T,
}

impl<T> GsShaderSpecialize<T>
    where
        T: SpecializeData {



}

pub trait SpecializeData {

    fn map_entries() -> Vec<vk::SpecializationMapEntry>;
}
