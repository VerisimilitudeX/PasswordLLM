use opencl3::device::{get_all_devices, Device, CL_DEVICE_TYPE_GPU};
use opencl3::error_codes::ClError;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GpuStats {
    pub name: String,
    pub clock: u32,
    pub cuda_cores: u32,
    pub smp: u32,
    pub gflops_fp32: u32,
    pub gflops_fp64: u32,
}

impl GpuStats {
    pub fn gflop64(&self) -> u32 {
        (self.clock * self.smp / 2) / 1000
    }

    pub fn gflop32(&self) -> u32 {
        (self.clock * self.smp * 2) / 1000
    }
}

pub fn list_gpus() -> Result<Vec<Device>, ClError> {
    Ok(get_all_devices(CL_DEVICE_TYPE_GPU)?
        .into_iter()
        .map(Device::new)
        .collect())
}

pub fn obtain_stats_from_device(
    device: &opencl3::device::Device,
    workflow: bool,
) -> Result<Option<GpuStats>, ClError> {
    if workflow {
        return Ok(None);
    }

    let clock = device.max_clock_frequency().unwrap();
    let cores = device.max_compute_units()?;

    let mut gpu: GpuStats = GpuStats {
        name: device.name().unwrap(),
        clock,
        cuda_cores: cores,
        smp: cores * 8,
        gflops_fp32: 0,
        gflops_fp64: 0,
    };

    gpu.gflops_fp32 = gpu.gflop32();
    gpu.gflops_fp64 = gpu.gflop64();

    Ok(Some(gpu))
}

pub fn obtain_stats_from_index(
    device_index: usize,
    workflow: bool,
) -> Result<Option<GpuStats>, ClError> {
    if workflow {
        return Ok(None);
    }

    let devices = list_gpus()?;
    let device = devices[device_index];

    obtain_stats_from_device(&device, workflow)
}
