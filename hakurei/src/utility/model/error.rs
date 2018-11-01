
use std::fmt;
use std::error::Error;

use tobj::LoadError;

#[derive(Debug)]
pub enum ModelLoadingErr {

    Obj(ModelObjLoadingError),
}

impl Error for ModelLoadingErr {}
impl fmt::Display for ModelLoadingErr {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | ModelLoadingErr::Obj(e) => e.to_string(),
        };

        write!(f, "{}", description)
    }
}


#[derive(Debug)]
pub enum ModelObjLoadingError {

    Loading(LoadError),
}

impl Error for ModelObjLoadingError {}
impl fmt::Display for ModelObjLoadingError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | ModelObjLoadingError::Loading(e) => e.to_string(),
        };

        write!(f, "{}", description)
    }
}
