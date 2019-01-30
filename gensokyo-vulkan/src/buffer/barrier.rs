
use ash::vk;

use crate::command::IntoVKBarrier;

#[allow(dead_code)]
struct BufferBarrierCI {

}

impl IntoVKBarrier for BufferBarrierCI {
    type BarrierType = vk::BufferMemoryBarrier;

    fn into_barrier(self) -> Self::BarrierType {

        unimplemented!()
    }
}
