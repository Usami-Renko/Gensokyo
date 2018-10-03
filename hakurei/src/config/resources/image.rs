
pub const IMAGE_FLIP_VERTICAL  : bool = false;
pub const IMAGE_FLIP_HORIZONTAL: bool = false;
pub const BYTE_PER_PIXEL: u32 = 4;
pub const FORCE_RGBA: bool = true;

#[derive(Debug, Clone)]
pub struct ImageLoadConfig {

    /// flip_vertical define whether to flip vertical when loading image.
    pub flip_vertical  : bool,
    /// flip_horizontal define whether to flip horizontal when loading image.
    pub flip_horizontal: bool,

    /// byte_per_pixel define the byte count in per pixel.
    byte_per_pixel: u32,
    /// force_rgba define whether to load the image from file with rgba channel.
    force_rgba: bool,
}

impl Default for ImageLoadConfig {

    fn default() -> ImageLoadConfig {
        ImageLoadConfig {
            flip_vertical  : false,
            flip_horizontal: false,

            byte_per_pixel: 4,
            force_rgba: true,
        }
    }
}
