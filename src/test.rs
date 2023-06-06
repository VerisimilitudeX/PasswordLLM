#[cfg(test)]
mod test {
    use super::*;
    
    // Test cases with different passcodes
    static TEST_CASES: &[(&str, u64, i64)] = &[
        ("password", 26, 38),     // Pool size: 26 (low_case) + 10 (digits), Entropy: 38
        ("Password123", 62, 65),  // Pool size: 26 (low_case) + 26 (up_case) + 10 (digits), Entropy: 65
        ("!@#123", 42, 32),       // Pool size: 26 (low_case) + 26 (up_case) + 10 (digits) + 6 (special), Entropy: 32
    ];

    #[test]
    fn test_pool_size_and_entropy() {
        use crate::calculate_entropy;
        use crate::get_pool_size;
        
        for &(passcode, expected_pool_size, expected_entropy) in TEST_CASES {
            let pool_size = get_pool_size(passcode.to_string());
  
            assert_eq!(*pool_size.get(0).unwrap(), expected_pool_size, "Pool size is not correct {} != {}, passcode = {:?}.", pool_size[0], expected_pool_size, passcode);

            let entropy = calculate_entropy(pool_size);
            assert_eq!(entropy.round() as i64, expected_entropy)
        }
    }
    
    #[test]
    fn pwned_test() {
        use crate::check_if_pwned;
        
        for &(passcode, _, _) in TEST_CASES {
            check_if_pwned(passcode.to_string());
        }
    }
}
