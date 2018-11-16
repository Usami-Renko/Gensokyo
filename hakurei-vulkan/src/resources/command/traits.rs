
use resources::descriptor::DescriptorSetItem;

pub trait IntoVKBarrier {
    type BarrierType;

    fn into_barrier(self) -> Self::BarrierType;
}

pub trait ToDescriptorSetItem {

    fn item(&self) -> DescriptorSetItem;
}
