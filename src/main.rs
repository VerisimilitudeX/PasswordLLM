use std::{f32::DIGITS, string};
use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let password = &args[1];
    println!("Hello, {password}!");

}

fn get_pool_size(password: String) -> i32{
    struct PoolTable{
        Digits: bool,
        Low_case: bool,
        Up_case: bool,
        Special: bool,
    }

    let mut password_characteristics = PoolTable {
        Digits : false,
        Low_case : false,
        Up_case : false, 
        Special : false
    };

    trait PoolSize {
        fn calculate(password: &String, password_characteristics: &PoolTable) -> &PoolTable; 
    }

    impl PoolSize for PoolTable {
        fn calculate(password: &String, password_characteristics: &PoolTable) -> &PoolTable{
            assert!(password.is_ascii());

            password_characteristics.Low_case = Regex::new(r#"[a-z]"#).unwrap().is_match(&password);
            password_characteristics.Up_case = Regex::new(r#"[A-Z]"#).unwrap().is_match(&password);
            password_characteristics.Digits = Regex::new(r#"\d"#).unwrap().is_match(&password);
            password_characteristics.Special = Regex::new(r#"[^\w\s]"#).unwrap().is_match(&password);
            return password_characteristics;
        }
    }
    
    let password_characteristics = PoolSize::calculate(&password, &password_characteristics);
    let mut pool_score = 0;
    match &password_characteristics {
        PoolTable { Digits: true, .. } => pool_score += 10,
        PoolTable { Low_case: true, .. } => pool_score += 26,
        PoolTable { Up_case: true, .. } => pool_score += 26,
        PoolTable { Special: true, .. } => pool_score += 32,
        _ => {}
    }
    pool_score
    
}


fn calculate_entropy() {}

