#[cfg(test)]
mod test {

    #[test]
    fn test_pool_size_and_entropy() {
        use crate::calculate_entropy;
        use crate::GetPoolSize;
        // Test cases with different passcodes
        let test_cases = vec![
            ("password", 26, 38),   // Pool size: 26 (low_case) + 10 (digits), Entropy: 38
            ("Password123", 62, 65),  // Pool size: 26 (low_case) + 26 (up_case) + 10 (digits), Entropy: 65
            ("!@#123", 43, 32),  // Pool size: 26 (low_case) + 26 (up_case) + 10 (digits) + 6 (special), Entropy: 32
        ];

        for (passcode, expected_pool_size, expected_entropy) in test_cases {
            println!("Passcode: {}", passcode);
            let pool_size = GetPoolSize(passcode.to_string());
  
            assert_eq!(pool_size[0], expected_pool_size, "Pool size is not correct {} != {}", pool_size[0], expected_pool_size);

            let entropy = calculate_entropy(pool_size);
            assert_eq!(entropy, expected_entropy.try_into().unwrap())
        }
    }
}
