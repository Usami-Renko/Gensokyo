
use toml::{ toml, toml_internal };

pub(super) fn manifest_toml() -> ::toml::Value {

    toml! {
        [core]
        [core.version]
        api         = "1.0.97"
        application = "1.0.0"
        engine      = "1.0.0"

        [core.name]
        application = "Gensokyo Program"
        engine      = "Gensokyo Rendering Engine"

        [core.validation]
        enable = true
        layers = ["VK_LAYER_LUNARG_standard_validation"]
        types  = "DebugUtils"
        print_instance_layers = false

        [core.validation.report]
        flags = ["Error", "Warning", "PerformanceWarning"]

        [core.validation.utils]
        flags    = []
        severity = ["Warning", "Error"]
        types    = ["General", "Performance", "Validation"]

        [core.device]
        queue_request_strategy = "SingleFamilySingleQueue"
        transfer_time_out = "Infinite"
        transfer_duration = 1000

        [core.device.print]
        device_name        = false
        device_api_version = false
        device_type        = false
        device_queues      = false

        [core.physical]
        device_types       = ["CPU", "IntegratedGPU", "DiscreteGPU"]
        features           = []
        extensions         = ["VK_KHR_swapchain"]
        queue_capabilities = []

        [core.swapchain]
        image_count = 2
        framebuffer_layers = 1
        prefer_surface_format = "B8G8R8A8_UNORM"
        prefer_surface_color_space = "SrgbNonlinear"
        present_mode_primary   = "Mailbox"
        present_mode_secondary = "Fifo"
        acquire_image_time_out = "Infinite"
        acquire_image_duration = 1000

        [window]
        title = "Gensokyo"
        mode  = "normal"
        always_on_top = false
        is_resizable  = true

        [window.dimension]
        width  = 800
        height = 600
        min_width  = 400
        min_height = 300
        max_width  = 1280
        min_width  = 720

        [window.cursor]
        is_grab = false
        is_hide = false

        [pipeline]

        [pipeline.depth_stencil]
        prefer_depth_stencil_formats = ["D32_SFLOAT", "D32_SFLOAT_S8_UINT", "D24_UNORM_S8_UINT"]

        [resources]

        [resources.image_load]
        flip_vertical   = false
        flip_horizontal = false
        force_rgba      = true
        byte_per_pixel  = 4
    }
}

#[allow(dead_code)]
fn complete_manifest_toml() {

//    [core]
//    [core.version]
//    api         = "0.0.0"
//    application = "0.0.0"
//    engine      = "0.0.0"
//
//    [core.name]
//    application = ""
//    engine      = ""
//
//    [core.validation]
//    enable = true
//    layers = ["VK_LAYER_LUNARG_standard_validation"]
//    types  = "DebugReport" | "DebugUtils"
//    print_instance_layers = false
//
//    [core.validation.report]
//    flags = ["Error", "Warning", "PerformanceWarning", "Debug", "Information"]
//
//    [core.validation.utils]
//    flags    = []
//    severity = ["Verbose", "Warning", "Error", "Info"]
//    types    = ["General", "Performance", "Validation"]
//
//    [core.device]
//    queue_request_strategy = "SingleFamilySingleQueue" | "SingleFamilyMultiQueues"
//    transfer_time_out = "Infinte" | "Immediate" | "Timing"
//    transfer_duration = 1000
//
//    [core.device.print]
//    device_name        = false
//    device_api_version = false
//    device_type        = false
//    device_queues      = false
//
//    [core.physical]
//    device_types       = ["CPU", "IntegratedGPU", "DiscreteGPU", "VirtualGPU"]
//    features           = [] // not cover here.
//    extensions         = ["VK_KHR_swapchain"]
//    queue_capabilities = ["Graphics", "Compute", "Transfer", "SparseBinding", "Protected"]
//
//    [core.swapchain]
//    image_count = 2
//    framebuffer_layers = 1
//    prefer_surface_format = "B8G8R8A8_UNORM"
//    prefer_surface_color_space = "SrgbNonlinear"
//    present_mode_primary   = "Mailbox" | "Immediate" | "Fifo" | "FifoRelaxed"
//    present_mode_secondary = "Mailbox" | "Immediate" | "Fifo" | "FifoRelaxed"
//    acquire_image_time_out = "Infinte" | "Immediate" | "Timing"
//    acquire_image_duration = 1000
//
//    [window]
//    title = ""
//    mode  = "normal" | "maximized" | "fullscreen"
//    always_on_top = false
//    is_resizable  = true
//
//    [window.dimension]
//    width  = 800
//    height = 600
//    min_width  = 400
//    min_height = 300
//    max_width  = 1280
//    min_width  = 720
//
//    [window.cursor]
//    is_grab = false
//    is_hide = false
//
//    [pipeline]
//
//    [pipeline.depth_stencil]
//    prefer_depth_stencil_formats = ["D32_SFLOAT", "D32_SFLOAT_S8_UINT", "D24_UNORM_S8_UINT"]
//    prefer_image_tiling = "Optimal" | "Linear"
//
//    [resources]
//
//    [resources.image_load]
//    flip_vertical   = false
//    flip_horizontal = false
//    force_rgba      = true
//    byte_per_pixel  = 4
}