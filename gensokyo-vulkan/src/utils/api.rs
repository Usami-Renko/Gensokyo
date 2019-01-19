
use crate::error::VkResult;

pub struct GsAssignIndex<I> {

    pub(crate) convey_info: I,
    pub(crate) assign_index: usize,
}

impl<I> GsAssignIndex<I> {

    pub(crate) fn take_info(self) -> I {
        self.convey_info
    }
}

pub trait GsAllocatorApi<I, R, D>: GsAllotIntoDistributor<D> + Sized {
    type AssignResult;

    fn assign(&mut self, info: I) -> Self::AssignResult;
    fn reset(&mut self);
}

pub trait GsAllotIntoDistributor<D> {

    fn allocate(self) -> VkResult<D>;
}

pub trait GsDistributeApi<R, T, S>: GsDistIntoRepository<S> + Sized {

    fn acquire(&self, index: GsAssignIndex<R>) -> T;
}

pub trait GsDistIntoRepository<S> {

    fn into_repository(self) -> S;
}
