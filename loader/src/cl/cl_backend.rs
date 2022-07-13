#![allow(non_snake_case)]

use std::{ffi::c_void, borrow::Borrow};

use crate::library::{Library, LoadError};

pub(crate) struct CLBackend {
    library: Library,
    clGetDeviceIDs: *const c_void,
    clGetDeviceInfo: *const c_void,
    clCreateContext: *const c_void,
    clReleaseContext: *const c_void,
    clCreateProgramWithIL: *const c_void,
    clEnqueueNDRangeKernel: *const c_void,
    clBuildProgram: *const c_void,
    clCreateCommandQueue: *const c_void,
    clCreateKernel: *const c_void,
    clFinish: *const c_void,
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
        let clReleaseContext;
        let clCreateProgramWithIL;
        let clEnqueueNDRangeKernel;
        let clBuildProgram;
        let clCreateCommandQueue;
        let clCreateKernel;
        let clFinish;

        unsafe {
            library = Library::load(LIBNAME)?;

            // Load funcitons
            clGetDeviceIDs = library.get_fn("clGetDeviceIDs")?;
            clGetDeviceInfo = library.get_fn("clGetDeviceInfo")?;
            clCreateContext = library.get_fn("clCreateContext")?;
            clReleaseContext = library.get_fn("clReleaseContext")?;
            clCreateProgramWithIL = library.get_fn("clCreateProgramWithIL")?;
            clEnqueueNDRangeKernel = library.get_fn("clEnqueueNDRangeKernel")?;
            clBuildProgram = library.get_fn("clBuildProgram")?;
            clCreateCommandQueue = library.get_fn("clCreateCommandQueue")?;
            clCreateKernel = library.get_fn("clCreateKernel")?;
            clFinish = library.get_fn("clFinish")?;
        }

        Ok(Self {
            library,
            clGetDeviceIDs,
            clGetDeviceInfo,
            clCreateContext,
            clReleaseContext,
            clCreateProgramWithIL,
            clEnqueueNDRangeKernel,
            clBuildProgram,
            clCreateCommandQueue,
            clCreateKernel,
            clFinish,
        })
    }
}