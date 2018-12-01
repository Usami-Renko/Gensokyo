
use core::device::DeviceConfig;
use core::device::queue::traits::GsQueueAbstract;
use core::device::queue::{ GsQueue, QueueUsage, GsGraphicsQueue, GsPresentQueue, GsTransferQueue };
use core::device::enums::DeviceQueueIdentifier;
use core::error::LogicalDeviceError;

use std::rc::Rc;

pub struct QueueContainer {

    graphics : Vec<GsGraphicsQueue>,
    presents : Vec<GsPresentQueue>,
    transfers: Vec<GsTransferQueue>,
}

impl QueueContainer {

    pub fn empty() -> QueueContainer {
        QueueContainer {
            graphics : vec![],
            presents : vec![],
            transfers: vec![],
        }
    }

    pub fn add_queue(&mut self, device: &ash::Device, usage: QueueUsage, queue: &Rc<GsQueue>, config: &DeviceConfig) -> Result<(), LogicalDeviceError> {

        match usage {
            | QueueUsage::Graphics => {
                let graphics_queue = GsGraphicsQueue::new(device, queue, config)?;
                self.graphics.push(graphics_queue);
            },
            | QueueUsage::Present  => {
                let present_queue = GsPresentQueue::new(device, queue, config)?;
                self.presents.push(present_queue);
            },
            | QueueUsage::Transfer => {
                let transfer_queue = GsTransferQueue::new(device, queue, config)?;
                self.transfers.push(transfer_queue);
            },
        };

        Ok(())
    }

    #[allow(dead_code)]
    pub fn graphics_queue(&self, index: usize) -> &GsGraphicsQueue { &self.graphics[index] }
    #[allow(dead_code)]
    pub fn present_queue(&self, index: usize) -> &GsPresentQueue { &self.presents[index] }
    #[allow(dead_code)]
    pub fn transfer_queue(&self, index: usize) -> &GsTransferQueue { &self.transfers[index] }

    pub fn take_last_graphics_queue(&mut self) -> GsGraphicsQueue {
        self.graphics.pop().unwrap()
    }
    pub fn take_last_present_queue(&mut self) -> GsPresentQueue {
        self.presents.pop().unwrap()
    }
    pub fn take_last_transfer_queue(&mut self) -> GsTransferQueue {
        self.transfers.pop().unwrap()
    }

    pub fn queue(&self, ident: DeviceQueueIdentifier, index: usize) -> &Rc<GsQueue> {
        match ident {
            | DeviceQueueIdentifier::Graphics => &self.graphics[index].queue(),
            | DeviceQueueIdentifier::Present  => &self.presents[index].queue(),
            | DeviceQueueIdentifier::Transfer => &self.transfers[index].queue(),
            | _ => panic!("Invaild queue identifier.")
        }
    }
}
