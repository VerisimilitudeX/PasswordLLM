#[allow(unused_must_use)]
#[allow(non_snake_case)]

mod tests;
mod utils;
pub mod gpu;
mod cal_time;

use serde::Serialize;
use utils::pwned_api;
use gpu::obtain_stats_from_index;
use cal_time::cal_time;

use std::env;
use std::fs::File;
use round::round;
use std::io::{BufRead, BufReader};
use tokio::{time::timeout, time::Duration};
use regex::Regex;

#[derive(Serialize, Debug)]
pub enum PasswordStrength {
	ProneToDictionaryAttack,
	ProneToBruteforceAttack,
	Weak,
	Medium,
	Strong,
	VeryStrong,
}

#[derive(Serialize)]
pub struct PasswordEvaluationResult {
	pub strength: PasswordStrength,
	pub entropy: f64,
	// None if no internet
	pub times_pwned: Option<u64>,
	// None if OpenCL errors are encountered
	pub possible_combinations: Option<u64>,
	// None if OpenCL errors are encountered
	pub approximate_time_to_crack_secs: Option<u64>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("RockYou.txt not found.")]
	RockYouNotFound,
	#[error("Timed out.")]
	Timeout,
	#[error(transparent)]
	OpenCLError(#[from] opencl3::error_codes::ClError),
	#[error(transparent)]
	PwnedApiError(#[from] pwned::errors::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub async fn evaluate_password_strength(password: String, dictionary: File, gpu_index: Option<usize>) -> Result<PasswordEvaluationResult, Error> {
    let args: Vec<String> = env::args().collect(); // stores arguments in vector
    let mut workflow = false;
    
    if args.len() > 1 {
        workflow = args.len() == 2;
    }

    let pool_size = get_pool_size(password.clone());
    let entropy = calculate_entropy(pool_size); // calls functions
    let alphabet_match = regex_match(password.clone());
	// TODO
    let statistics = if let Some(gpu_index) = gpu_index {
		obtain_stats_from_index(gpu_index, workflow)?
	} else {
		None
	};

	let mut time = None;
    if let Some(statistics) = statistics {
		time = Some(cal_time(statistics, entropy));
    }

    let pwned = tokio::spawn({
		let password = password.clone();
		async move { pwned_api::pass_check(&password).await }
	});

	let mut pass_strength = match entropy as i64 {
        strength if strength < 80 => PasswordStrength::Weak,
        strength if strength < 112 => PasswordStrength::Medium,
        strength if strength < 128 => PasswordStrength::Strong,
        _ => PasswordStrength::VeryStrong,
    };

    if !alphabet_match { // just a big match statement to check to see if it should call a function
        let rockyou = timeout(Duration::from_secs(60), check_dictionary(password.clone(), dictionary)).await;
        match rockyou {
            Ok(in_rockyou) => {
                if in_rockyou {
					pass_strength = PasswordStrength::ProneToDictionaryAttack;
            	}
            },
            Err(_) => return Err(Error::Timeout),
        }
    }
    else {
        pass_strength = PasswordStrength::ProneToBruteforceAttack;
    }


	let time = match time {
		Some((a, b)) => (Some(a), Some(b)),
		None => (None, None)
	};
	Ok(PasswordEvaluationResult {
		strength: pass_strength,
		entropy,
		times_pwned: pwned.await.unwrap().ok(),
		possible_combinations: time.0,
		approximate_time_to_crack_secs: time.1,
	})
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

async fn check_dictionary(password: String, dictionary: File) -> bool {
    let reader = BufReader::new(dictionary);

    let mut errors = 0;
    for passwords in reader.lines() {
        match passwords {
            Ok(passwords) => {
                if passwords == password {
                    return true;
                }
            }
            Err(_) => {
                errors+=1;
            }
        }
    }

	println!("Failed to read {} lines in rockyou.txt", errors);

	false
}

fn regex_match(password: String) -> bool {
    let regex = Regex::new(r"[a-zA-Z]").unwrap();

    if password.len() < 3 && regex.is_match(&password) {
        return true;
    }
    false
}
