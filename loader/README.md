## Nvidia CUDA and OpenCL Loader for Heikousen

This library is unsafe and only provides backend of Heikousen. Use to load Nvidia CUDA and OpenCL dynamic libraries.

Provides bindings for the following dynamic libraries:
| Backend    | Windows    | Linux                    | MacOS           |
|------------|------------|--------------------------|-----------------|
| OpenCL     | OpenCL.dll | libOpenCL.so             | libOpenCL.dylib |
| NvidiaCUDA | cudart.dll | libcudart.so, libcuda.so | libcudart.dylib |