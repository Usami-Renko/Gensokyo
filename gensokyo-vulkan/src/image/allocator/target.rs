
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::{ GsImage, ImageTgtCI };
use crate::image::view::ImageViewCI;
use crate::image::enums::ImageInstanceType;
use crate::image::traits::ImageCopiable;
use crate::image::utils::ImageCopyInfo;
use crate::image::storage::ImageStorageInfo;
use crate::image::instance::traits::{ ImageCIAbstract, ImageBarrierBundleAbs };
use crate::image::instance::desc::ImageInstanceInfoDesc;
use crate::image::instance::sample::SampleImageBarrierBundle;
use crate::image::instance::depth::DSImageBarrierBundle;
use crate::image::allocator::types::ImageMemoryTypeAbs;
use crate::image::allocator::distributor::GsImageDistributor;

use crate::memory::{ MemoryFilter, MemoryDstEntity };
use crate::memory::transfer::DataCopyer;

use crate::error::{ VkResult, VkError };
use crate::utils::allot::{ GsAssignIndex, GsAllocatorApi, GsAllotIntoDistributor };
use crate::types::vkbytes;

use std::collections::HashMap;
use std::marker::PhantomData;

// TODO: Currently not support multi imageview for an image.

pub struct GsImageAllocator<M>
    where
        M: ImageMemoryTypeAbs {

    phantom_type: PhantomData<M>,
    storage_type: M,

    device  : GsDevice,

    image_infos: Vec<ImageAllotCI>,

    memory_filter: MemoryFilter,
}

impl<M, I, R> GsAllocatorApi<I, R, GsImageDistributor<M>> for GsImageAllocator<M>
    where
        I: ImageCIAbstract<R>,
        M: ImageMemoryTypeAbs {

    type AssignResult = VkResult<GsAssignIndex<R>>;

    fn assign(&mut self, ci: I) -> Self::AssignResult {

        let image = ci.build(&self.device)?;
        self.memory_filter.filter(&image)?;

        let (image_info, message) = ci.refactor(&self.device, image)?;
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

    fn allocate(self) -> VkResult<GsImageDistributor<M>> {

        let mut allocator = self; // make self mutable.

        if allocator.image_infos.is_empty() {
            return Err(VkError::other("There must be images appended to allocator before allocate memory."))
        }

        // 1.select memory type for image.
        let total_space = allocator.image_infos.iter()
            .fold(0, |sum, image_info| {
                sum + image_info.space
            });

        // 2.allocate memory.
        let memory = allocator.storage_type.allot_memory(&allocator.device, total_space, &allocator.memory_filter)?;

        // 3.bind image to memory.
        let mut offset = 0;
        for image_info in allocator.image_infos.iter() {
            memory.bind_to_image(&allocator.device, &image_info.image, offset)?;
            offset += image_info.space;
        }

        // 4.record image barrier transitions(copy data if needed).
        let mut copyer = DataCopyer::new(&allocator.device)?;

        let mut barrier_bundles = collect_barrier_bundle(&allocator.image_infos);
        for bundle in barrier_bundles.iter_mut() {
            bundle.make_barrier_transform(&allocator.device, &copyer, &mut allocator.image_infos)?;
        }

        // 5.execute image barrier transition.
        copyer.done()?;

        // final done.
        GsImageDistributor::new(allocator.phantom_type, allocator.device, allocator.image_infos, memory)
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

            image_infos: vec![],

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

    pub final_layout: vk::ImageLayout,
}

impl ImageAllotCI {

    pub fn new(typ: ImageInstanceType, storage: ImageStorageInfo, image: GsImage, image_ci: ImageTgtCI, view_ci: ImageViewCI) -> ImageAllotCI {

        let space = image.alignment_size();

        ImageAllotCI {
            typ, image, image_ci, view_ci, storage, space,
            final_layout: vk::ImageLayout::UNDEFINED,
        }
    }

    pub fn gen_desc(&self) -> ImageInstanceInfoDesc {

        ImageInstanceInfoDesc {
            current_layout : self.final_layout,
            dimension      : self.storage.dimension,
            subrange: self.view_ci.subrange.clone(),
        }
    }

    pub fn destroy(&self, device: &GsDevice) {

        self.image.destroy(device);
    }
}

impl ImageCopiable for ImageAllotCI {

    fn copy_info(&self) -> ImageCopyInfo {

        use crate::image::utils::image_subrange_to_layers;
        // The layout parameter is the destination layout after data copy.
        // This value should be vk::TransferDstOptimal.
        let subrange_layers = image_subrange_to_layers(&self.view_ci.subrange);
        ImageCopyInfo::new(&self.image, subrange_layers, self.final_layout, self.storage.dimension)
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
                    let bundle = Box::new(SampleImageBarrierBundle::new(stage, indices));
                    bundle as Box<dyn ImageBarrierBundleAbs>
                },
                | ImageInstanceType::DepthStencilAttachment => {
                    let bundle = Box::new(DSImageBarrierBundle::new(indices));
                    bundle as Box<dyn ImageBarrierBundleAbs>
                },
                | ImageInstanceType::DepthStencilImage { format: _, stage: _ } => {
                    unimplemented!()
                },
            }

        }).collect();

    bundles
}
