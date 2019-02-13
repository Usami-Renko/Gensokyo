
use crate::types::{ vkbytes, vkDim3D };

use crate::image::format::GsImageFormat;

pub struct ImageStorageInfo {

    pub source: ImageSource,
    /// dimension describes the number of data elements in each dimension of the base level.
    pub dimension: vkDim3D,
    /// format describes the format and type of the data elements that will be contained in the image.
    pub format: GsImageFormat,
}

pub enum ImageSource {

    UploadData(ImageData),
    NoSource,
}

pub struct ImageData {

    pub data: Vec<u8>,
    pub size: vkbytes,
}

impl ImageData {

    pub fn new(data: Vec<u8>, size: vkbytes) -> ImageData {

        ImageData { data, size }
    }
}
