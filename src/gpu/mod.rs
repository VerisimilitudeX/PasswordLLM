#[allow(non_snake_case)]
#[allow(non_camel_case_types)]

pub mod gpu { 
    use opencl3::device::{get_all_devices, CL_DEVICE_TYPE_GPU};
    use std::io;
    use std::error::Error;

    pub mod GPU_Tools {
        pub struct GPU_Status { // hold and organize information of gpu
            pub name: String,
            pub clock: u32,
            pub cuda_core: u32,
            pub SMP: u32,
            pub gflops_fp32: u32,
            pub gflops_fp64: u32,
        }

        impl GPU_Status {
            pub fn GFLOP64(&self) -> u32 {
                let gpu_gflops_FP64 = ((self.clock * self.cuda_core * 2) / 4) / 1000;
                return gpu_gflops_FP64;
            }
            pub fn GFLOP32(&self) -> u32 {
                let gpu_gflops_FP32: u32 = (self.clock * self.cuda_core * 2) / 1000;
                return gpu_gflops_FP32;
            }
        }
    }
    use GPU_Tools::GPU_Status;
    pub fn obtainGPU(workflow: bool) -> Result<Option<GPU_Status>, Box<dyn Error>> {
        
        if workflow {
            return Ok(None);
        }

        let devices = get_all_devices(CL_DEVICE_TYPE_GPU);
        let devices = match devices {
            Ok(x) => x,
            Err(err) => {
                println!("Cannot detect a GPU!");
                return Err(err.into());
            }
        };

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
        let gpu_device = opencl3::device::Device::new(devices[gpu_choice as usize]);
        let gpu_clock = gpu_device.max_clock_frequency().unwrap();
        let gpu_cores = gpu_device.max_compute_units()?;
        let gpu_cores = gpu_cores * 8;

        let mut GPU: GPU_Status = GPU_Status {
            name: gpu_device.name().unwrap(),
            clock: gpu_clock,
            cuda_core: gpu_cores,
            SMP: gpu_cores * 8,
            gflops_fp32: 0,
            gflops_fp64: 0,
        };

        GPU.gflops_fp32 = GPU.GFLOP32();
        GPU.gflops_fp64 = GPU.GFLOP64();

        println!("Your GPU has a clock speed of {} MHz", GPU.clock);
        println!("Your {} has {} CUDA cores or {} stream multiprocessors", GPU.name, GPU.cuda_core, GPU.SMP);
        println!("This GPU has {} GFLOPS for FP32!", GPU.gflops_fp32); // todo format properly
        println!("Or {} GFLOPS for FP64!", GPU.gflops_fp64);
        Ok(Some(GPU))
    }
}