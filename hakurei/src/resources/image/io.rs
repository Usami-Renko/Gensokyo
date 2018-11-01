
use ash::vk;

use image;
use image::GenericImage;

use config::resources::ImageLoadConfig;
use utility::dimension::{ Dimension2D, Dimension3D };
use resources::error::ImageError;

use std::path::Path;
use std::mem;

pub(crate) struct ImageStorageInfo {

    pub source: ImageSource,
    /// dimension describes the number of data elements in each dimension of the base level.
    pub dimension: Dimension3D,
    /// format describes the format and type of the data elements that will be contained in the image.
    pub format: vk::Format,
}

pub(crate) enum ImageSource {
    UploadData(ImageData),
    NoSource,
}

pub(crate) struct ImageData {
    pub data: Vec<u8>,
    pub size: vk::DeviceSize,
}

impl ImageData {

    pub fn new(data: Vec<u8>, size: vk::DeviceSize) -> ImageData {
        ImageData { data, size }
    }
}

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

        let image_size = ((mem::size_of::<u8>() as u32) * width * height * config.byte_per_pixel) as vk::DeviceSize;
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
            dimension: Dimension3D {
                // TODO: Currently not support muti-level image.
                width, height, depth: 1
            },
            // TODO: Currently only support this specific format.
            format: vk::Format::R8g8b8a8Unorm,
        };

        Ok(info)
    }

    pub fn from_unload(dimension: Dimension2D, format: vk::Format) -> Result<ImageStorageInfo, ImageError> {

        let info = ImageStorageInfo {
            source: ImageSource::NoSource,
            format,
            // TODO: Currently not support muti-level image.
            dimension: Dimension3D {
                width : dimension.width,
                height: dimension.height,
                depth : 1,
            }
        };

        Ok(info)
    }
}
