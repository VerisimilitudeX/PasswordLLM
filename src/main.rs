mod tests;
use std::env;
use std::fs::File;
use regex::Regex;
mod utils;
use utils::PwnedAPI::pass_check;
use round::round;
use std::io::{BufRead, BufReader, Result, Write, Read};
use std::io;
use std::os::windows::fs::FileExt;
use std::fs;
use lnk::ShellLink;
//use tokio;
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
    let Dict_Attack: bool = password_list(password.clone());
    check_if_pwned(password);

    if Dict_Attack == true {
        println!("\n EEEEEEEK");
    }
    else {println!("falseee");}

    match entropy as i64 {
        strength if strength < 80 => println!("Password strength: Weak"),
        strength if strength < 112 => println!("Password strength: Medium"),
        strength if strength < 128 => println!("Password strength: Strong"),
        _ => println!("Password strength: Very Strong"),
    }
    
    println!("Entropy: {} bits", entropy);
    println!("\nPress ENTER to exit...");

    io::stdin().read_line(&mut String::new()).unwrap();
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

fn password_list(password: String) -> bool {
    let dir = env::current_dir().unwrap();
    let paths: fs::ReadDir = fs::read_dir(dir).unwrap();

    for file in paths {
        let mut file = file.unwrap();
        if file.file_name() == "RockYou.lnk"
         {
            let test = lnk::ShellLink::open(file.path()).unwrap();
            println!("test: {:#?}", test);
            //if file.path().to_string_lossy(). {
                println!("Found symlink!");
                let file = file.path().canonicalize().unwrap();
            
                println!("Symlink: {:?}", file);
                let file = File::open(file);
                let reader = BufReader::new(file.unwrap());

                for passwords in reader.lines() {
                    println!("jhjhj {:?}", passwords);
                    match passwords {
                        Ok(passwords) => {
                            if passwords == password {
                                println!("Password found in test!");
                                return true;
                            }
                        }
                        Err(err) => {
                            eprintln!("Error reading line: {}", err);
                        }
                }
            }
            
            }
        //}
        else {
            println!("Couldn't find symlink!");
        }
    }
    return false;
}
