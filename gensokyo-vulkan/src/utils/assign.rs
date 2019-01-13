
pub struct GsAssignIndex<I> {

    pub(crate) allot_info: I,
    pub(crate) assign_index: usize,
}

impl<I> GsAssignIndex<I> {

    pub(crate) fn take_info(self) -> I {
        self.allot_info
    }
}
