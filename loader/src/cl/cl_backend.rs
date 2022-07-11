#![allow(non_snake_case)]

use crate::library::Library;
use std::{path::Path, os::raw::c_char};

pub(crate) struct CLBackend {
    library: Library
}

#[cfg(target_os = "linux")]
const LIBNAME : &str = "libOpenCL.so";

#[cfg(target_os = "windows")]
const LIBNAME : &str = "OpenCL.dll";

#[cfg(target_os = "macos")]
const LIBNAME : &str = "libOpenCL.dylib";

impl crate::backend::Backend for CLBackend {
    type Error = Error;

    fn is_installed() -> bool {
        Library::lib_check(&[LIBNAME])[0]
    }

    fn load() -> Result<Self, Self::Error> where Self: Sized {
        unsafe {
            let library = Library::load(LIBNAME)?;
            return Ok(Self { library });
        }
    }
}

pub(crate) enum Error {
    LoadFailed(crate::library::LoadError)
}

impl From<crate::library::LoadError> for Error {
    fn from(load_error: crate::library::LoadError) -> Self {
        Self::LoadFailed(load_error)
    }
}