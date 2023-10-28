use std::{path::PathBuf, io::Write};

use dialoguer::{Select, theme::ColorfulTheme};
use password_llm::PasswordStrength;
use thousands::Separable;
use futures_util::stream::StreamExt;
use lazy_static::lazy_static;
use std::io;

#[cfg(target_os = "windows")]
lazy_static! {
	static ref ROCKYOU_PATH: PathBuf = PathBuf::from("C:\\ProgramData\\PasswordLLM");
}
#[cfg(target_os = "linux")]
lazy_static! {
	static ref ROCKYOU_PATH: PathBuf = PathBuf::from("/var/cache/PasswordLLM");
}
// TODO: macos

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	download_rockyou().await?;

	let password = dialoguer::Password::default().with_prompt("Enter your password").interact().unwrap();

	let gpus = password_llm::gpu::list_gpus();
	let mut gpu_index = None;
	if let Ok(gpus) = gpus {
		let gpu_names = gpus.iter()
			.map(|gpu| gpu.name().unwrap())
			.collect::<Vec<_>>();

		gpu_index = Some(if gpu_names.len() > 1 {
			Select::with_theme(&ColorfulTheme::default())
				.with_prompt(format!("You have multiple ({}) GPUs in your system. Please select one.", gpus.len()))
				.default(0)
				.items(&gpu_names[..])
				.interact()?
		} else {
			0
		});

		let gpu = password_llm::gpu::obtain_stats_from_index(gpu_index.as_ref().unwrap().to_owned(), false).unwrap().unwrap();
		println!("GPU Info:\n\tClock speed: {} MHz\n\tCUDA cores: {}, equivalent to {} streaming multiprocessors\n\tFP32 GFLOPS: {}\n\tFP64 GFLOPS: {}",
			gpu.clock, gpu.cuda_cores, gpu.smp, gpu.gflop32(), gpu.gflop64());
	} else {
		eprintln!("Failed to query available GPUs: {}\nAre your drivers installed correctly?", gpus.err().unwrap());
	}

	println!("Evaluating password...");

	let evaluation_result = password_llm::evaluate_password_strength(password, std::fs::File::open(ROCKYOU_PATH.join("rockyou.txt"))?, gpu_index).await?;
	println!("\nPassword statistics:\n\tEntropy: {} bits", evaluation_result.entropy);

	if let Some(times_pwned) = evaluation_result.times_pwned {
		if times_pwned > 0 {
			println!("\tYour password has been pwned {} times!", times_pwned);
		} else {
			println!("\tYour password is not present in any database leaks.");
		}
	} else {
		eprint!("\tFailed to check pwned database.")
	}

	match evaluation_result.strength {
		PasswordStrength::ProneToBruteforceAttack => println!("\tYour password is incredibly easy to bruteforce!"),
		PasswordStrength::ProneToDictionaryAttack => println!("\tYour password is vulnerable to dictionary-based bruteforce attacks!"),
		other => println!("\tPassword strength: {:?}", other),
	}


	if gpu_index.is_some() {
		let total_seconds = evaluation_result.approximate_time_to_crack_secs.unwrap();
		let seconds = total_seconds % 60;
		let minutes = (total_seconds / 60) % 60;
		let hours = (total_seconds / (60 * 60)) % 24;
		let days = (total_seconds / (60 * 60 * 24)) % 7;
		let weeks = total_seconds / (60 * 60 * 24 * 7);

		println!("\tPossible combinations: {}\n\tTime to crack: Approximately {} weeks {} days {:02} hours {:02} minutes and {:02} seconds", 
			evaluation_result.possible_combinations.unwrap().separate_with_commas(), weeks.separate_with_commas(), days, hours, minutes, seconds
		);
	}
    println!("\nPress ENTER to exit...");
    io::stdin().read_line(&mut String::new()).unwrap();

	Ok(())
}

async fn download_rockyou() -> Result<(), Box<dyn std::error::Error>> {
	let _ = std::fs::create_dir(ROCKYOU_PATH.as_path());

	let check_file = std::fs::File::open(ROCKYOU_PATH.join("rockyou_check"));
	if check_file.is_ok() {
		return Ok(());
	}

	println!("{} is either missing or corrupted", ROCKYOU_PATH.join("rockyou.txt").into_os_string().into_string().unwrap());
	
	let response = reqwest::get("https://github.com/brannondorsey/naive-hashcat/releases/download/data/rockyou.txt")
		.await?;
	
	let file_size = response.content_length().unwrap();
	let mut stream = response.bytes_stream();
	let mut writer = std::io::BufWriter::new(std::fs::File::create(ROCKYOU_PATH.join("rockyou.txt"))?);
	let mut written = 0;
	let mut lock = std::io::stdout().lock();
	while let Some(bytes) = stream.next().await {
		written += std::io::copy(&mut bytes?.as_ref(), &mut writer)?;

		write!(lock, "Downloading rockyou.txt: {}/{} bytes\r", written, file_size)?;
	}
	writer.flush()?;
	println!();

	std::fs::File::create(ROCKYOU_PATH.join("rockyou_check"))?;
	Ok(())
}