
pub(crate) mod core;
pub(crate) mod window;
pub(crate) mod swapchain;
pub(crate) mod sync;

/// Set this true to enable verbose log information.
#[cfg(feature = "verbose")]
pub(crate) const VERBOSE: bool = true;
#[cfg(not(feature = "verbose"))]
pub(crate) const VERBOSE: bool = false;
