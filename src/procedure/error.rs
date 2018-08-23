
use winit;

use core::error::{ InstanceError, ValidationError, PhysicalDeviceError, SurfaceError, LogicalDeviceError };
use swapchain::SwapchainInitError;
use pipeline::error::PipelineError;

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
    SwapchainCreation(SwapchainInitError),
    Pipeline(PipelineError),
}

impl Error for ProcedureError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            | ProcedureError::Instance(ref e)          => Some(e),
            | ProcedureError::Validation(ref e)        => Some(e),
            | ProcedureError::Surface(ref e)           => Some(e),
            | ProcedureError::PhysicalDevice(ref e)    => Some(e),
            | ProcedureError::LogicalDevice(ref e)     => Some(e),
            | ProcedureError::SwapchainCreation(ref e) => Some(e),
            | ProcedureError::Pipeline(ref e)          => Some(e),
        }
    }
}

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
