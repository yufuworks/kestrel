mod metrics;
mod polling;
mod ssh;

use metrics::Metrics;
use serde::Serialize;

#[derive(Serialize)]
struct MetricsPayload {
    pub cpu_usage: f64,
    pub memory_total: u64,
    pub memory_used: u64,
    pub temperature: f64,
    pub disk_total: u64,
    pub disk_used: u64,
    pub uptime_secs: u64,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_metrics(host: String, user: String) -> Result<MetricsPayload, String> {
    let m = Metrics::fetch(&host, &user)?;
    Ok(MetricsPayload {
        cpu_usage: m.cpu_usage,
        memory_total: m.memory_total,
        memory_used: m.memory_used,
        temperature: m.temperature,
        disk_total: m.disk_total,
        disk_used: m.disk_used,
        uptime_secs: m.uptime_secs,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_metrics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
