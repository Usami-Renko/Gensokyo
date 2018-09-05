
pub(crate) mod core;
pub(crate) mod window;
pub(crate) mod swapchain;
pub(crate) mod sync;
pub(crate) mod image;

/// Set this true to enable verbose log information.
#[cfg(not(feature = "verbose"))]
pub(crate) const VERBOSE: bool = false;
#[cfg(feature = "verbose")]
pub(crate) const VERBOSE: bool = true;
