use shared_dev_tools::process::{ProcessId, ProcessKind};
use wallet::config::Environment;

#[tauri::command(rename_all = "snake_case")]
pub fn start_process(kind: ProcessKind, env: Environment) -> std::result::Result<ProcessId, String> {

    println!("TODO - start process for {:?} on {}", kind, env);

    Ok(ProcessId(0))
}

#[tauri::command(rename_all = "snake_case")]
pub fn kill_process(process_id: ProcessId) -> std::result::Result<(), String> {

    println!("TODO - kill process {}", process_id);

    Ok(())
}