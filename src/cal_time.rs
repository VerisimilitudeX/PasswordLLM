pub mod cal_time {
    use round::{round, round_up};
    use thousands::Separable;
    use crate::gpu::gpu::GPU_Tools::GPU_Status;
    #[allow(non_snake_case)]

    pub fn cal_time(GPU: GPU_Status, Entropy: f64) -> u64 {
        let MD5_Hashrate: u64 = (GPU.GFLOP64() as u64 * 124) * 1000000;
        let combinations = round(f64::powf(2.0, Entropy), 0);

        println!("Possible Combinations {}", combinations.separate_with_commas());
        let time_sec = round_up((combinations / MD5_Hashrate as f64) / 2.0, 0);
        println!("Time: {} seconds", time_sec.separate_with_commas());
        return time_sec as u64
        // 29,586,412,362,451 / 51584000000 = 573 seconds to crack
    }
}
mod test_time {
    #[test]
    #[allow(non_snake_case)]
    pub fn check_time() {
        use crate::gpu::gpu::GPU_Tools::*;
        use crate::cal_time;

        let Entropy = 44.75;

        let mut GPU: GPU_Status = GPU_Status {
            name: "Intel".to_string(),
            clock: 1300,
            cuda_core: 80,
            SMP: 640,
            gflops_fp32: 0,
            gflops_fp64: 0,
        };

        GPU.gflops_fp32 = GPU.GFLOP32();
        GPU.gflops_fp64 = GPU.GFLOP64();

        let result = cal_time(GPU, Entropy);
        assert_eq!(result, 2295)
    }
}
