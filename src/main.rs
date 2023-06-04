use std::{env, string};
extern crate regex;
use regex::Regex;

// Entire file calculates the entropy
fn main() {
    let args: Vec<String> = env::args().collect();
    let password = &args[0];
    println!("Hello, {}!", password);
}

// Pool size based on https://github.com/Kush-munot/Password-Strength-Checker
fn get_pool_size(password: String) -> i32 {
    struct PoolTable {
        digits: bool,
        low_case: bool,
        up_case: bool,
        special: bool,
    }

    impl PoolTable {
        fn calculate(&mut self, password: &String) {
            assert!(password.is_ascii());

            self.low_case = Regex::new(r#"[a-z]"#).unwrap().is_match(&password);
            self.up_case = Regex::new(r#"[A-Z]"#).unwrap().is_match(&password);
            self.digits = Regex::new(r#"\d"#).unwrap().is_match(&password); // Updates password_characteristics struct with bool values if password contains
            self.special = Regex::new(r#"[^\w\s]"#).unwrap().is_match(&password);
        }
    }

    let mut password_characteristics = PoolTable {
        digits: false,
        low_case: false,
        up_case: false,
        special: false,
    };

    password_characteristics.calculate(&password);

    let mut pool_score = 0;
    match password_characteristics {
        PoolTable { digits: true, .. } => pool_score += 10,
        PoolTable { low_case: true, .. } => pool_score += 26, // Add to scoring
        PoolTable { up_case: true, .. } => pool_score += 26,
        PoolTable { special: true, .. } => pool_score += 32,
        _ => {}
    }
    pool_score
}

fn calculate_entropy() {
    // Calculates entropy from the pool_score
}
