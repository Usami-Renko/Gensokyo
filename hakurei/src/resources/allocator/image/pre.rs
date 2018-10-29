
use ash::vk;

use config::resources::ImageLoadConfig;
use core::device::HaDevice;
use core::physical::{ HaPhyDevice, MemorySelector };

use resources::repository::DataCopyer;
use resources::image::HaImage;
use resources::image::{ SampleImageInfo, DepthStencilImageInfo };
use resources::image::{ ImageBarrierBundleAbs, SampleImageBarrierBundle, DepSteImageBarrierBundle };
use resources::image::{ ImageBranchType, ImageBranchInfoAbs };
use resources::allocator::HaImageDistributor;
use resources::allocator::{ ImageAllocateInfo, ImageStorageType, ImgMemAlloAbstract };
use resources::error::{ ImageError, AllocatorError };

use std::collections::hash_map::{ HashMap, RandomState };

// TODO: Currently not support multi imageview for an image.

pub struct HaImagePreAllocator {

    physical: HaPhyDevice,
    device  : HaDevice,

    image_config: ImageLoadConfig,
    image_infos: Vec<ImageAllocateInfo>,

    storage_type: ImageStorageType,
    allocator: Box<ImgMemAlloAbstract>,
    require_mem_flag: vk::MemoryPropertyFlags,
    memory_selector : MemorySelector,
}

impl HaImagePreAllocator {

    pub(crate) fn new(physical: &HaPhyDevice, device: &HaDevice, typ: ImageStorageType, image_config: ImageLoadConfig)
        -> HaImagePreAllocator {

        HaImagePreAllocator {

            physical: physical.clone(),
            device  : device.clone(),

            image_config,
            image_infos: vec![],

            storage_type: typ,
            allocator: typ.allocator(),
            require_mem_flag: typ.memory_type().property_flags(),
            memory_selector : MemorySelector::init(physical),
        }
    }

    pub fn append_sample_image(&mut self, info: &mut SampleImageInfo) -> Result<(), AllocatorError> {

        self.append_image(info)
    }

    pub fn append_depth_stencil_image(&mut self, info: &mut DepthStencilImageInfo) -> Result<(), AllocatorError> {

        self.append_image(info)
    }

    fn append_image(&mut self, info: &mut impl ImageBranchInfoAbs) -> Result<(), AllocatorError> {

        let storage = info.storage(&self.physical, &self.image_config)?;
        let image = HaImage::config(&self.device, info.view_desc(), storage.dimension, storage.format)?;
        self.memory_selector.try(image.requirement.memory_type_bits, self.require_mem_flag)?;

        info.set_allocate_index(self.image_infos.len());
        self.image_infos.push(info.allocate_info(image, storage));

        Ok(())
    }

    pub fn allocate(mut self) -> Result<HaImageDistributor, AllocatorError> {

        if self.image_infos.is_empty() {
            return Err(AllocatorError::Image(ImageError::NoImageAttachError))
        }

        // 1.create image buffer.
        let optimal_memory_index = self.memory_selector.optimal_memory()?;
        let mem_type = self.physical.memory.memory_type(optimal_memory_index);
        let total_space = self.image_infos.iter()
            .fold(0, |sum, image_info| {
                sum + image_info.space
            });

        // 2.allocate memory.
        self.allocator.allocate(
            &self.device, total_space, optimal_memory_index, Some(mem_type)
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

        // 4.prepare copy data to image.
        let mut copyer = DataCopyer::new(&self.device)?;

        // 5. record image barrier transitions.
        let mut barrier_bundles = collect_barrier_bundle(&self.physical, &self.device, &self.image_infos);
        for bundle in barrier_bundles.iter_mut() {
            bundle.make_transfermation(&copyer, &mut self.image_infos)?;
        }

        // 6.execute image barrier transition.
        copyer.done()?;

        // 7.do some cleaning.
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
        self.require_mem_flag = self.storage_type.memory_type().property_flags();
    }
}



fn collect_barrier_bundle(physical: &HaPhyDevice, device: &HaDevice, image_infos: &[ImageAllocateInfo]) -> Vec<Box<ImageBarrierBundleAbs>> {

    let mut barrier_indices: HashMap<ImageBranchType, Vec<usize>, RandomState> = HashMap::new();

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
                    let bundle = Box::new(SampleImageBarrierBundle::new(
                        physical, device, stage.clone(), indices));
                    bundle as Box<ImageBarrierBundleAbs>
                },
                | ImageBranchType::DepthStencilImage(usage) => {
                    let bundle = Box::new(DepSteImageBarrierBundle::new(
                        usage.clone(), indices));
                    bundle as Box<ImageBarrierBundleAbs>
                },
            }

        }).collect::<Vec<_>>();

    bundles
}
