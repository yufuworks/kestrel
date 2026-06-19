use crate::ssh::SshRunner;
use crate::MetricsPayload;
use std::sync::atomic::AtomicBool;
use tauri::AppHandle;

// polling.rs
// ├── ポーリングループの起動（start_polling）
// ├── 1回のfetch試行ロジック（try_fetch）
// │   ├── 実行中フラグの確認
// │   ├── fetch 呼び出し
// │   └── 結果の返却（Success / Error / Delayed）
// └── 結果の型定義（PollResult）

pub enum PollingResult {
    Success(MetricsPayload),
    Error(String),
    Delayed,
}

// === 純粋ロジック ===

fn try_fetch(
    is_running: &AtomicBool,
    runner: &impl SshRunner,
    host: &str,
    user: &str,
) -> PollingResult {
    todo!()
}

// === ポーリングループ ===

pub async fn start_polling(app: AppHandle, host: String, user: String) {
    todo!()
}

#[cfg(test)]
// ユニットテスト（3ケース）**
// - 正常: fetch成功 → `metrics_updated`
// - エラー: fetch失敗 → `metrics_error`
// - 遅延: 実行中フラグあり → `metrics_delayed`

// **統合テスト（ポーリング確認）**
// - 短いインターバル（例: 2秒）で動かす
// - 約10秒待って4〜5回 fetch が呼ばれたことをスパイで確認
// - flaky リスクは余裕を持たせた範囲チェックで対処

mod tests {
    use super::*;

    // try_fetch
    #[test]
    fn try_fetch_recieve_fetch_succeess_then_returns_metrics_updated() {
        // implement test
    }
    #[test]
    fn try_fetch_recieve_fetch_error_then_returns_metrics_error() {
        // implement test
    }
    #[test]
    fn try_fetch_try_fetch_when_already_running_returns_delayed() {
        // implement test
    }
}
