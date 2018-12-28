
use ash::vk;

use crate::core::device::GsDevice;
use crate::core::physical::GsPhyDevice;

use crate::image::target::{ GsImage, ImageDescInfo };
use crate::image::view::ImageViewDescInfo;
use crate::image::enums::ImageInstanceType;
use crate::image::traits::ImageCopiable;
use crate::image::utils::ImageCopyInfo;
use crate::image::storage::ImageStorageInfo;
use crate::image::instance::{ ImageBarrierBundleAbs, ImageInstanceInfoAbs, ImageInstanceInfoDesc };
use crate::image::instance::sample::{ SampleImageInfo, SampleImageBarrierBundle };
use crate::image::instance::depth::{ DepthStencilAttachmentInfo, DepSteImageBarrierBundle };
use crate::image::allocator::types::ImageMemoryTypeAbs;
use crate::image::allocator::distributor::GsImageDistributor;
use crate::image::error::ImageError;

use crate::memory::{ MemoryFilter, MemoryDstEntity };
use crate::memory::transfer::DataCopyer;
use crate::memory::AllocatorError;

use crate::types::vkbytes;

use std::collections::HashMap;
use std::marker::PhantomData;

// TODO: Currently not support multi imageview for an image.

pub struct GsImageAllocator<M> where M: ImageMemoryTypeAbs {

    phantom_type: PhantomData<M>,
    storage_type: M,

    physical: GsPhyDevice,
    device  : GsDevice,

    image_infos : Vec<ImageAllocateInfo>,

    memory_filter : MemoryFilter,
}

impl<M> GsImageAllocator<M> where M: ImageMemoryTypeAbs {

    pub fn new(physical: &GsPhyDevice, device: &GsDevice, storage_type: M) -> GsImageAllocator<M> {

        GsImageAllocator {
            phantom_type: PhantomData,
            storage_type,

            physical: physical.clone(),
            device  : device.clone(),

            image_infos: vec![],

            memory_filter: MemoryFilter::new(physical, storage_type.memory_type()),
        }
    }

    pub fn append_sample_image(&mut self, info: &mut SampleImageInfo) -> Result<(), AllocatorError> {

        let storage = info.take_storage()
            .ok_or(AllocatorError::DuplicateAppendImage)?;
        self.append_image(info, storage)
    }

    pub fn append_depth_stencil_image(&mut self, info: &mut DepthStencilAttachmentInfo) -> Result<(), AllocatorError> {

        let storage = info.gen_storage_info();
        self.append_image(info, storage)
    }

    fn append_image(&mut self, info: &mut impl ImageInstanceInfoAbs, storage: ImageStorageInfo) -> Result<(), AllocatorError> {

        let image = info.build_image(&self.device)?;
        self.memory_filter.filter(&image)?;

        info.set_allocate_index(self.image_infos.len());
        self.image_infos.push(info.allocate_info(image, storage));

        Ok(())
    }

    pub fn allocate(mut self) -> Result<GsImageDistributor<M>, AllocatorError> {

        if self.image_infos.is_empty() {
            return Err(AllocatorError::Image(ImageError::NoImageAppendError))
        }

        // 1.select memory type for image.
        let total_space = self.image_infos.iter()
            .fold(0, |sum, image_info| {
                sum + image_info.space
            });

        // 2.allocate memory.
        let memory = self.storage_type.allot_memory(&self.device, total_space, &self.memory_filter)?;

        // 3.bind image to memory.
        let mut offset = 0;
        for image_info in self.image_infos.iter() {
            memory.bind_to_image(&self.device, &image_info.image, offset)?;
            offset += image_info.space;
        }

        // 4.record image barrier transitions(copy data if needed).
        let mut copyer = DataCopyer::new(&self.device)?;

        let mut barrier_bundles = collect_barrier_bundle(&self.image_infos);
        for bundle in barrier_bundles.iter_mut() {
            bundle.make_transfermation(&self.physical, &self.device, &copyer, &mut self.image_infos)?;
        }

        // 5.execute image barrier transition.
        copyer.done()?;

        // final done.
        GsImageDistributor::new(self.phantom_type, self.device, self.image_infos, memory)
    }

    pub fn reset(&mut self) {

        self.image_infos.iter().for_each(|image_info| {
            image_info.destroy(&self.device);
        });

        self.memory_filter.reset();
    }
}


pub struct ImageAllocateInfo {

    pub typ: ImageInstanceType,

    pub image: GsImage,
    pub image_desc: ImageDescInfo,
    pub view_desc : ImageViewDescInfo,

    pub storage: ImageStorageInfo,
    pub space  : vkbytes,

    pub final_layout: vk::ImageLayout,
}

impl ImageAllocateInfo {

    pub fn new(typ: ImageInstanceType, storage: ImageStorageInfo, image: GsImage, image_desc: ImageDescInfo, view_desc: ImageViewDescInfo) -> ImageAllocateInfo {

        let space = image.aligment_size();

        ImageAllocateInfo {
            typ, image, image_desc, view_desc, storage, space,
            final_layout: vk::ImageLayout::UNDEFINED,
        }
    }

    pub fn gen_desc(&self) -> ImageInstanceInfoDesc {

        ImageInstanceInfoDesc {
            current_layout : self.final_layout,
            dimension      : self.storage.dimension,
            subrange: self.view_desc.subrange.clone(),
        }
    }

    pub fn destroy(&self, device: &GsDevice) {

        self.image.destroy(device);
    }
}

impl ImageCopiable for ImageAllocateInfo {

    fn copy_info(&self) -> ImageCopyInfo {

        use crate::image::utils::image_subrange_to_layers;
        // The layout paramater is the destination layout after data copy.
        // This value should be vk::TransferDstOptimal.
        let subrange_layers = image_subrange_to_layers(&self.view_desc.subrange);
        ImageCopyInfo::new(&self.image, subrange_layers, self.final_layout, self.storage.dimension)
    }
}

fn collect_barrier_bundle(image_infos: &[ImageAllocateInfo]) -> Vec<Box<dyn ImageBarrierBundleAbs>> {

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
                    let bundle = Box::new(DepSteImageBarrierBundle::new(indices));
                    bundle as Box<dyn ImageBarrierBundleAbs>
                },
                | ImageInstanceType::DepthStencilImage { format: _, stage: _ } => {
                    unimplemented!()
                },
            }

        }).collect();

    bundles
}
