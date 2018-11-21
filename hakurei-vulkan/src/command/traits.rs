
use descriptor::DescriptorSetEntity;

pub trait IntoVKBarrier {
    type BarrierType;

    fn into_barrier(self) -> Self::BarrierType;
}

pub trait ToDescriptorSetEntity {

    fn entity(&self) -> DescriptorSetEntity;
}
