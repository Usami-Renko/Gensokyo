
use ash::vk;

use crate::command::IntoVKBarrier;

#[allow(dead_code)]
pub struct MemoryBarrierCI {

}

impl IntoVKBarrier for MemoryBarrierCI {
    type BarrierType = vk::MemoryBarrier;

    fn into_barrier(self) -> Self::BarrierType {

        unimplemented!()
    }
}
