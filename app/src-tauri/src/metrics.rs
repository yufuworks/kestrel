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
    /// SSH 経由で Pi の各メトリクスを取得してまとめて返す
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

/// CPU stat 文字列から CPU 使用率（%）を計算して返す。
fn parse_cpu(raw: &str) -> Result<f64, String> {
    // 入力形式: <label:"cpu"> <user:数値> <nice:数値> <sys:数値> <idle:数値> ...
    // label は無視し、user 以降の数値フィールドのみ使用する。
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
    let total: u64 = fields.iter().sum(); // CPUのトータルを求める
    Ok((1.0 - idle as f64 / total as f64) * 100.0)
}

/// メモリ情報文字列からメモリ合計（バイト）を返す。
fn parse_memory_total(raw: &str) -> Result<u64, String> {
    parse_field(raw, 1)
}

/// メモリ情報文字列からメモリ使用量（バイト）を返す。
fn parse_memory_used(raw: &str) -> Result<u64, String> {
    parse_field(raw, 2)
}

/// ミリ℃の整数文字列を℃（f64）に変換して返す。
fn parse_temperature(raw: &str) -> Result<f64, String> {
    raw.trim()
        .parse::<f64>()
        .map(|t| t / 1000.0)
        .map_err(|e| e.to_string())
}

/// ディスク使用状況文字列からディスク合計（バイト）を返す。
fn parse_disk_total(raw: &str) -> Result<u64, String> {
    parse_field(raw, 1)
}

/// ディスク使用状況文字列からディスク使用量（バイト）を返す。
fn parse_disk_used(raw: &str) -> Result<u64, String> {
    parse_field(raw, 2)
}

/// 稼働時間文字列の先頭値を秒単位の整数で返す。
fn parse_uptime_secs(raw: &str) -> Result<u64, String> {
    // 入力形式: <uptime:秒数> <idle:秒数>
    raw.trim()
        .split_whitespace()
        .next()
        .ok_or("uptime parse error".to_string())?
        .parse::<f64>()
        .map(|t| t as u64)
        .map_err(|e| e.to_string())
}

/// 空白区切りの文字列から指定インデックスのフィールドを u64 として返す。
fn parse_field(raw: &str, index: usize) -> Result<u64, String> {
    raw.trim()
        .split_whitespace()
        .nth(index)
        .ok_or("parse error".to_string())?
        .parse::<u64>()
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // parse_cpu
    // 入力: "cpu <user> <nice> <sys> <idle> <iowait> ..."
    // idle = fields[3]、total = 全フィールドの合計
    // 使用率 = (1 - idle / total) * 100
    #[test]
    fn parse_cpu_valid_input_returns_usage_percent() {
        let result = parse_cpu("cpu 1234 567 890 12345 678 0 0 0 0 0").unwrap();
        let idle = 12345.0_f64;
        let total = (1234 + 567 + 890 + 12345 + 678) as f64;
        let expected = (1.0 - idle / total) * 100.0;
        assert!((result - expected).abs() < 0.01);
    }
    #[test]
    fn parse_cpu_less_than_5_fields_input_returns_error() {
        assert_eq!(
            parse_cpu("cpu 1234 567 890"),
            Err("cpu parse error".to_string())
        );
    }

    // parse_memory_total
    // 入力: "Mem <total> <used> <free> ..."（free -b | grep Mem）
    #[test]
    fn parse_memory_total_input_returns_value() {
        assert_eq!(parse_memory_total(" 100 200 300 400 "), Ok(200))
    }

    // parse_memory_used
    // 入力: "Mem <total> <used> <free> ..."（free -b | grep Mem）
    #[test]
    fn parse_memory_used_input_returns_value() {
        assert_eq!(parse_memory_used(" 100 200 300 400 "), Ok(300))
    }

    // parse_temperature
    // 入力: ミリ℃の整数文字列（例: "43500" → 43.5℃）
    #[test]
    fn parse_temperature_input_returns_value() {
        assert_eq!(parse_temperature(" 12345 "), Ok(12.345))
    }
    #[test]
    fn parse_temperature_input_returns_error() {
        assert!(parse_temperature(" invalid ").is_err())
    }

    // parse_disk_total
    // 入力: "<filesystem> <total> <used> <avail> ..."（df -B1 / | tail -1）
    #[test]
    fn parse_disk_total_input_returns_value() {
        assert_eq!(parse_disk_total(" 100 200 300 400 "), Ok(200))
    }

    // parse_disk_used
    // 入力: "<filesystem> <total> <used> <avail> ..."（df -B1 / | tail -1）
    #[test]
    fn parse_disk_used_input_returns_value() {
        assert_eq!(parse_disk_used(" 100 200 300 400 "), Ok(300))
    }

    // parse_uptime_secs
    // 入力: "<uptime秒> <idle秒>"（/proc/uptime）。先頭値のみ使用、小数切り捨て
    #[test]
    fn parse_uptime_secs_input_returns_value() {
        assert_eq!(parse_uptime_secs(" 12.3456 23.456 "), Ok(12))
    }
    #[test]
    fn parse_uptime_secs_empty_input_returns_error() {
        assert_eq!(parse_uptime_secs(""), Err("uptime parse error".to_string()))
    }
    #[test]
    fn parse_uptime_secs_invalid_input_returns_error() {
        assert!(parse_uptime_secs("invalid 0.123").is_err())
    }
    #[test]
    fn parse_uptime_secs_input_numeric_returns_value() {
        assert_eq!(parse_uptime_secs(" 345 4567 "), Ok(345))
    }

    // parse_field
    // 入力: 空白区切りの文字列。index は 0 始まり
    #[test]
    fn parse_field_valid_input_returns_value() {
        assert_eq!(parse_field("100 200 300 400", 1), Ok(200))
    }
    #[test]
    fn parse_field_out_of_range_returns_error() {
        assert_eq!(
            parse_field("100 200 300 400", 4),
            Err("parse error".to_string())
        )
    }
    #[test]
    fn parse_field_non_numberic_returns_error() {
        assert!(parse_field("100 200 str 400", 2).is_err())
    }
}
