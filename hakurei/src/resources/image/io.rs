
use image;
use image::GenericImage;

use vk::resources::error::ImageError;
use vk::utils::types::{ vkMemorySize, vkformat, vkDimension2D, vkDimension3D };

use std::path::Path;
use std::mem;


impl ImageStorageInfo {

    pub fn from_load2d(path: &Path, config: &ImageLoadConfig) -> Result<ImageStorageInfo, ImageError> {

        let mut image_obj = image::open(path)
            .or(Err(ImageError::SourceLoadError))?;

        if config.flip_vertical {
            image_obj = image_obj.flipv();
        }
        if config.flip_horizontal {
            image_obj = image_obj.fliph();
        }

        let width  = image_obj.width();
        let height = image_obj.height();

        let image_size = ((mem::size_of::<u8>() as u32) * width * height * config.byte_per_pixel) as vkMemorySize;
        let data = if config.force_rgba {
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
            source: ImageSource::UploadData(ImageData::new(data, image_size)),
            dimension: vkDimension3D {
                // TODO: Currently not support muti-level image.
                width, height, depth: 1
            },
            format: config.img_format,
        };

        Ok(info)
    }

    pub fn from_unload(dimension: vkDimension2D, format: vkformat) -> Result<ImageStorageInfo, ImageError> {

        let info = ImageStorageInfo {
            source: ImageSource::NoSource,
            format,
            // TODO: Currently not support muti-level image.
            dimension: vkDimension3D {
                width : dimension.width,
                height: dimension.height,
                depth : 1,
            }
        };

        Ok(info)
    }
}

#[derive(Debug, Clone)]
pub struct ImageLoadConfig {

    /// flip_vertical define whether to flip vertical when loading image.
    pub flip_vertical: bool,
    /// flip_horizontal define whether to flip horizontal when loading image.
    pub flip_horizontal: bool,
    /// byte_per_pixel define the byte count in per pixel.
    pub byte_per_pixel: u32,
    /// force_rgba define whether to load the image from file with rgba channel.
    pub force_rgba: bool,
    // TODO: Currently only support R8g8b8a8Unorm format.
    pub img_format: vkformat,
}
