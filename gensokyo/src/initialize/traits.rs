
use crate::initialize::initializer::AssetInitializer;
use crate::error::GsResult;

pub trait FromInitializer<R=Self>: Sized {

    fn new(initializer: &AssetInitializer) -> R;
}

pub trait TryFromInitializer<R=Self>: Sized {

    fn new(initializer: &AssetInitializer) -> GsResult<R>;
}

pub trait FromInitializerP1<P1, R=Self>: Sized {

    fn new(initializer: &AssetInitializer, param: P1) -> R;
}

pub trait TryFromInitializerP1<P1, R=Self>: Sized {

    fn new(initializer: &AssetInitializer, param: P1) -> GsResult<R>;
}

pub trait FromInitializerP2<P1, P2, R=Self>: Sized {

    fn new(initializer: &AssetInitializer, param1: P1, param2: P2) -> Self;
}

pub trait TryFromInitializerP2<P1, P2, R=Self>: Sized {

    fn new(initializer: &AssetInitializer, param1: P1, param2: P2) -> GsResult<R>;
}
