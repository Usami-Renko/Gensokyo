
use ash::vk;

use command::IntoVKBarrier;

#[allow(dead_code)]
pub struct HaMemoryBarrier {

}

impl IntoVKBarrier for HaMemoryBarrier {
    type BarrierType = vk::MemoryBarrier;

    fn into_barrier(self) -> Self::BarrierType {

        unimplemented!()
    }
}
