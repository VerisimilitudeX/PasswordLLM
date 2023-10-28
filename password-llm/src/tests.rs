#[cfg(test)]
mod test {
    use crate::{utils::pwned_api, cal_time};
    
    // Test cases with different passcodes
    static TEST_CASES: &[(&str, u64, i64, u32, bool)] = &[
        ("password", 26, 38, 9659365, true),     // Pool size: 26 (low_case) + 10 (digits), Entropy: 38, rockyoupasswordlist: True
        ("Password123", 62, 66, 47708, true),  // Pool size: 26 (low_case) + 26 (up_case) + 10 (digits), Entropy: 65, rockyoupasswordlist: True
        ("!@#123", 42, 32, 1259, true),       // Pool size: 26 (low_case) + 26 (up_case) + 10 (digits) + 6 (special), Entropy: 32, rockyoupasswordlist: True
    ]; // password string, pool size, entropy, expected database count

    #[test]
    fn test_pool_size_and_entropy() {
        use crate::calculate_entropy;
        use crate::get_pool_size;
        
        for &(passcode, expected_pool_size, expected_entropy, _, _) in TEST_CASES {
            let pool_size = get_pool_size(passcode.to_string());
  
            assert_eq!(*pool_size.first().unwrap(), expected_pool_size, "Pool size is not correct {} != {}, passcode = {:?}.", pool_size[0], expected_pool_size, passcode);

            let entropy = calculate_entropy(pool_size);
            assert_eq!(entropy.round() as i64, expected_entropy)
        }
    }
    #[tokio::test]
    async fn test_password_list() {
        use crate::check_dictionary;
        for &(passcode, _, _, _, detected) in TEST_CASES {
            let result = check_dictionary(passcode.to_string(), std::fs::File::open(format!("{}/../src-tauri/resources/rockyou.txt", env!("CARGO_MANIFEST_DIR"))).unwrap()).await;
            assert_eq!(result, detected)
        }
    }

    
    #[tokio::test] 
    async fn pwned_test() {
        for &(passcode, _, _, expected_count, _) in TEST_CASES {
            let count = pwned_api::pass_check(passcode).await.unwrap();
            assert!(count >= expected_count as u64, "Count should be greater than or equal to expected_count");
        }
    }

	#[test]
    #[allow(non_snake_case)]
    pub fn check_time() {
        use crate::gpu::*;

        let Entropy = 44.75;

        let mut GPU: GpuStats = GpuStats {
            name: "Intel".to_string(),
            clock: 1300,
            cuda_cores: 80,
            smp: 640,
            gflops_fp32: 0,
            gflops_fp64: 0,
        };

        GPU.gflops_fp32 = GPU.gflop32();
        GPU.gflops_fp64 = GPU.gflop64();

        let (_, duration) = cal_time(GPU, Entropy);
        assert_eq!(duration, 287)
    }
}
