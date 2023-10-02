#[allow(non_snake_case)]

pub mod gpu { 
    use opencl3::device::{get_all_devices, CL_DEVICE_TYPE_GPU};
    use std::io;
    use crate::round;

    pub fn obtainGPU<T>() -> Result<u32, T> {
        let devices: Vec<*mut std::ffi::c_void> = get_all_devices(CL_DEVICE_TYPE_GPU).expect("Cannot detect a GPU!");
        let mut gpu_choice: u32 = 0;

        if devices.len() > 1 {
            loop {
                println!("You have multiple ({}) GPUs, please select one!", devices.len());
                for gpu in 0..devices.len() {
                    let device = devices[gpu];
                    let device = opencl3::device::Device::new(device);
                    println!("\tGPU Device {}: {}", gpu, device.name().unwrap());
                }
                print!("Selection: ");

                let mut tree = String::new();
                io::stdin().read_line(&mut tree).expect("Failed to read line");
                let water = tree.trim();

                match water.parse::<u32>() {
                    Ok(_) => {
                        gpu_choice = water.parse().unwrap();
                        if gpu_choice > devices.len() as u32 {
                            panic!("Please select a valid GPU!")
                        }
                        break;
                    }
                    Err(_) => {
                        println!("Please type a number!");
                    }
                }
            }
        }
        println!("You have selected GPU Device {}", gpu_choice);
        let gpu = opencl3::device::Device::new(devices[gpu_choice as usize]);
        let gpu_clock = gpu.max_clock_frequency().unwrap();
        let gpu_cores = gpu.max_compute_units();
        let gpu_cores = gpu_cores.unwrap(); // use ?
        let gpu_cores = gpu_cores * 8;

        let gpu_gflops: u32 = gpu_clock * gpu_cores * 2;

        println!("Your GPU has a clock speed of {} MHz", gpu_clock);
        println!("Your {} has {} CUDA cores and {} stream multiprocessors", gpu.name().unwrap(), gpu.max_compute_units().unwrap(), (gpu.max_compute_units().unwrap() * 8));
        println!("This GPU has {} GFLOPS for FP32!", gpu_gflops.to_string()); // todo format properly
        Ok(gpu_gflops)
    }
}