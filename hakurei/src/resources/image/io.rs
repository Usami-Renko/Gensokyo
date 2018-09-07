
use ash::vk;

use image;
use image::GenericImage;

use utility::dimension::Dimension3D;

use config::image::{ IMAGE_FLIP_VERTICAL, IMAGE_FLIP_HORIZONTAL, BYTE_PER_PIXEL, FORCE_RGBA };
use resources::error::ImageError;

use std::path::Path;
use std::mem;

pub struct ImageStorageInfo {
    pub data: Vec<u8>,
    pub size: vk::DeviceSize,
    /// dimension describes the number of data elements in each dimension of the base level.
    pub dimension: Dimension3D,
    /// format describes the format and type of the data elements that will be contained in the image.
    pub format: vk::Format,
}

pub fn load_texture(path: &Path) -> Result<ImageStorageInfo, ImageError> {

    let mut image_obj = image::open(path)
        .or(Err(ImageError::SourceLoadError))?;

    if IMAGE_FLIP_VERTICAL {
        image_obj = image_obj.flipv();
    }
    if IMAGE_FLIP_HORIZONTAL {
        image_obj = image_obj.fliph();
    }

    let width  = image_obj.width();
    let height = image_obj.height();

    let image_size = ((mem::size_of::<u8>() as u32) * width * height * BYTE_PER_PIXEL) as vk::DeviceSize;
    let data = if FORCE_RGBA {
        match &image_obj {
            | image::DynamicImage::ImageLuma8(_)
            | image::DynamicImage::ImageRgb8(_) => image_obj.to_rgba().into_raw(),
            | image::DynamicImage::ImageLumaA8(_)
            | image::DynamicImage::ImageRgba8(_) => image_obj.raw_pixels(),
        }
    } else {
        image_obj.raw_pixels()
    };


    let info = ImageStorageInfo {
        data,
        size: image_size,
        dimension: Dimension3D {
            // TODO: Currently not support muti-level image.
            width, height, depth: 1
        },
        // TODO: Currently only support this specific format.
        format: vk::Format::R8g8b8a8Unorm,
    };
    Ok(info)
}
