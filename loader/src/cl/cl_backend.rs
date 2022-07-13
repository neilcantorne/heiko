#![allow(non_snake_case)]

use std::os::raw::c_char;
use std::{ffi::c_void, borrow::Borrow};

use crate::library::{Library, LoadError};
use crate::cl;

pub(crate) struct CLBackend {
    library: Library,

    // Function table
    pub(in super) clGetDeviceIDs: extern fn(cl::platform_id, cl::device_type, u32, *mut cl::device_id, *mut u32) -> i32,
    pub(in super) clGetDeviceInfo: extern fn(cl::device_id, cl::device_info, usize, *mut c_void, *mut usize) -> i32,
    pub(in super) clCreateContext: extern fn(*const cl::context_properties, u32, *const cl::device_id, usize, *mut c_void, *mut i32) -> cl::context,
    pub(in super) clReleaseContext: extern fn(cl::context) -> i32,
    pub(in super) clCreateProgramWithIL: extern fn(cl::context, *const c_void, usize, *mut i32) -> cl::program,
    pub(in super) clEnqueueNDRangeKernel: extern fn(cl::command_queue, cl::kernel, u32, *const usize, *const usize, *const usize, u32, *const cl::event, *mut cl::event) -> i32,
    pub(in super) clBuildProgram: extern fn(cl::program, u32, *const cl::device_id, *const c_char, usize,*mut c_void) -> i32,
    pub(in super) clCreateCommandQueue: extern fn(cl::context, cl::device_id, cl::command_queue_properties, *mut i32) -> cl::command_queue,
    pub(in super) clCreateKernel: extern fn(cl::program, *const c_char, *mut i32) -> cl::kernel,
    pub(in super) clFinish: extern fn(cl::command_queue) -> i32
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

    unsafe fn load() -> Result<Self, LoadError> where Self: Sized {
        let library = Library::load(LIBNAME)?;

        // Load funcitons
        let clGetDeviceIDs = library.get_fn("clGetDeviceIDs")?;
        let clGetDeviceInfo = library.get_fn("clGetDeviceInfo")?;
        let clCreateContext = library.get_fn("clCreateContext")?;
        let clReleaseContext = library.get_fn("clReleaseContext")?;
        let clCreateProgramWithIL = library.get_fn("clCreateProgramWithIL")?;
        let clEnqueueNDRangeKernel = library.get_fn("clEnqueueNDRangeKernel")?;
        let clBuildProgram = library.get_fn("clBuildProgram")?;
        let clCreateCommandQueue = library.get_fn("clCreateCommandQueue")?;
        let clCreateKernel = library.get_fn("clCreateKernel")?;
        let clFinish = library.get_fn("clFinish")?;

        use std::mem::transmute;

        Ok(Self {
            library,
            clGetDeviceIDs: transmute(clGetDeviceIDs),
            clGetDeviceInfo: transmute(clGetDeviceInfo),
            clCreateContext: transmute(clCreateContext),
            clReleaseContext: transmute(clReleaseContext),
            clCreateProgramWithIL: transmute(clCreateProgramWithIL),
            clEnqueueNDRangeKernel: transmute(clEnqueueNDRangeKernel),
            clBuildProgram: transmute(clBuildProgram),
            clCreateCommandQueue: transmute(clCreateCommandQueue),
            clCreateKernel: transmute(clCreateKernel),
            clFinish: transmute(clFinish)
        })
    }
}