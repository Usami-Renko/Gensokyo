
use toml::de::Error as TomlError;
use std::fmt;
use std::error::Error;

/// possible error may occur during the creation of vk::Instance.
#[derive(Debug, Clone)]
pub enum ConfigError {

    ParseError,
    UserConfigSyntaxError(TomlError),
    DirectoryAccessError,
    IoError,
    Mapping(MappingError),
}

impl Error for ConfigError {}
impl fmt::Display for ConfigError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | ConfigError::ParseError => String::from("Failed to parse toml file."),
            | ConfigError::UserConfigSyntaxError(ref e) => format!("Failed to read user manifest, syntax error: {}", e),
            | ConfigError::DirectoryAccessError => String::from("Failed to access the current working directory."),
            | ConfigError::IoError => String::from("Failed to perform I/O operation."),
            | ConfigError::Mapping(ref e) => e.to_string(),
        };

        write!(f, "{}", description)
    }
}

impl_from_err!(Mapping(MappingError) -> ConfigError);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MappingError {

    DebugReportError,
    DeviceTypeError,
    DeviceQueueOperationError,
    DeviceTransferTimeError,
    QueueStrategyError,
    PhysicalFeatureError,
    PhysicalExtensionError,
    SwapchainPresentModeError,
    SwapchainImageTimeAcqurieError,
    FormatMappingError,
    ColorspaceMappingError,
    ImgTilingMappingError,
}

impl Error for MappingError {}
impl fmt::Display for MappingError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | MappingError::DebugReportError               => "Failed to recognize request debug report flag.",
            | MappingError::DeviceTypeError                => "Failed to recognize request device type.",
            | MappingError::DeviceQueueOperationError      => "Failed to recognize request device queue operation.",
            | MappingError::DeviceTransferTimeError        => "Failed to recognize request device transfer time.",
            | MappingError::QueueStrategyError             => "Failed to recognize request Queue request strategy.",
            | MappingError::PhysicalFeatureError           => "Failed to recognize request physical device feature.",
            | MappingError::PhysicalExtensionError         => "Failed to recognize request physical device extension.",
            | MappingError::SwapchainPresentModeError      => "Failed to recognize request swapchain present mode.",
            | MappingError::SwapchainImageTimeAcqurieError => "Failed to recognize request swapchain image acquire time.",
            | MappingError::FormatMappingError             => "Failed to recognize request format.",
            | MappingError::ColorspaceMappingError         => "Failed to recognize request color space.",
            | MappingError::ImgTilingMappingError          => "Failed to recognize request image titling.",
        };

        write!(f, "{}", description)
    }
}
