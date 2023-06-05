#[cfg(test)]
mod test {

    #[test]
    fn test_pool_size_and_entropy() {
        // Test cases with different passcodes
        let test_cases = vec![
            ("password", 36, 38),   // Pool size: 26 (low_case) + 10 (digits), Entropy: 38
            ("Password123", 62, 65),  // Pool size: 26 (low_case) + 26 (up_case) + 10 (digits), Entropy: 65
            ("!@#123", 68, 33),  // Pool size: 26 (low_case) + 26 (up_case) + 10 (digits) + 6 (special), Entropy: 31.0
        ];

        for (passcode, expected_pool_size, expected_entropy) in test_cases {
            let pool_size = GetPoolSize(passcode.to_string());
            assert_eq!(pool_size[0], expected_pool_size);

            let entropy = calculate_entropy(pool_size);
            assert_eq!(entropy, expected_entropy);
        }
    }
}
