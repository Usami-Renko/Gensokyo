
use utility::dimension::Dimension2D;

#[derive(Debug, Clone)]
pub struct WindowConfig {

    pub dimension: Dimension2D,
    pub title    : String,
}

impl Default for WindowConfig {

    fn default() -> WindowConfig {
        WindowConfig {
            dimension: Dimension2D {
                width : 800,
                height: 600,
            },
            title: String::from("hakurei"),
        }
    }
}
