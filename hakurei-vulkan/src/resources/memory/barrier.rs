
use ash::vk;

use resources::command::IntoVKBarrier;

pub struct HaMemoryBarrier {

}

impl IntoVKBarrier for HaMemoryBarrier {
    type BarrierType = vk::MemoryBarrier;

    fn into_barrier(self) -> Self::BarrierType {

        unimplemented!()
    }
}
