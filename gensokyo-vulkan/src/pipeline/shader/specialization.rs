
use ash::vk;

pub struct GsShaderSpecialize<T>
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
