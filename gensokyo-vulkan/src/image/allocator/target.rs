
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::{ GsImage, ImageTgtCI };
use crate::image::view::ImageViewCI;
use crate::image::enums::ImageInstanceType;
use crate::image::traits::ImageCopiable;
use crate::image::sampler::GsSampler;
use crate::image::utils::{ ImageCopyInfo, ImageCopySubrange };
use crate::image::storage::ImageStorageInfo;
use crate::image::instance::traits::{ ImageCIAbstract, ImageBarrierBundleAbs };
use crate::image::instance::desc::ImageInstanceInfoDesc;
use crate::image::instance::sample::SampleImageBarrierBundle;
use crate::image::instance::depth::DSImageBarrierBundle;
use crate::image::instance::traits::IImageConveyor;
use crate::image::allocator::types::ImageMemoryTypeAbs;
use crate::image::allocator::distributor::GsImageDistributor;

use crate::memory::{ MemoryFilter, MemoryDstEntity };
use crate::memory::transfer::DataCopyer;

use crate::error::{ VkResult, VkError };
use crate::utils::allot::{ GsAssignIndex, GsAllocatorApi, GsAllotIntoDistributor };
use crate::types::vkbytes;

use std::collections::{ HashMap, HashSet };
use std::marker::PhantomData;

// TODO: Currently not support multi imageview for an image.

pub struct GsImageAllocator<M>
    where
        M: ImageMemoryTypeAbs {

    phantom_type: PhantomData<M>,
    storage_type: M,

    device  : GsDevice,

    image_infos: Vec<ImageAllotCI>,
    samplers: HashSet<GsSampler>,

    memory_filter: MemoryFilter,
}

impl<M, I, R> GsAllocatorApi<I, R, GsImageDistributor<M>> for GsImageAllocator<M>
    where
        I: ImageCIAbstract<R>,
        M: ImageMemoryTypeAbs,
        R: IImageConveyor {

    type AssignResult = VkResult<GsAssignIndex<R>>;

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

        if let Some(sampler) = message.sampler() {
            self.samplers.insert(sampler);
        }

        let dst_index = GsAssignIndex {
            convey_info: message,
            assign_index: self.image_infos.len(),
        };

        self.image_infos.push(image_info);

        Ok(dst_index)
    }

    fn reset(&mut self) {

        self.image_infos.iter().for_each(|ci| ci.destroy(&self.device));
        self.image_infos.clear();
        self.memory_filter.reset();
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

    pub typ: ImageInstanceType,

    pub image: GsImage,
    pub image_ci: ImageTgtCI,
    pub view_ci : ImageViewCI,

    pub storage: ImageStorageInfo,
    pub space  : vkbytes,

    // the layout of the image base mip-level that the program operate on.
    pub current_layout: vk::ImageLayout,
    // the current access flags for the image.
    pub current_access: vk::AccessFlags,
}

impl ImageAllotCI {

    pub fn new(typ: ImageInstanceType, storage: ImageStorageInfo, image: GsImage, image_ci: ImageTgtCI, view_ci: ImageViewCI) -> ImageAllotCI {

        let space = image.alignment_size();
        let current_layout = image_ci.property.initial_layout;
        let current_access = vk::AccessFlags::empty();

        ImageAllotCI { typ, image, image_ci, view_ci, storage, space, current_layout, current_access }
    }

    pub fn gen_desc(&self) -> ImageInstanceInfoDesc {

        ImageInstanceInfoDesc {
            current_layout : self.current_layout,
            dimension      : self.storage.dimension,
            subrange: self.view_ci.subrange.clone(),
        }
    }

    pub fn destroy(&self, device: &GsDevice) {

        self.image.destroy(device);
    }
}

impl ImageCopiable for ImageAllotCI {

    fn copy_range(&self, subrange: ImageCopySubrange) -> ImageCopyInfo {

        // The layout parameter is the destination layout after data copy.
        // This value should be vk::TransferDstOptimal.
        ImageCopyInfo {
            handle: self.image.handle,
            layout: self.current_layout,
            extent: self.storage.dimension,
            sub_resource_layers: subrange,
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
                | ImageInstanceType::SampleImage { stage } => {
                    let bundle = SampleImageBarrierBundle::new(stage, indices);
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
