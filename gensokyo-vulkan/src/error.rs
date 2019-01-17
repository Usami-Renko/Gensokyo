
use failure::{ Backtrace, Context, Fail };

use std::result;
use std::path::{ Path, PathBuf };
use std::fmt;

pub type VkResult<T> = result::Result<T, VkError>;

// -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct VkError {

    ctx: Context<VkErrorKind>,
}

impl VkError {

    pub fn kind(&self) -> &VkErrorKind {
        self.ctx.get_context()
    }

    pub(crate) fn unlink(dst_obj: impl AsRef<str>) -> VkError {
        VkError::from(VkErrorKind::Unlink(dst_obj.as_ref().to_string()))
    }

    pub(crate) fn query(property: impl AsRef<str>) -> VkError {
        VkError::from(VkErrorKind::Query(property.as_ref().to_string()))
    }

    pub(crate) fn create(obj: impl AsRef<str>) -> VkError {
        VkError::from(VkErrorKind::Create(obj.as_ref().to_string()))
    }

    pub(crate) fn unsupported(feature: impl AsRef<str>) -> VkError {
        VkError::from(VkErrorKind::UnSupport(feature.as_ref().to_string()))
    }

    pub(crate) fn sync(desc: impl AsRef<str>) -> VkError {
        VkError::from(VkErrorKind::Sync(desc.as_ref().to_string()))
    }

    pub(crate) fn device(desc: impl AsRef<str>) -> VkError {
        VkError::from(VkErrorKind::Device(desc.as_ref().to_string()))
    }

    pub(crate) fn shaderc(desc: impl AsRef<str>) -> VkError {
        VkError::from(VkErrorKind::Shaderc(desc.as_ref().to_string()))
    }

    pub(crate) fn str_convert(convert_name: impl AsRef<str>) -> VkError {
        VkError::from(VkErrorKind::StrConvert(convert_name.as_ref().to_string()))
    }

    pub(crate) fn window(desc: impl AsRef<str>) -> VkError {
        VkError::from(VkErrorKind::Window(desc.as_ref().to_string()))
    }

    pub(crate) fn other(desc: impl AsRef<str>) -> VkError {
        VkError::from(VkErrorKind::Other(desc.as_ref().to_string()))
    }
}

impl Fail for VkError {

    fn cause(&self) -> Option<&Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for VkError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}
// -------------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------------
/// The specific kind of error that can occur.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VkErrorKind {

    /// An error occurred while building connection between application and vulkan.
    Unlink(String),
    /// An error occurred while querying some properties from Vulkan.
    Query(String),
    /// An error occurred while creating Vulkan Object.
    Create(String),
    /// An error indicated requiring some unsupported feature.
    UnSupport(String),
    /// An error about synchronous or asynchronous operations.
    Sync(String),
    /// An error triggered by Invalid Device operations.
    Device(String),
    /// An error that occurred while trying to compile shader code in runtime.
    Shaderc(String),
    /// An error happened when trying to convert between CStr and String.
    StrConvert(String),
    /// An error occurred while communicate with Window.
    Window(String),
    /// An error that occurred while working with a file path.
    Path(PathBuf),
    /// Other errors
    Other(String),
}

impl fmt::Display for VkErrorKind {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            | VkErrorKind::Unlink(e)    => write!(f, "Failed to bridge connection between {} and Vulkan.", e),
            | VkErrorKind::Query(e)     => write!(f, "Failed to query {} property from Vulkan or Device.", e),
            | VkErrorKind::Create(e)    => write!(f, "Failed to create {}.", e),
            | VkErrorKind::UnSupport(e) => write!(f, "Feature {} is not supported in current Vulkan Device.", e),
            | VkErrorKind::Sync(e)      => write!(f, "{}.", e),
            | VkErrorKind::Device(e)    => write!(f, "Invalid Operation: {}", e),
            | VkErrorKind::Shaderc(e)   => write!(f, "Error occurred during runtime shader compiling: {}.", e),
            | VkErrorKind::StrConvert(e)=> write!(f, "Failed to convert string {} between c-style string and Rust string.", e),
            | VkErrorKind::Window(e)    => writeln!(f, "Failed to interact with Window: {}.", e),
            | VkErrorKind::Path(path)   => write!(f, "Failed to locate file at: {:?}", path),
            | VkErrorKind::Other(e)     => write!(f, "{}", e),
        }
    }
}

impl VkErrorKind {

    /// A convenience routine for creating an error associated with a path.
    pub(crate) fn path(path: impl AsRef<Path>)-> VkErrorKind {
        VkErrorKind::Path(path.as_ref().to_path_buf())
    }
}

impl From<VkErrorKind> for VkError {

    fn from(kind: VkErrorKind) -> VkError {
        VkError::from(Context::new(kind))
    }
}

impl From<Context<VkErrorKind>> for VkError {

    fn from(ctx: Context<VkErrorKind>) -> VkError {
        VkError { ctx }
    }
}
// -------------------------------------------------------------------------------------------
