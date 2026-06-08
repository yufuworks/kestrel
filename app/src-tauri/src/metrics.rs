use crate::ssh;

/// Pi から取得したメトリクス
pub struct Metrics {
    pub cpu_usage: f64,
    pub memory_total: u64,
    pub memory_used: u64,
    pub temperature: f64,
    pub disk_total: u64,
    pub disk_used: u64,
    pub uptime_secs: u64,
}

impl Metrics {
    pub fn fetch(host: &str, user: &str) -> Result<Metrics, String> {
        let cpu_raw = ssh::run_command(host, user, "cat /proc/stat | head -1")?;
        let mem_raw = ssh::run_command(host, user, "free -b | grep Mem")?;
        let temp_raw = ssh::run_command(host, user, "cat /sys/class/thermal/thermal_zone0/temp")?;
        let disk_raw = ssh::run_command(host, user, "df -B1 / | tail -1")?;
        let uptime_raw = ssh::run_command(host, user, "cat /proc/uptime")?;

        Ok(Metrics {
            cpu_usage: parse_cpu(&cpu_raw)?,
            memory_total: parse_memory_total(&mem_raw)?,
            memory_used: parse_memory_used(&mem_raw)?,
            temperature: parse_temperature(&temp_raw)?,
            disk_total: parse_disk_total(&disk_raw)?,
            disk_used: parse_disk_used(&disk_raw)?,
            uptime_secs: parse_uptime_secs(&uptime_raw)?,
        })
    }
}

fn parse_cpu(raw: &str) -> Result<f64, String> {
    let fields: Vec<u64> = raw
        .trim()
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    if fields.len() < 4 {
        return Err("cpu parse error".to_string());
    }

    let idle = fields[3];
    let total: u64 = fields.iter().sum();
    Ok((1.0 - idle as f64 / total as f64) * 100.0)
}

fn parse_memory_total(raw: &str) -> Result<u64, String> {
    parse_field(raw, 1)
}

fn parse_memory_used(raw: &str) -> Result<u64, String> {
    parse_field(raw, 2)
}

fn parse_temperature(raw: &str) -> Result<f64, String> {
    raw.trim()
        .parse::<f64>()
        .map(|t| t / 1000.0)
        .map_err(|e| e.to_string())
}

fn parse_disk_total(raw: &str) -> Result<u64, String> {
    parse_field(raw, 1)
}

fn parse_disk_used(raw: &str) -> Result<u64, String> {
    parse_field(raw, 2)
}

fn parse_uptime_secs(raw: &str) -> Result<u64, String> {
    raw.trim()
        .split_whitespace()
        .next()
        .ok_or("uptime parse error".to_string())?
        .parse::<f64>()
        .map(|t| t as u64)
        .map_err(|e| e.to_string())
}

fn parse_field(raw: &str, index: usize) -> Result<u64, String> {
    raw.trim()
        .split_whitespace()
        .nth(index)
        .ok_or("parse error".to_string())?
        .parse::<u64>()
        .map_err(|e| e.to_string())
}
