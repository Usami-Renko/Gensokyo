
use failure::{ Backtrace, Context, Fail };

use crate::assets::error::AssetsError;

use std::result;
use std::path::{ Path, PathBuf };
use std::fmt;

use gsvk::error::VkError;

pub type GsResult<T> = result::Result<T, GsError>;

// -------------------------------------------------------------------------------------------
#[derive(Debug)]
pub struct GsError {
    ctx: Context<GsErrorKind>,
}

impl GsError {

    pub fn kind(&self) -> &GsErrorKind {
        self.ctx.get_context()
    }

    pub fn config(config_name: impl AsRef<str>) -> GsError {
        GsError::from(GsErrorKind::Config { config_name: config_name.as_ref().to_string() })
    }

    pub fn assets(error: AssetsError) -> GsError {
        GsError::from(GsErrorKind::Assets(error))
    }

    pub(crate) fn window(description: &'static str) -> GsError {
        GsError::from(GsErrorKind::Window { description })
    }

    pub fn other(description: impl AsRef<str>) -> GsError {
        GsError::from(GsErrorKind::Other { description: description.as_ref().to_string() })
    }

    pub fn serialize(err: bincode::Error) -> GsError {
        GsError::from(GsErrorKind::Serialize(err))
    }
}

impl From<VkError> for GsError {

    fn from(error: VkError) -> GsError {
        GsError::from(GsErrorKind::Vk(error))
    }
}

impl Fail for GsError {

    fn cause(&self) -> Option<&Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for GsError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.ctx.fmt(f)
    }
}
// -------------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------------
#[derive(Debug, Fail)]
pub enum GsErrorKind {

    #[fail(display = "{}", _0)]
    Vk(#[cause] VkError),
    /// An error occurred while reading configuration.
    #[fail(display = "Failed to recognize {} config in manifest", config_name)]
    Config { config_name: String },
    /// An error occurred while loading assets(model, image...).
    #[fail(display = "Encounter error when loading assets: {}", _0)]
    Assets(#[cause] AssetsError),
    /// An error occurred while communicate with Window.
    #[fail(display = "Failed to interact with Window: {}.", description)]
    Window { description: &'static str },
    /// An error that occurred while working with a file path.
    #[fail(display = "Failed to locate file at: {:?}", path)]
    Path { path: PathBuf },
    /// An unexpected I/O error occurred.
    #[fail(display = "I/O Error")]
    Io,
    #[fail(display = "Error occurred when trying to serialize data: {}", _0)]
    Serialize(#[cause] bincode::Error),
    /// Other errors
    #[fail(display = "{}", description)]
    Other { description: String },
}

impl GsErrorKind {

    /// A convenience routine for creating an error associated with a path.
    pub(crate) fn path(path: impl AsRef<Path>)-> GsErrorKind {
        GsErrorKind::Path { path: path.as_ref().to_path_buf() }
    }
}

impl From<GsErrorKind> for GsError {

    fn from(kind: GsErrorKind) -> GsError {
        GsError::from(Context::new(kind))
    }
}

impl From<Context<GsErrorKind>> for GsError {

    fn from(ctx: Context<GsErrorKind>) -> GsError {
        GsError { ctx }
    }
}
// -------------------------------------------------------------------------------------------
