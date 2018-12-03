
pub trait IntoVKBarrier {
    type BarrierType;

    fn into_barrier(self) -> Self::BarrierType;
}
