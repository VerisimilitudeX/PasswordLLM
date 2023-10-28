use crate::gpu::GpuStats;
#[allow(non_snake_case)]

// Returns possible combinations and approximate time to crack in seconds
pub fn cal_time(GPU: GpuStats, Entropy: f64) -> (u64, u64) {
    let MD5_Hashrate: u64 = (GPU.gflop64() as u64 * 124) * 1000000;
    let combinations = f64::powf(2.0, Entropy).round();

    let time_sec = f64::ceil((combinations / MD5_Hashrate as f64) / 2.0);

    (combinations as u64, time_sec as u64)   
	// 29,586,412,362,451 / 51584000000 = 573 seconds to crack
}

