
use vk::core::device::HaDevice;
use vk::core::physical::HaPhyDevice;

use vk::resources::memory::MemorySelector;
use vk::resources::transfer::DataCopyer;
use vk::resources::image::{ HaImage, ImageStorageType };
use vk::resources::error::{ ImageError, AllocatorError };

use resources::image::sample::{ SampleImageInfo, SampleImageBarrierBundle };
use resources::image::depth::{ DepthStencilAttachmentInfo, DepSteImageBarrierBundle };

use resources::image::ImageBarrierBundleAbs;
use resources::image::{ ImageBranchType, ImageBranchInfoAbs };
use resources::allocator::image::distributor::HaImageDistributor;
use resources::allocator::image::device::DeviceImgMemAllocator;
use resources::allocator::image::cached::CachedImgMemAllocator;
use resources::allocator::image::traits::ImgMemAlloAbstract;
use resources::allocator::image::enums::ImageAllocateInfo;

use resources::image::io::ImageLoadConfig;

use std::collections::HashMap;

// TODO: Currently not support multi imageview for an image.

pub struct HaImageAllocator {

    physical: HaPhyDevice,
    device  : HaDevice,

    image_config: ImageLoadConfig,
    image_infos : Vec<ImageAllocateInfo>,

    storage_type: ImageStorageType,
    allocator   : Box<ImgMemAlloAbstract>,
    memory_selector : MemorySelector,
}

impl HaImageAllocator {

    pub(crate) fn new(physical: &HaPhyDevice, device: &HaDevice, typ: ImageStorageType, image_config: ImageLoadConfig)
        -> HaImageAllocator {

        HaImageAllocator {

            physical: physical.clone(),
            device  : device.clone(),

            image_config,
            image_infos: vec![],

            storage_type: typ,
            allocator   : gen_allocator(typ),
            memory_selector : MemorySelector::init(physical, typ.memory_type()),
        }
    }

    pub fn append_sample_image(&mut self, info: &mut SampleImageInfo) -> Result<(), AllocatorError> {

        self.append_image(info)
    }

    pub fn append_depth_stencil_image(&mut self, info: &mut DepthStencilAttachmentInfo) -> Result<(), AllocatorError> {

        self.append_image(info)
    }

    fn append_image(&mut self, info: &mut impl ImageBranchInfoAbs) -> Result<(), AllocatorError> {

        let storage = info.storage(&self.physical, &self.image_config)?;
        let image = HaImage::config(&self.device, info.view_desc(), storage.dimension, storage.format)?;
        self.memory_selector.try(&image)?;

        info.set_allocate_index(self.image_infos.len());
        self.image_infos.push(info.allocate_info(image, storage));

        Ok(())
    }

    pub fn allocate(mut self) -> Result<HaImageDistributor, AllocatorError> {

        if self.image_infos.is_empty() {
            return Err(AllocatorError::Image(ImageError::NoImageAppendError))
        }

        // 1.select memory type for image.
        let total_space = self.image_infos.iter()
            .fold(0, |sum, image_info| {
                sum + image_info.space
            });

        // 2.allocate memory.
        self.allocator.allocate(
            &self.device, total_space, &self.memory_selector
        )?;

        // 3.bind image to memory.
        {
            let memory = self.allocator.borrow_memory()?;

            let mut offset = 0;
            for image_info in self.image_infos.iter() {
                memory.bind_to_image(&self.device, &image_info.image, offset)?;
                offset += image_info.space;
            }
        }

        // 4.record image barrier transitions(copy data if needed).
        let mut copyer = DataCopyer::new(&self.device)?;

        let mut barrier_bundles = collect_barrier_bundle(&self.image_infos);
        for bundle in barrier_bundles.iter_mut() {
            bundle.make_transfermation(&self.physical, &self.device, &copyer, &mut self.image_infos)?;
        }

        // 5.execute image barrier transition.
        copyer.done()?;

        // 6.do some cleaning.
        barrier_bundles.iter_mut()
            .for_each(|bundle| bundle.cleanup());

        // final done.
        HaImageDistributor::new(self.device, self.image_infos, self.allocator.take_memory()?)
    }

    pub fn reset(&mut self) {

        self.image_infos.iter().for_each(|image_info| {
            image_info.cleanup(&self.device);
        });

        self.memory_selector.reset();
    }
}



fn collect_barrier_bundle(image_infos: &[ImageAllocateInfo]) -> Vec<Box<ImageBarrierBundleAbs>> {

    let mut barrier_indices: HashMap<ImageBranchType, Vec<usize>, _> = HashMap::new();

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
                | ImageBranchType::SampleImage(stage) => {
                    let bundle = Box::new(SampleImageBarrierBundle::new(stage, indices));
                    bundle as Box<ImageBarrierBundleAbs>
                },
                | ImageBranchType::DepthStencilAttachment => {
                    let bundle = Box::new(DepSteImageBarrierBundle::new(indices));
                    bundle as Box<ImageBarrierBundleAbs>
                },
                | ImageBranchType::DepthStencilImage(_, _) => {
                    unimplemented!()
                }
            }

        }).collect();

    bundles
}

fn gen_allocator(typ: ImageStorageType) -> Box<ImgMemAlloAbstract> {
    match typ {
        | ImageStorageType::Device => Box::new(DeviceImgMemAllocator::new()),
        | ImageStorageType::Cached => Box::new(CachedImgMemAllocator::new()),
    }
}