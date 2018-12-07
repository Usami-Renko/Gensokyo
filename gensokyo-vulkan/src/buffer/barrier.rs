
use ash::vk;

use crate::command::IntoVKBarrier;

#[allow(dead_code)]
struct GsBufferBarrier {

}

impl IntoVKBarrier for GsBufferBarrier {
    type BarrierType = vk::BufferMemoryBarrier;

    fn into_barrier(self) -> Self::BarrierType {

        unimplemented!()
    }
}
