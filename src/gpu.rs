#[allow(non_snake_case)]
#[allow(unused_imports)]

pub mod gpu {
    use rust_gpu_tools::{opencl, Device, GPUError, Program, Vendor};

    pub fn obtainGPU() {
        let device = Device::all();
        println!("{:?}", device);
    }
}