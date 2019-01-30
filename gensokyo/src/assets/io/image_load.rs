
use image;
use image::GenericImageView;

use gsvk::image::storage::{ ImageStorageInfo, ImageSource, ImageData };
use gsvk::types::{ vkbytes, vkDim3D };
use gsvk::types::format::GsFormat;

use crate::assets::error::AssetsError;
use crate::error::{ GsResult, GsError };

use std::path::Path;
use std::mem;

pub struct ImageLoader {

    config: ImageLoadConfig,
}

impl From<ImageLoadConfig> for ImageLoader {

    fn from(config: ImageLoadConfig) -> ImageLoader {
        ImageLoader { config }
    }
}

impl ImageLoader {

    pub fn load_2d(&self, path: &Path) -> GsResult<ImageStorageInfo> {

        let mut image_obj = image::open(path)
            .map_err(|e| GsError::assets(AssetsError::Image(e)))?;

        if self.config.flip_vertical {
            image_obj = image_obj.flipv();
        }
        if self.config.flip_horizontal {
            image_obj = image_obj.fliph();
        }

        let width  = image_obj.width();
        let height = image_obj.height();

        let image_size = ((mem::size_of::<u8>() as u32) * width * height * self.config.byte_per_pixel) as vkbytes;
        let data = if self.config.force_rgba {
            match &image_obj {
                | image::DynamicImage::ImageLuma8(_)
                | image::DynamicImage::ImageBgr8(_)
                | image::DynamicImage::ImageRgb8(_) => image_obj.to_rgba().into_raw(),
                | image::DynamicImage::ImageLumaA8(_)
                | image::DynamicImage::ImageBgra8(_)
                | image::DynamicImage::ImageRgba8(_) => image_obj.raw_pixels(),
            }
        } else {
            image_obj.raw_pixels()
        };

        let info = ImageStorageInfo {
            source: ImageSource::UploadData(ImageData::new(data, image_size)),
            dimension: vkDim3D {
                // TODO: Currently not support muti-level image.
                width, height, depth: 1
            },
            format: self.config.img_format,
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
    pub img_format: GsFormat,
}
