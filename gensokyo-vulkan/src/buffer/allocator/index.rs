
use buffer::instance::UniformAttachment;

pub struct BufferBlockIndex {

    pub(crate) value: usize,
    pub(crate) attachment: Option<BufferDistAttachment>,
}

pub enum BufferDistAttachment {

    Uniform(UniformAttachment)
}
