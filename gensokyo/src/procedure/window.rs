
use winit;

use crate::config::window::WindowConfig;

use gsvk::types::vkDim2D;

pub(crate) struct WindowInfo {

    window_size : vkDim2D,
    window_title: String,

    config: WindowConfig,
}

impl WindowInfo {

    pub fn from(config: WindowConfig) -> WindowInfo {

        let window_size = config.dimension.clone();
        let window_title = config.title.clone();

        WindowInfo {
            window_size, window_title, config,
        }
    }

    pub fn build(&self, event_loop: &winit::EventsLoop) -> Result<winit::Window, winit::CreationError> {

        let mut builder = winit::WindowBuilder::new()
            .with_title(self.window_title.clone())
            .with_dimensions((self.window_size.width, self.window_size.height).into());

        if self.config.always_on_top {
            builder = builder.with_always_on_top(true);
        }

        builder = if self.config.is_resizable {
            builder.with_resizable(true)
        } else {
            builder.with_resizable(false)
        };

        builder = match self.config.mode.as_str() {
            | "normal"     => builder,
            | "maximized"  => builder.with_maximized(true),
            | "fullscreen" => {
                let primary_monitor = event_loop.get_primary_monitor();
                builder.with_fullscreen(Some(primary_monitor))
            },
            | _ => builder,
        };

        if let Some(min) = self.config.min_dimension {
            builder = builder.with_min_dimensions((min.width as f64, min.height as f64).into());
        }
        if let Some(max) = self.config.max_dimension {
            builder = builder.with_max_dimensions((max.width as f64, max.height as f64).into());
        }

        let window = builder.build(event_loop)?;

        if self.config.is_cursor_grap {
            window.grab_cursor(true).unwrap();
        }

        if self.config.is_cursor_hide {
            window.hide_cursor(true);
        }

        Ok(window)
    }
}
