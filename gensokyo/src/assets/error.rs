
use std::fmt;
use std::error::Error;

use gsma::impl_from_err;

use crate::assets::model::ModelLoadingError;

#[derive(Debug)]
pub enum AssetsError {

    Io(IoError),
    Model(ModelLoadingError),
}

impl Error for AssetsError {}
impl fmt::Display for AssetsError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | AssetsError::Io(ref e)    => e.to_string(),
            | AssetsError::Model(ref e) => e.to_string(),
        };

        write!(f, "{}", description)
    }
}

impl_from_err!(Io(IoError) -> AssetsError);
impl_from_err!(Model(ModelLoadingError) -> AssetsError);

#[derive(Debug)]
pub enum IoError {

    ImageSourceLoadingError,
}

impl Error for IoError {}
impl fmt::Display for IoError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | IoError::ImageSourceLoadingError => "Failed to load image from source.",
        };

        write!(f, "{}", description)
    }
}
