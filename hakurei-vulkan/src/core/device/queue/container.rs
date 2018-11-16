
use core::DeviceV1;

use core::device::DeviceConfig;
use core::device::queue::traits::HaQueueAbstract;
use core::device::queue::{ HaQueue, QueueUsage, HaGraphicsQueue, HaPresentQueue, HaTransferQueue };
use core::device::enums::DeviceQueueIdentifier;
use core::error::LogicalDeviceError;

use std::rc::Rc;

pub struct QueueContainer {

    graphics : Vec<HaGraphicsQueue>,
    presents : Vec<HaPresentQueue>,
    transfers: Vec<HaTransferQueue>,
}

impl QueueContainer {

    pub fn empty() -> QueueContainer {
        QueueContainer {
            graphics : vec![],
            presents : vec![],
            transfers: vec![],
        }
    }

    pub fn add_queue(&mut self, device: &DeviceV1, usage: QueueUsage, queue: &Rc<HaQueue>, config: &DeviceConfig) -> Result<(), LogicalDeviceError> {

        match usage {
            | QueueUsage::Graphics => {
                let graphics_queue = HaGraphicsQueue::new(device, queue, config)?;
                self.graphics.push(graphics_queue);
            },
            | QueueUsage::Present  => {
                let present_queue = HaPresentQueue::new(device, queue, config)?;
                self.presents.push(present_queue);
            },
            | QueueUsage::Transfer => {
                let transfer_queue = HaTransferQueue::new(device, queue, config)?;
                self.transfers.push(transfer_queue);
            },
        };

        Ok(())
    }

    #[allow(dead_code)]
    pub fn graphics_queue(&self, index: usize) -> &HaGraphicsQueue { &self.graphics[index] }
    #[allow(dead_code)]
    pub fn present_queue(&self, index: usize) -> &HaPresentQueue { &self.presents[index] }
    #[allow(dead_code)]
    pub fn transfer_queue(&self, index: usize) -> &HaTransferQueue { &self.transfers[index] }

    pub fn take_last_graphics_queue(&mut self) -> HaGraphicsQueue {
        self.graphics.pop().unwrap()
    }
    pub fn take_last_present_queue(&mut self) -> HaPresentQueue {
        self.presents.pop().unwrap()
    }
    pub fn take_last_transfer_queue(&mut self) -> HaTransferQueue {
        self.transfers.pop().unwrap()
    }

    pub fn queue(&self, ident: DeviceQueueIdentifier, index: usize) -> &Rc<HaQueue> {
        match ident {
            | DeviceQueueIdentifier::Graphics => &self.graphics[index].queue(),
            | DeviceQueueIdentifier::Present  => &self.presents[index].queue(),
            | DeviceQueueIdentifier::Transfer => &self.transfers[index].queue(),
            | _ => panic!("Invaild queue identifier.")
        }
    }
}
