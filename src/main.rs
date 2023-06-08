mod tests;
use std::env;
use regex::Regex;
mod utils;
use utils::PwnedAPI::pass_check;
use round::round;
// Entire file calculates the entropy

pub fn main(){
    let args: Vec<String> = env::args().collect();
    let password;

    if args.len() > 1 {
        password = args[1].clone();
    } else {
        password = rpassword::prompt_password("Password: ").unwrap();
    }

    let pool_size = get_pool_size(password.to_string());
    let entropy = calculate_entropy(pool_size);
    check_if_pwned(password);

    match entropy as i64 {
        strength if strength < 80 => println!("Password strength: Weak"),
        strength if strength < 112 => println!("Password strength: Medium"),
        strength if strength < 128 => println!("Password strength: Strong"),
        _ => println!("Password strength: Very Strong"),
    }
    
    println!("Entropy: {} bits", entropy);
}

// Pool size based on https://github.com/Kush-munot/Password-Strength-Checker
pub fn get_pool_size(password: String) -> Vec<u64> {

    fn calculate(password: &str) -> i64 {
        assert!(password.is_ascii());
        let mut pool_score: i64 = 0;

        if Regex::new(r#"[A-Z]"#).unwrap().is_match(password) {
            pool_score += 26
        }
  
        if Regex::new(r#"[a-z]"#).unwrap().is_match(password) {
            pool_score += 26;
        }

        if Regex::new(r#"[\d]"#).unwrap().is_match(password) {
            pool_score += 10
        }
        // Updates password_characteristics struct with bool values if password contains digits
        if Regex::new(r#"[^A-Za-z0-9\s]"#).unwrap().is_match(password) {
            pool_score += 32;
        }
        pool_score
    }

    let pool_score = calculate(&password);

    let score: Vec<u64> = vec![
        pool_score.try_into().unwrap(),
        password.chars().count().try_into().unwrap(),
    ];
    score
}

pub fn calculate_entropy(pool_score: Vec<u64>) -> f64 {
    let score: f64 = pool_score[0] as f64;
    let password_length: usize = pool_score[1] as usize; // Update the type to usize

    let mut entropy: f64 = (score.powf(password_length as f64)).log2();
    entropy = round(entropy, 2);
    entropy
}

pub fn check_if_pwned(password: String) -> u64 {
    let times_discovered = pass_check(&password);
    if times_discovered > 0 {
        println!("Password has been discovered {} times.", times_discovered); 
    }
    times_discovered
}
