
pub use self::obj::{ ObjDataEntity, ModelObjLoader };
pub use self::error::ModelLoadingErr;

pub(crate) use self::error::ModelObjLoadingError;

mod obj;
mod error;
