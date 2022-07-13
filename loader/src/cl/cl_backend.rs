#![allow(non_snake_case)]

use std::{ffi::c_void, borrow::Borrow};

use crate::library::{Library, LoadError};

pub(crate) struct CLBackend {
    library: Library,
    clGetDeviceIDs: *const c_void,
    clGetDeviceInfo: *const c_void,
    clCreateContext: *const c_void,
}

#[cfg(target_os = "linux")]
const LIBNAME : &str = "libOpenCL.so";

#[cfg(target_os = "windows")]
const LIBNAME : &str = "OpenCL.dll";

#[cfg(target_os = "macos")]
const LIBNAME : &str = "libOpenCL.dylib";

impl crate::backend::Backend for CLBackend {
    fn is_installed() -> bool {
        Library::lib_check(&[LIBNAME])[0]
    }

    fn load() -> Result<Self, LoadError> where Self: Sized {
        let library;
        let clGetDeviceIDs;
        let clGetDeviceInfo;
        let clCreateContext;

        unsafe {
            library = Library::load(LIBNAME)?;

            // Load funcitons
            clGetDeviceIDs = library.get_fn("clGetDeviceIDs")?;
            clGetDeviceInfo = library.get_fn("clGetDeviceInfo")?;
            clCreateContext = library.get_fn("clCreateContext")?; 
        }

        Ok(Self {
            library,
            clGetDeviceIDs,
            clGetDeviceInfo,
            clCreateContext
        })
    }
}