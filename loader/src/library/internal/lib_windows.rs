#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::{os::raw::c_char, ffi::{c_void, CString, CStr}};

pub(crate) struct Library {
    handle : lib_handle
}

impl Library {
    pub unsafe fn load(name: &str) -> Result<Self, crate::library::LoadError> {
        let handle = {
            let name_cstr = CString::new(name)?;
            LoadLibraryA(name_cstr.as_ptr())
        };

        if handle.is_null() {
            return Err(crate::library::LoadError::Windows { code: GetLastError() });
        }

        Ok( Self { handle } )
    }

    pub unsafe fn get_fn(&self, symbol: &str) -> Option<*const c_void> {
        match CString::new(symbol) {
            Ok(cstr) => {
                let ptr = GetProcAddress(self.handle, cstr.as_ptr());
                if ptr.is_null() { return None; }
                Some(ptr)
            },
            Err(_) => None
        }
    }
}

// FFI with Windows library loader
type lib_handle = *const c_void;

#[link(name="kernel32", kind="dylib")]
extern "C" {
    fn LoadLibraryA(filename: *const c_char) -> lib_handle;
    fn GetLastError() -> u32;
    fn GetProcAddress(handle: lib_handle, symbol: *const c_char) -> lib_handle;
    fn FreeLibrary(handle: lib_handle) -> bool;
}