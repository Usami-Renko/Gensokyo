
#[macro_export]
macro_rules! data_size {
    ($data:expr, $d_type:ty) => (
        (::std::mem::size_of::<$d_type>() * $data.len()) as vkMemorySize
    );
    ($data:expr) => (
        (::std::mem::size_of_value($data)) as vkMemorySize
    );
}

// the macro is copy from crate 'memoffset' v0.2
#[macro_export]
macro_rules! offset_of {
    ($father:ty, $($field:tt)+) => ({
        #[allow(unused_unsafe)]
        let root: $father = unsafe { ::std::mem::uninitialized() };

        let base = &root as *const _ as usize;

        // Future error: borrow of packed field requires unsafe function or block (error E0133)
        #[allow(unused_unsafe)]
        let member =  unsafe { &root.$($field)* as *const _ as usize };

        ::std::mem::forget(root);

        member - base
    });
}

#[macro_export]
macro_rules! collect_handle {
    ($handle_wrapper:expr) => {

        $handle_wrapper.iter()
            .map(|wrapper| wrapper.handle)
            .collect::<Vec<_>>()
    };
}

#[macro_export]
macro_rules! impl_from_err {
    ($sub_err:ident($from_err:ty) -> $impl_err:ident) => (

        impl From<$from_err> for $impl_err {
            fn from(error: $from_err) -> Self {
                $impl_err::$sub_err(error)
            }
        }
    )
}
