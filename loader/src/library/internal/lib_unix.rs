#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::{os::raw::c_char, ffi::{c_void, CString, CStr}};

pub(crate) struct Library {
    handle : lib_handle
}   

impl Library {

    pub fn lib_check<const LIBCOUNT: usize>(names: &[&str; LIBCOUNT]) -> [bool; LIBCOUNT]  {
        crate::library::files_exists::<false, LIBCOUNT, &str>("/usr/lib", names).0
    }
    
    pub unsafe fn load(name: &str) -> Result<Self, crate::library::LoadError> {
        let handle = {
            let name_cstr = CString::new(name)?;
            dlopen(name_cstr.as_ptr(), RTLD_LAZY)
        };

        if handle.is_null() {
            let error = CStr::from_ptr(dlerror());

            return Err(crate::library::LoadError::Unix { 
                message:  error
                    .to_str()?
                    .to_string()
            });
        }

        Ok( Self { handle } )
    }

    pub unsafe fn get_fn(&self, symbol: &str) -> Option<*const c_void> {
        match CString::new(symbol) {
            Ok(cstr) => {
                let ptr = dlsym(self.handle, cstr.as_ptr());
                if ptr.is_null() { return None; }
                Some(ptr)
            },
            Err(_) => None
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            dlclose(self.handle);
        }
    }
}

// FFI with Unix library loader
type lib_handle = *const c_void;
const RTLD_LAZY: i32 = 0x00001;

#[link(name="dl", kind="dylib")]
extern "C" {
    fn dlopen(filename: *const c_char, flags: i32) -> lib_handle;
    fn dlerror() -> *const c_char;
    fn dlsym(handle: lib_handle, symbol: *const c_char) -> lib_handle;
    fn dlclose(handle: lib_handle) -> i32;
}