
pub use self::traits::{ DescriptorBindingCI, DescriptorMeta, DescriptorArrayMeta };
pub use self::buffer::{ DescriptorBindingBufTgt, DescriptorBindingBufInfo };
pub use self::image::{ DescriptorBindingImgTgt, DescriptorBindingImgInfo };
pub use self::image::{ DescriptorBindingImgArrayTgt, DescriptorBindingImgArrayInfo, ImgArrayBinding };

mod traits;
mod buffer;
mod image;
