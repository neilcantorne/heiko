use std::{os::raw::c_char, ffi::c_void};

pub(crate) struct Library {

}

impl Library {
    pub unsafe fn load(name: &str) -> Result<Self, crate::library::LoadError> {
        todo!();
    }

    pub unsafe fn get_fn<T>(&self, symbol: &str) -> Option<T>{
        todo!();
    }
}

// FFI with Linux library loader
type lib_handle = *const c_void;
const RTLD_LAZY: i32 = 0x00001;

#[link(name="dl", kind="dylib")]
extern "C" {
    fn dlopen(filename: *const c_char, flags: i32) -> lib_handle;
    fn dlerror() -> *const c_char;
    fn dlsym(handle: lib_handle, symbol: *const c_char) -> lib_handle;
    fn dlclose(handle: lib_handle) -> i32;
}