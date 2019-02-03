
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::GsImage;
use crate::image::enums::ImageInstanceType;
use crate::image::traits::ImageCopiable;
use crate::image::copy::ImageFullCopyInfo;

use crate::image::instance::base::{ GsBackendImage, SampleImageBarrierBundle };
use crate::image::instance::traits::{ ImageCIApi, ImageCISpecificApi, ImageBarrierBundleAbs };
use crate::image::instance::traits::{ IImageConveyor, ImageInstanceInfoDesc };
use crate::image::instance::sampler::{ GsSampler, SamplerCI };
use crate::image::instance::sampler::{ GsSamplerArray, SamplerArrayCI };
use crate::image::instance::depth::DSImageBarrierBundle;
use crate::image::instance::sampler::GsSamplerMirror;
use crate::image::allocator::types::ImageMemoryTypeAbs;
use crate::image::allocator::distributor::GsImageDistributor;

use crate::memory::{ MemoryFilter, MemoryDstEntity };
use crate::memory::transfer::DataCopyer;

use crate::error::{ VkResult, VkError };
use crate::utils::allot::{ GsAssignIndex, GsAllocatorApi, GsAllotIntoDistributor };
use crate::types::{ vkuint, vkbytes, vkDim3D };

use std::collections::{ HashMap, HashSet };
use std::marker::PhantomData;

// TODO: Currently not support multi imageview for an image.

pub struct GsImageAllocator<M>
    where
        M: ImageMemoryTypeAbs {

    phantom_type: PhantomData<M>,
    storage_type: M,

    device: GsDevice,

    image_infos: Vec<ImageAllotCI>,
    samplers: HashSet<GsSamplerMirror>,

    memory_filter: MemoryFilter,
}

impl<M, I> GsAllocatorApi<I, GsImageDistributor<M>> for GsImageAllocator<M>
    where
        I: ImageCIApi + ImageCISpecificApi,
        M: ImageMemoryTypeAbs {

    type AssignResult = VkResult<GsAssignIndex<<I as ImageCISpecificApi>::IConveyor>>;

    fn assign(&mut self, ci: I) -> Self::AssignResult {

        // confirm if the physical device support image requirement.
        // TODO: Add option to disable check in release mode.
        ci.check_physical_support(&self.device)?;
        // generate vk::Image.
        let image = ci.build(&self.device)?;

        // check memory support.
        self.memory_filter.filter(&image)?;

        // `image_info` contains the message about memory allocation.
        // `message` contains the information used after allocation about the image itself.
        let (image_info, message) = ci.refactor(&self.device, image)?;

        if let Some(sampler) = message.sampler_mirror() {
            self.samplers.insert(sampler);
        }

        let dst_index = GsAssignIndex {
            convey_info: message,
            assign_index: self.image_infos.len(),
        };

        self.image_infos.push(image_info);

        Ok(dst_index)
    }
}

impl<M> GsAllocatorApi<SamplerCI, GsImageDistributor<M>> for GsImageAllocator<M>
    where
        M: ImageMemoryTypeAbs {

    type AssignResult = VkResult<GsSampler>;

    fn assign(&mut self, ci: SamplerCI) -> Self::AssignResult {

        let sampler = ci.build(&self.device)?;
        self.samplers.insert(sampler.mirror());

        Ok(sampler)
    }
}

impl<M> GsAllocatorApi<SamplerArrayCI, GsImageDistributor<M>> for GsImageAllocator<M>
    where
        M: ImageMemoryTypeAbs {

    type AssignResult = VkResult<GsSamplerArray>;

    fn assign(&mut self, ci: SamplerArrayCI) -> Self::AssignResult {

        let sampler_array = ci.build(&self.device)?;
        self.samplers.extend(sampler_array.mirrors());

        Ok(sampler_array)
    }
}

impl<M> GsAllotIntoDistributor<GsImageDistributor<M>> for GsImageAllocator<M>
    where
        M: ImageMemoryTypeAbs {

    fn allocate(mut self) -> VkResult<GsImageDistributor<M>> {

        // confirm there are images awaiting to be allocated.
        if self.image_infos.is_empty() {
            return Err(VkError::other("There must be images appended to allocator before allocate memory."))
        }

        // 1.select memory type for image.
        let total_space = self.image_infos.iter()
            .fold(0, |sum, image_info| {
                sum + image_info.space
            });

        // 2.allocate memory.
        let memory = self.storage_type
            .allot_memory(&self.device, total_space, &self.memory_filter)?;

        // 3.bind image to memory.
        let mut offset = 0;
        for image_info in self.image_infos.iter() {
            memory.bind_to_image(&self.device, &image_info.image, offset)?;
            offset += image_info.space;
        }

        // 4.record image barrier transitions(copy data, generate mipmap...etc, if needed).
        let mut copyer = DataCopyer::new(&self.device)?;

        let mut barrier_bundles = collect_barrier_bundle(&self.image_infos);
        for bundle in barrier_bundles.iter_mut() {
            bundle.make_barrier_transform(&self.device, &copyer, &mut self.image_infos)?;
        }

        // 5.execute image barrier transition.
        copyer.done()?;

        // final done.
        GsImageDistributor::new(self.phantom_type, self.device, self.image_infos, self.samplers, memory)
    }

    fn reset(&mut self) {

        self.image_infos.iter().for_each(|ci| ci.destroy(&self.device));
        self.image_infos.clear();
        self.memory_filter.reset();
    }
}

impl<M> GsImageAllocator<M>
    where
        M: ImageMemoryTypeAbs {

    pub fn create(device: &GsDevice, storage_type: M) -> GsImageAllocator<M> {

        GsImageAllocator {
            phantom_type: PhantomData,
            storage_type,

            device  : device.clone(),

            image_infos: Vec::new(),
            samplers   : HashSet::new(),

            memory_filter: MemoryFilter::new(device, storage_type.memory_type()),
        }
    }
}


pub struct ImageAllotCI {

    pub backend: GsBackendImage,

    pub typ: ImageInstanceType,

    pub image: GsImage,
    pub space  : vkbytes,

    // the layout of the image base mip-level that the program operate on.
    pub current_layout: vk::ImageLayout,
    // the current access flags for the image.
    pub current_access: vk::AccessFlags,
}

impl ImageAllotCI {

    pub fn new(typ: ImageInstanceType, image: GsImage, backend: GsBackendImage) -> ImageAllotCI {

        let space = image.alignment_size();
        let current_layout = backend.image_ci.property.initial_layout;
        let current_access = vk::AccessFlags::empty();

        ImageAllotCI { backend, typ, image, space, current_layout, current_access }
    }

    pub fn gen_desc(&self) -> ImageInstanceInfoDesc {

        ImageInstanceInfoDesc {
            current_layout : self.current_layout,
            dimension      : self.backend.storage.dimension,
            subrange       : self.backend.view_ci.subrange.clone(),
        }
    }

    pub fn destroy(&self, device: &GsDevice) {

        self.image.destroy(device);
    }
}

impl ImageCopiable for ImageAllotCI {

    fn full_copy_mipmap(&self, copy_mip_level: vkuint) -> ImageFullCopyInfo {

        debug_assert_eq!(self.current_layout, vk::ImageLayout::TRANSFER_DST_OPTIMAL);

        use std::cmp::max;

        ImageFullCopyInfo {
            handle: self.image.handle,
            // layout parameter is the destination layout after data copy.
            layout: self.current_layout,
            extent: vkDim3D {
                width : max(self.backend.storage.dimension.width  >> copy_mip_level, 1),
                height: max(self.backend.storage.dimension.height >> copy_mip_level, 1),
                // TODO: This parameter may be wrong setting.
                depth : self.backend.image_ci.property.array_layers,
            },
            sub_resource_layers: vk::ImageSubresourceLayers {
                aspect_mask      : self.backend.view_ci.subrange.0.aspect_mask,
                mip_level        : copy_mip_level,
                // copy all the layers.
                base_array_layer : 0,
                layer_count      : self.typ.layer_count(),
            },
        }
    }
}

fn collect_barrier_bundle(image_infos: &[ImageAllotCI]) -> Vec<Box<dyn ImageBarrierBundleAbs>> {

    let mut barrier_indices: HashMap<ImageInstanceType, Vec<usize>, _> = HashMap::new();

    for (index, image_info) in image_infos.iter().enumerate() {

        // make the logic a little strange to avoid borrow conflict.
        let is_found = {
            if let Some(indices) = barrier_indices.get_mut(&image_info.typ) {
                indices.push(index);
                true
            } else {
                false
            }
        };

        if is_found == false {
            barrier_indices.insert(image_info.typ.clone(), vec![index]);
        }
    };

    let bundles = barrier_indices.into_iter()
        .map(|(image_type, indices)| {

            match image_type {
                | ImageInstanceType::CombinedImageSampler { stage } => {
                    let bundle = SampleImageBarrierBundle::new(stage, image_type.clone(), indices);
                    Box::new(bundle) as Box<dyn ImageBarrierBundleAbs>
                },
                | ImageInstanceType::SampledImage { stage } => {
                    let bundle = SampleImageBarrierBundle::new(stage, image_type.clone(), indices);
                    Box::new(bundle) as Box<dyn ImageBarrierBundleAbs>
                },
                | ImageInstanceType::CubeMapImage { stage } => {
                    let bundle = SampleImageBarrierBundle::new(stage, image_type.clone(), indices);
                    Box::new(bundle) as Box<dyn ImageBarrierBundleAbs>
                },
                | ImageInstanceType::DepthStencilAttachment => {
                    let bundle = DSImageBarrierBundle::new(indices);
                    Box::new(bundle) as Box<dyn ImageBarrierBundleAbs>
                },
                | ImageInstanceType::DepthStencilImage { format: _, stage: _ } => {
                    unimplemented!()
                },
            }

        }).collect();

    bundles
}
