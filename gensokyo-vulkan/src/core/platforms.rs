
use winit;
use ash::vk;
use ash::version::{ EntryV1_0, InstanceV1_0 };
use std::ffi::CStr;

#[cfg(target_os = "macos")]
use ash::extensions::MacOSSurface;
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
use ash::extensions::XlibSurface;
#[cfg(target_os = "windows")]
use ash::extensions::Win32Surface;

#[cfg(target_os = "macos")]
use metal::CoreAnimationLayer;
#[cfg(target_os = "macos")]
use cocoa::base::id as cocoa_id;
#[cfg(target_os = "macos")]
use cocoa::appkit::{ NSView, NSWindow };
#[cfg(target_os = "macos")]
use objc::runtime::YES;

/// get the names of required extension used in macOS.
#[cfg(target_os = "macos")]
pub fn platform_surface_names() -> &'static CStr {
    MacOSSurface::name()
}

/// get the names of required extension used in Windows.
#[cfg(all(windows))]
pub fn platform_surface_names() -> &'static CStr {
    Win32Surface::name()
}

/// get the names of required extensions used in linux.
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub fn platform_surface_names() -> &'static CStr {
    XlibSurface::name()
}

/// get the required surface used in linux.
#[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
pub unsafe fn generate_surface<E: EntryV1_0, I: InstanceV1_0>(
    entry: &E,
    instance: &I,
    window: &winit::Window,
) -> Result<vk::SurfaceKHR, vk::Result> {

    use winit::os::unix::WindowExt;
    use std::ptr;

    let x11_display = window.get_xlib_display().unwrap();
    let x11_window = window.get_xlib_window().unwrap();
    let x11_create_info = vk::XlibSurfaceCreateInfoKHR {
        s_type : vk::StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
        p_next : ptr::null(),
        flags  : Default::default(),
        window : x11_window as vk::Window,
        dpy    : x11_display as *mut vk::Display,
    };
    let xlib_surface_loader =
        XlibSurface::new(entry, instance);
    xlib_surface_loader.create_xlib_surface_khr(&x11_create_info, None)
}

/// get the required surface used in macOS.
#[cfg(target_os = "macos")]
pub unsafe fn generate_surface<E: EntryV1_0, I: InstanceV1_0>(
    entry: &E,
    instance: &I,
    window: &winit::Window,
) -> Result<vk::SurfaceKHR, vk::Result> {

    use winit::os::macos::WindowExt;
    use std::os::raw::c_void;
    use std::mem;
    use std::ptr;

    let wnd: cocoa_id = mem::transmute(window.get_nswindow());

    let layer = CoreAnimationLayer::new();

    layer.set_edge_antialiasing_mask(0);
    layer.set_presents_with_transaction(false);
    layer.remove_all_animations();

    let view = wnd.contentView();

    layer.set_contents_scale(view.backingScaleFactor());
    view.setLayer(mem::transmute(layer.as_ref()));
    view.setWantsLayer(YES);

    let create_info = vk::MacOSSurfaceCreateInfoMVK {
        s_type : vk::StructureType::MACOS_SURFACE_CREATE_INFO_M,
        p_next : ptr::null(),
        flags  : Default::default(),
        p_view : window.get_nsview() as *const c_void
    };

    let macos_surface_loader =
        MacOSSurface::new(entry, instance);
    macos_surface_loader.create_mac_os_surface_mvk(&create_info, None)
}

/// get the required surface used in Windows.
#[cfg(target_os = "windows")]
pub unsafe fn generate_surface<E: EntryV1_0, I: InstanceV1_0>(
    entry: &E,
    instance: &I,
    window: &winit::Window,
) -> Result<vk::SurfaceKHR, vk::Result> {

    use winapi::shared::windef::HWND;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winit::os::windows::WindowExt;
    use std::os::raw::c_void;
    use std::ptr;

    let hwnd = window.get_hwnd() as HWND;
    let hinstance = GetModuleHandleW(ptr::null()) as *const c_void;
    let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
        s_type    : vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
        p_next    : ptr::null(),
        flags     : Default::default(),
        hwnd      : hwnd as *const c_void,
        hinstance,
    };
    let win32_surface_loader =
        Win32Surface::new(entry, instance);
    win32_surface_loader.create_win32_surface_khr(&win32_create_info, None)
}
// ------------------------------------------------------------------------
