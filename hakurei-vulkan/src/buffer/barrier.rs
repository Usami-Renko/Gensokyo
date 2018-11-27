
use ash::vk;

use command::IntoVKBarrier;

#[allow(dead_code)]
struct HaBufferBarrier {

}

impl IntoVKBarrier for HaBufferBarrier {
    type BarrierType = vk::BufferMemoryBarrier;

    fn into_barrier(self) -> Self::BarrierType {

        unimplemented!()
    }
}
