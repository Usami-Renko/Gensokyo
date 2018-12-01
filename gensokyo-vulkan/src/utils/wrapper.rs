
pub struct VKWrapperInfo<C, I>
    where C: ?Sized,
          I: Sized {

    pub content: Box<C>, // C is not a trait object.
    pub info: I,
}

pub trait VKWrapperContent<C> {}
