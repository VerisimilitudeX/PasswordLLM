#[allow(unused_must_use)]
#[allow(non_snake_case)]

mod tests;
mod utils;
mod gpu;
mod cal_time;

use utils::pwned_api::pass_check;
use gpu::gpu::obtainGPU;
use cal_time::cal_time::cal_time;

use std::env;
use std::fs::File;
use std::path::Path;
use round::round;
use std::fs;
use std::io::{BufRead, BufReader};
use std::io;
use tokio::{time::timeout, time::Duration};
use parselnk::Lnk;
use regex::Regex;

#[tokio::main]
pub async fn main() {
    let args: Vec<String> = env::args().collect(); // stores arguments in vector
    let password;
    let mut workflow = false;
    
    if args.len() > 1 {
        password = args[1].clone();
        workflow = args.len() == 2;

    } else {
        password = rpassword::prompt_password("Password: ").unwrap(); // prompts for passwords if no arguments are given
    }

    let pool_size = get_pool_size(password.clone());
    let entropy = calculate_entropy(pool_size); // calls functions
    let alphabet_match = regex_match(password.clone());
    let mut statistics = obtainGPU(workflow); // todo

    if statistics.as_mut().unwrap().is_some() {
        cal_time(statistics.unwrap().unwrap(), entropy);
    }
    check_if_pwned(password.clone()).await;

    if !alphabet_match { // just a big match statement to check to see if it should call a function
        let rockyou = timeout(Duration::from_secs(60), password_list(password.clone())).await;
        match rockyou{
            Ok(x) => {
                match x {
                    Ok(y) => {
                        if y {
                            println!("Bruteforce Diagnostic: Your password can be easily cracked due to dictonary-based bruteforcing attacks. Change it now!"); 
                        }
                        else {
                            println!("\nYour password is not in the RockYou.txt password list. Good job!");
                        }
                    }
                    Err(p) => {
                        println!("RockYou not found. {:?}", p);
                    }
                }

            }
            Err(k) => {
                println!("\nSomething went wrong, timed out, elapsed time: {:?}", k.to_string());
            }
        }
    }
    else {
        println!("\n Your password is incredibly easy to bruteforce, it is less than 3 characters are only contacts alphabetic characters.")
    }

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

        if Regex::new(r"[\d]").unwrap().is_match(password) {
            pool_score += 10
        }
        // Updates password_characteristics struct with bool values if password contains digits
        if Regex::new(r"[^A-Za-z0-9\s]").unwrap().is_match(password) {
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

pub async fn check_if_pwned(password: String) -> u64 {
    let times_discovered = pass_check(&password).await;
    if times_discovered > 0 {
        println!("Password has been discovered {} times.", times_discovered); 
    }
    times_discovered
}

async fn password_list(password: String) -> Result<bool, ()> {
    let dir = env::current_dir().unwrap();
    let directory: fs::ReadDir = fs::read_dir(dir).unwrap();

    for file in directory {
        let file = file.unwrap();
        if file.file_name() == "RockYou.lnk"
         {
            let file_lnk = file.path();
            let file_lnk: Result<Lnk, _> = Lnk::try_from(file_lnk.as_path());
            let file_lnk: Option<String> = file_lnk.unwrap().link_info.local_base_path;
            
            let file_lnk: String = {
                match file_lnk {
                    Some(x) => x,
                    None => {
                        panic!("Your shortcut link exists, but it's pointing to a unaccessable file.");
                    }
                }
            };
            let file = Path::new(&file_lnk);
            if file.exists() && file.is_file() {
                println!("Found RockYou.txt located at {}", file.to_string_lossy());
                tokio::time::sleep(Duration::from_secs(5)).await;
                
                let file = File::open(file);
                let file = file.unwrap();
                let reader = BufReader::new(file);

                let mut line = 0;
                let mut counter = 0;
                for passwords in reader.lines() {
                    counter+=1;
                    line+=1;
                    match passwords {
                        Ok(passwords) => {
                            if passwords == password {
                                return Ok(true);
                            }
                            else if counter % 4000 == 0 { 
                                println!("Searching...");
                                counter = 0;
                            }
                        }
                        Err(err) => {
                            println!("Error reading line: {:?}, the error is \"{}\"", line, err);
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    }
                    if line >= 2459760 { // if it's at the end of the file, stop the search and return password not found
                        return Ok(false);
                    }
                }
            }
            else {
                println!("Error 1, cannot find correct source file: {:?}, may not exist or be a file. {:?}", file.to_string_lossy(), file.file_name());
                return Err(());
            }
            println!("Error 3");
            println!("{:?}", file.file_name());
        }
    }
    println!("File not found, skipping!");
    Err(())
}

fn regex_match(password: String) -> bool {
    let regex = Regex::new(r"[a-zA-Z]").unwrap();

    if password.len() < 3 && regex.is_match(&password) {
        return true;
    }
    false
}
