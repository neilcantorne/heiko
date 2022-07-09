<p align="center">
  <img src="assets/logo.svg" width=128 height=128/>
</p>
<h1 align="center">Heikousen</h1>

<p align="center" style="vertical-align:middle">
    <a href="https://github.com/neilcantorne/heiko/actions/workflows/ci.yml">
        <img src="https://github.com/neilcantorne/heiko/actions/workflows/ci.yml/badge.svg?branch=dev" />
    </a>
</p>

<p>
    Heikousen ("Parallel Lines") lines of code that leverage GPU parallel execution.
    Write compute kernels in idiomatic Rust code, and execute on GPU and other hardware accelerators through OpenCL or Nvidia CUDA backend. <br>
    <b>Work in progress</b>
</p>

---

## Usage
Example of convolutional operation commonly use in Convolutional Neural Networks.

**Define Compute Kernel**<br>
Write compute kernel like normal Rust function and use heiko::kernel macro to indicate that the function is a compute kernel.
The compute kernel will be compiled to NVVM PTX for Nvidia CUDA or SPIR-V for OpenCL.

```rust
use heiko::Tensor2d;

// Convolutional filter for grayscale image
// This function/compute kernel will execute on GPU
#[heiko::kernel]
fn convolve(image: &Tensor2d<f32>, filter: &Tensor2d<f32>,
    indexed!(index: &TensorIndex2d), output: &mut Tensor2d<f32>) {
    output[region] = image[index].dot_product(filter);
}
```

**Find a device**<br>
Use heiko::Device::devices() to retrieve available devices on your computer.

```rust
use heiko::Device;

fn get_device() -> Option<Device> {
    // Search for GPU device
    while let Some(device) = Device::devices(DeviceType::Gpu) {
        return Some(device);
    }

    return None;
}
```

**Compile and Call Kernel**<br>
Heikousen supports async execution through Tokio runtime.

```rust
use heiko::{Context, Device, KernelExecError, Tensor2d};

#[tokio::main]
async fn main() -> Result<(), KernelExecError>{
    // Search for device
    if let Some(device) = get_device() {
        // create a context from device
        let context = Context::new(device);

        // load image and convert to tensor
        let image = Tensor2d::<f32>::from_image_file("image.png", Channel::Lightness);
        
        // create filter matrix
        let filter = heiko::mat_f32!([
             1,  1,  1 |
             0,  0,  0 |
            -1, -1, -1 
        ]);

        let stride = heiko::Vec2d::new(2, 2);

        // Create buffer for storing output
        let mut output = heiko::Tensor2d::new_zeros(
            (image.width - filter.width) / stride.x, + 1, 
            (image.height - filter.height) / stride.y + 1
        );

        // Compile kernel
        let convolve_kernel = context.use_kernel(convolve)
            .index_param::<0>(Tensor2d::slide_xy(stride)) // slide based on first parameter
            .compile();

        //Execute kernel the resulting matrix are written on output variable
        convolve_kernel(&image, &filter, &mut output).await?;
    }
    else {
        println!("No device available");
    }
}

```