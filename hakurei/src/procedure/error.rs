
use winit;

use core::error::{ InstanceError, ValidationError, PhysicalDeviceError, SurfaceError, LogicalDeviceError };
use core::swapchain::error::SwapchainError;
use pipeline::error::PipelineError;
use resources::error::CommandError;
use resources::error::AllocatorError;
use resources::error::DescriptorError;
use sync::error::SyncError;


use std::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum RuntimeError {

    Window(winit::CreationError),
    Procedure(ProcedureError),
}

impl Error for RuntimeError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            | RuntimeError::Window(ref e)    => Some(e),
            | RuntimeError::Procedure(ref e) => Some(e),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            | RuntimeError::Window(ref e)    => e.to_string(),
            | RuntimeError::Procedure(ref e) => e.to_string(),
        };
        write!(f, "{}", description)
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ProcedureError {

    Instance(InstanceError),
    Validation(ValidationError),
    Surface(SurfaceError),
    PhysicalDevice(PhysicalDeviceError),
    LogicalDevice(LogicalDeviceError),
    Swapchain(SwapchainError),
    Pipeline(PipelineError),
    Command(CommandError),
    Sync(SyncError),
    Descriptor(DescriptorError),
    Allocator(AllocatorError),

    SwapchainRecreate,
}

impl Error for ProcedureError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            | ProcedureError::Instance(ref e)       => Some(e),
            | ProcedureError::Validation(ref e)     => Some(e),
            | ProcedureError::Surface(ref e)        => Some(e),
            | ProcedureError::PhysicalDevice(ref e) => Some(e),
            | ProcedureError::LogicalDevice(ref e)  => Some(e),
            | ProcedureError::Swapchain(ref e)      => Some(e),
            | ProcedureError::Pipeline(ref e)       => Some(e),
            | ProcedureError::Command(ref e)        => Some(e),
            | ProcedureError::Sync(ref e)           => Some(e),
            | ProcedureError::Descriptor(ref e)     => Some(e),
            | ProcedureError::Allocator(ref e)      => Some(e),

            | ProcedureError::SwapchainRecreate     => None,
        }
    }
}

impl_from_err!(Procedure(ProcedureError)           -> RuntimeError);
impl_from_err!(Instance(InstanceError)             -> ProcedureError);
impl_from_err!(Validation(ValidationError)         -> ProcedureError);
impl_from_err!(Surface(SurfaceError)               -> ProcedureError);
impl_from_err!(PhysicalDevice(PhysicalDeviceError) -> ProcedureError);
impl_from_err!(LogicalDevice(LogicalDeviceError)   -> ProcedureError);
impl_from_err!(Swapchain(SwapchainError)           -> ProcedureError);
impl_from_err!(Pipeline(PipelineError)             -> ProcedureError);
impl_from_err!(Command(CommandError)               -> ProcedureError);
impl_from_err!(Sync(SyncError)                     -> ProcedureError);
impl_from_err!(Descriptor(DescriptorError)         -> ProcedureError);
impl_from_err!(Allocator(AllocatorError)           -> ProcedureError);

impl fmt::Display for ProcedureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = if let Some(err) = self.cause() {
            err.to_string()
        } else {
            "Unknown Error".to_owned()
        };

        write!(f, "{}", description)
    }
}

