
use ash::vk;

use command::IntoVKBarrier;

struct HaBufferBarrier {

}

impl IntoVKBarrier for HaBufferBarrier {
    type BarrierType = vk::BufferMemoryBarrier;

    fn into_barrier(self) -> Self::BarrierType {

        unimplemented!()
    }
}
