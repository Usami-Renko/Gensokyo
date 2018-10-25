
use toml;

pub(super) fn defalut_config_toml() -> toml::Value {

    toml! {
        [core]
        [core.version]
        api         = "1.0.85"
        application = "1.0.0"
        engine      = "1.0.0"

        [core.name]
        application = "Hakurei Program"
        engine      = "Hakurei Rendering Engine"

        [core.validation]
        enable = true
        layers = ["VK_LAYER_LUNARG_standard_validation"]
        flags  = ["Error", "Warning", "PerformanceWarning"]

        [core.device]
        types      = ["CPU", "IntegratedGPU", "DiscreteGPU"]
        features   = []
        extensions = ["swapchain"]
        queue_ops  = []
        queue_request_strategy = "SingleFamilySingleQueue"
        transfer_time_out = "Infinte"
        transfer_duration = 1000

        [core.swapchain]
        image_count = 2
        framebuffer_layers = 1
        prefer_surface_format = "B8g8r8a8Unorm"
        prefer_surface_color_space = "SrgbNonlinear"
        present_mode_primary   = "Mailbox"
        present_mode_secondary = "Fifo"
        acquire_image_time_out = "Infinte"
        acquire_image_duration = 1000

        [window]
        title = "hakurei"

        [window.dimension]
        width  = 800
        height = 600

        [pipeline]

        [pipeline.depth_stencil]
        prefer_depth_stencil_formats = ["D32Sfloat", "D32SfloatS8Uint", "D24UnormS8Uint"]
        prefer_image_tiling = "Optimal"

        [resources]

        [resources.image_load]
        flip_vertical   = false
        flip_horizontal = false
        force_rgba      = true
        byte_per_pixel  = 4
    }
}
