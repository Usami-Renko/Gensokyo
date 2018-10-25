
pub struct VKWrapperInfo<C, I>
    where C: ?Sized,
          I: Sized {

    pub content: Box<C>,
    pub info: I,
}

pub trait VKWrapperContent<C> {}
