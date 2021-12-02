use crate::port::PortType;
use crate::port_collection::PortCollection;
use core::ffi::c_void;
use core::ops::Deref;
use core::ptr::NonNull;

/// Handle for input ports.
///
/// Fields of this type can be dereferenced to the input type of the port type.
pub struct InputPort<T: PortType> {
    port: T::InputPortType,
}

impl<T: PortType> Deref for InputPort<T> {
    type Target = T::InputPortType;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.port
    }
}

impl<T: PortType> PortCollection for InputPort<T> {
    type Cache = *mut c_void;

    unsafe fn from_connections(cache: &Self::Cache, sample_count: u32) -> Option<Self> {
        Some(Self {
            port: T::input_from_raw(NonNull::new(*cache)?, sample_count),
        })
    }
}
