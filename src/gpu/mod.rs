#[allow(non_snake_case)]
#[allow(unused_imports)]
pub mod gpu { 
    use opencl3::device::{get_all_devices, CL_DEVICE_TYPE_GPU};
    use std::io;
    pub fn obtainGPU() -> Result<(), ()> {
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

        println!("Your {} has {} CUDA cores and {} stream multiprocessors", gpu.name().unwrap(), gpu.max_compute_units().unwrap(), (gpu.max_compute_units().unwrap() * 8));
        //println!("Device: {:?}", device.name());
        //println!("Device: {:?}", device.vendor());
        //println!("Device: {:?}", device.version());
        //println!("Device: {:?}", device.driver_version());
        //println!("MAx compute units: {:?}", device.max_compute_units());
        Ok(())
    }
}