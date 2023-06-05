use std::{env};
extern crate regex;
use regex::Regex;

// Entire file calculates the entropy
fn main() {
    let args: Vec<String> = env::args().collect();
    let password = &args[1];
  
    let PoolSize = GetPoolSize(password.to_string());
    let Entropy = calculate_entropy(PoolSize.clone());
    println!("Entropy: {}", Entropy);
    println!("Pool Size: {:?}", PoolSize[0].clone())
}

// Pool size based on https://github.com/Kush-munot/Password-Strength-Checker
pub fn GetPoolSize(password: String) -> Vec<u64> {
    pub struct PoolTable {
        digits: bool,
        low_case: bool,
        up_case: bool,
        special: bool,
    }

    fn calculate(password: &String) -> PoolTable {
        //assert!(password.is_ascii());

        let mut password_characteristics = PoolTable {
            digits: false,
            low_case: false,
            up_case: false,
            special: false,
        };

        password_characteristics.low_case = Regex::new(r#"[a-z]"#).unwrap().is_match(&password);
        password_characteristics.up_case = Regex::new(r#"[A-Z]"#).unwrap().is_match(&password);
        password_characteristics.digits = Regex::new(r#"\d"#).unwrap().is_match(&password); // Updates password_characteristics struct with bool values if password contains
        password_characteristics.special = Regex::new(r#"[^\w\s]"#).unwrap().is_match(&password);
        password_characteristics
    
    }

    let pass_char = calculate(&password);

    let mut pool_score: i32 = 0;
    match pass_char {
        PoolTable { digits: true, .. } => pool_score += 10,
        PoolTable { low_case: true, .. } => pool_score += 26, // Add to scoring
        PoolTable { up_case: true, .. } => pool_score += 26,
        PoolTable { special: true, .. } => pool_score += 32,
        _ => {}
    }
    let score: Vec<u64> = vec![pool_score.try_into().unwrap(), password.chars().count().try_into().unwrap()];
    score

}

pub fn calculate_entropy(pool_score: Vec<u64>) -> f64 {
    // Calculates entropy from the pool_score
    let score: u64 = pool_score[0];
    let password_length: u64 = pool_score[1];

    let entropy: f64 = (score.pow(password_length as u32) as f64).log2();
    entropy.round()
}
