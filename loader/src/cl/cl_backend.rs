use std::os::raw::c_char;
use std::ffi::c_void;

use crate::library::{Library, LoadError};
use crate::cl;

pub(crate) struct CLBackend {
    library: Library,

    // Function table
    pub(in super) cl_get_device_ids: extern fn(cl::platform_id, cl::device_type, u32, *mut cl::device_id, *mut u32) -> i32,
    pub(in super) cl_get_device_info: extern fn(cl::device_id, cl::device_info, usize, *mut c_void, *mut usize) -> i32,
    pub(in super) cl_create_context: extern fn(*const cl::context_properties, u32, *const cl::device_id, usize, *mut c_void, *mut i32) -> cl::context,
    pub(in super) cl_release_context: extern fn(cl::context) -> i32,
    pub(in super) cl_create_program_with_il: extern fn(cl::context, *const c_void, usize, *mut i32) -> cl::program,
    pub(in super) cl_enqueue_nd_range_kernel: extern fn(cl::command_queue, cl::kernel, u32, *const usize, *const usize, *const usize, u32, *const cl::event, *mut cl::event) -> i32,
    pub(in super) cl_build_program: extern fn(cl::program, u32, *const cl::device_id, *const c_char, usize,*mut c_void) -> i32,
    pub(in super) cl_create_command_queue: extern fn(cl::context, cl::device_id, cl::command_queue_properties, *mut i32) -> cl::command_queue,
    pub(in super) cl_create_kernel: extern fn(cl::program, *const c_char, *mut i32) -> cl::kernel,
    pub(in super) cl_finish: extern fn(cl::command_queue) -> i32
}

// OpenCL name depends on OS
#[cfg(target_os = "linux")]
const LIBNAME : &str = "libOpenCL.so";

#[cfg(target_os = "windows")]
const LIBNAME : &str = "OpenCL.dll";

#[cfg(target_os = "macos")]
const LIBNAME : &str = "libOpenCL.dylib";

// CL Backend implementation
impl crate::backend::Backend for CLBackend {
    fn is_installed() -> bool {
        Library::lib_check(&[LIBNAME])[0]
    }

    unsafe fn load() -> Result<Self, LoadError> where Self: Sized {
        let library = Library::load(LIBNAME)?;

        // Load functions
        let cl_get_device_ids = library.get_fn("clGetDeviceIDs")?;
        let cl_get_device_info = library.get_fn("clGetDeviceInfo")?;
        let cl_create_context = library.get_fn("clCreateContext")?;
        let cl_release_context = library.get_fn("clReleaseContext")?;
        let cl_create_program_with_il = library.get_fn("clCreateProgramWithIL")?;
        let cl_enqueue_nd_range_kernel = library.get_fn("clEnqueueNDRangeKernel")?;
        let cl_build_program = library.get_fn("clBuildProgram")?;
        let cl_create_command_queue = library.get_fn("clCreateCommandQueue")?;
        let cl_create_kernel = library.get_fn("clCreateKernel")?;
        let cl_finish = library.get_fn("clFinish")?;

        use std::mem::transmute;

        Ok(Self {
            library,
            cl_get_device_ids: transmute(cl_get_device_ids),
            cl_get_device_info: transmute(cl_get_device_info),
            cl_create_context: transmute(cl_create_context),
            cl_release_context: transmute(cl_release_context),
            cl_create_program_with_il: transmute(cl_create_program_with_il),
            cl_enqueue_nd_range_kernel: transmute(cl_enqueue_nd_range_kernel),
            cl_build_program: transmute(cl_build_program),
            cl_create_command_queue: transmute(cl_create_command_queue),
            cl_create_kernel: transmute(cl_create_kernel),
            cl_finish: transmute(cl_finish)
        })
    }
}