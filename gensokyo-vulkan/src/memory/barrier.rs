
use ash::vk;

use crate::command::IntoVKBarrier;

#[allow(dead_code)]
pub struct GsMemoryBarrier {

}

impl IntoVKBarrier for GsMemoryBarrier {
    type BarrierType = vk::MemoryBarrier;

    fn into_barrier(self) -> Self::BarrierType {

        unimplemented!()
    }
}
