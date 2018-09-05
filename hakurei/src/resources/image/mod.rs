
pub use self::image::ImageDescInfo;
pub use self::view::ImageViewDescInfo;
pub use self::flag::{ ImageLayout, ImageAspectFlag, ImageCreateFlag, ImageUsageFlag };
pub use self::item::ImageViewItem;
pub use self::sampler::{ HaSampler, SamplerDescInfo };
pub use self::enums::{ ImageType, ImageViewType, ImageTiling };

use ash;

// TODO: Rewrap these value
pub type Filter        = ash::vk::Filter;
pub type MipmapMode    = ash::vk::SamplerMipmapMode;
pub type CompareOp     = ash::vk::CompareOp;
pub type BorderColor   = ash::vk::BorderColor;

pub(crate) use self::image::HaImage;
pub(crate) use self::view::HaImageView;
pub(crate) use self::io::{ load_texture, ImageStorageInfo };

mod image;
mod view;
mod sampler;
mod flag;
mod io;
mod item;
mod enums;