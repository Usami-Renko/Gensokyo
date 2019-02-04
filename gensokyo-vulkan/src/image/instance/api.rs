
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::GsImage;
use crate::image::instance::base::GsBackendImage;
use crate::image::compress::ImageCompressType;
use crate::image::mipmap::MipmapMethod;
use crate::image::instance::traits::{ ImageCIApi, ImageCICommonApi, ImageCISpecificApi };
use crate::image::instance::traits::{ ImageTgtCIApi, ImageViewCIApi };
use crate::image::view::ImageSubRange;

use crate::types::vkuint;
use crate::error::VkResult;

pub trait ImageCIInheritApi {

    fn backend(&self) -> &GsBackendImage;
    fn backend_mut(&mut self) -> &mut GsBackendImage;
}

impl<T> ImageTgtCIApi for T
    where
        T: ImageCIInheritApi {

    fn set_tiling(&mut self, tiling: vk::ImageTiling) {
        self.backend_mut().set_tiling(tiling);
    }

    fn set_initial_layout(&mut self, layout: vk::ImageLayout) {
        self.backend_mut().set_initial_layout(layout);
    }

    fn set_samples(&mut self, count: vk::SampleCountFlags, mip_levels: vkuint, array_layers: vkuint) {
        self.backend_mut().set_samples(count, mip_levels, array_layers);
    }

    fn set_share_queues(&mut self, queue_family_indices: Vec<vkuint>) {
        self.backend_mut().set_share_queues(queue_family_indices);
    }

    fn set_compression(&mut self, compression: ImageCompressType) {
        self.backend_mut().set_compression(compression);
    }
}

impl<T> ImageViewCIApi for T
    where
        T: ImageCIInheritApi {

    fn set_mapping_component(&mut self, r: vk::ComponentSwizzle, g: vk::ComponentSwizzle, b: vk::ComponentSwizzle, a: vk::ComponentSwizzle) {
        self.backend_mut().set_mapping_component(r, g, b ,a);
    }

    fn set_subrange(&mut self, value: ImageSubRange) {
        self.backend_mut().set_subrange(value);
    }
}

impl<T> ImageCICommonApi for T
    where
        T: ImageCIInheritApi {

    fn set_mipmap(&mut self, method: MipmapMethod) {
        self.backend_mut().set_mipmap(method);
    }

    fn estimate_mip_levels(&self) -> vkuint {
        self.backend().estimate_mip_levels()
    }

    fn build(&self, device: &GsDevice) -> VkResult<GsImage> {
        self.backend().build(device)
    }
}

impl<T> ImageCIApi for T
    where
        T: ImageCIInheritApi + ImageCISpecificApi {
    //...
}
