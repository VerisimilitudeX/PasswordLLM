// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use password_llm::Error;

#[tauri::command]
async fn evaluate_password_strength(
    password: String,
    gpu_index: usize,
    handle: tauri::AppHandle,
) -> Result<password_llm::PasswordEvaluationResult, Error> {
    let resource_path = handle
        .path_resolver()
        .resolve_resource("resources/rockyou.txt")
        .expect("Failed to resolve resource");

    let file = std::fs::File::open(&resource_path).unwrap();

    password_llm::evaluate_password_strength(password, file, Some(gpu_index)).await
}

#[tauri::command]
fn list_gpus() -> Result<Vec<password_llm::gpu::GpuStats>, password_llm::Error> {
    let mut gpus = vec![];
    for gpu in password_llm::gpu::list_gpus()? {
        gpus.push(password_llm::gpu::obtain_stats_from_device(&gpu, false)?.unwrap());
    }

    Ok(gpus)
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            evaluate_password_strength,
            list_gpus
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
