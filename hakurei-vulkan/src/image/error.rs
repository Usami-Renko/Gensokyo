
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ImageError {
    SourceLoadError,
    ImageCreationError,
    ViewCreationError,
    NoImageAppendError,
    NotYetAllocateError,
    SamplerCreationError,
}

impl Error for ImageError {}
impl fmt::Display for ImageError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            | ImageError::SourceLoadError      => write!(f, "Failed to load image from source."),
            | ImageError::ImageCreationError   => write!(f, "Failed to create Image Object."),
            | ImageError::ViewCreationError    => write!(f, "Failed to create ImageView Object."),
            | ImageError::NotYetAllocateError  => write!(f, "The image is not allocated yet. Have you forget to call HaImagePreAllocator::append_**_image() before HaImageDistributor::acquire_**_image()?"),
            | ImageError::NoImageAppendError   => write!(f, "There must be images appended to allocator before allocate memory."),
            | ImageError::SamplerCreationError => write!(f, "Failed to create Sampler Object."),
        }
    }
}
