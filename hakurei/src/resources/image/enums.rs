
use ash::vk;

use utility::marker::VulkanEnum;

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageType {
    Type1d,
    Type2d,
    Type3d,
}

impl VulkanEnum for ImageType {
    type EnumType = vk::ImageType;

    fn value(&self) -> Self::EnumType {
        match *self {
            | ImageType::Type1d => vk::ImageType::Type1d,
            | ImageType::Type2d => vk::ImageType::Type2d,
            | ImageType::Type3d => vk::ImageType::Type3d,
        }
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageViewType {
    Type1d,
    Type2d,
    Type3d,
    Cube,
    Type1dArray,
    Type2dArray,
    CubeArray,
}

impl VulkanEnum for ImageViewType {
    type EnumType = vk::ImageViewType;

    fn value(&self) -> Self::EnumType {
        match *self {
            | ImageViewType::Type1d      => vk::ImageViewType::Type1d,
            | ImageViewType::Type2d      => vk::ImageViewType::Type2d,
            | ImageViewType::Type3d      => vk::ImageViewType::Type3d,
            | ImageViewType::Cube        => vk::ImageViewType::Cube,
            | ImageViewType::Type1dArray => vk::ImageViewType::Type1dArray,
            | ImageViewType::Type2dArray => vk::ImageViewType::Type2dArray,
            | ImageViewType::CubeArray   => vk::ImageViewType::CubeArray,
        }
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageTiling {
    Linear,
    Optimal,
}

impl VulkanEnum for ImageTiling {
    type EnumType = vk::ImageTiling;

    fn value(&self) -> Self::EnumType {
        match *self {
            | ImageTiling::Linear  => vk::ImageTiling::Linear,
            | ImageTiling::Optimal => vk::ImageTiling::Optimal,
        }
    }
}