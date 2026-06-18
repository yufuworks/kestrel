import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { formatBytes, formatUptime } from "./utils";

interface Metrics {
  cpu_usage: number;
  memory_total: number;
  memory_used: number;
  temperature: number;
  disk_total: number;
  disk_used: number;
  uptime_secs: number;
}

function App() {
  const [metrics, setMetrics] = useState<Metrics | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function fetchMetrics() {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<Metrics>("get_metrics", {
        host: "localhost:2222",
        user: "yufu",
      });
      setMetrics(result);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    fetchMetrics();
    const timer = setInterval(fetchMetrics, 5000);
    return () => clearInterval(timer);
  }, []);

  return (
    <main className="container">
      <h1>Kestrel</h1>

      <button onClick={fetchMetrics} disabled={loading}>
        {loading ? "取得中..." : "メトリクス取得"}
      </button>

      {error && <p className="error">{error}</p>}

      {metrics && (
        <>
          <div className="metric">
            <div className="metric-header">
              <span>CPU使用率</span>
              <span>{metrics.cpu_usage.toFixed(1)}%</span>
            </div>
            <div className="progress-bar">
              <div
                className="progress-fill fill-cpu"
                style={{ width: `${metrics.cpu_usage}%` }}
              />
            </div>
          </div>

          <div className="metric">
            <div className="metric-header">
              <span>メモリ</span>
              <span>
                {((metrics.memory_used / metrics.memory_total) * 100).toFixed(
                  1,
                )}
                %
              </span>
            </div>
            <div className="progress-bar">
              <div
                className="progress-fill fill-memory"
                style={{
                  width: `${(metrics.memory_used / metrics.memory_total) * 100}%`,
                }}
              />
            </div>
            <div className="metric-sub">
              {formatBytes(metrics.memory_used)} /{" "}
              {formatBytes(metrics.memory_total)}
            </div>
          </div>

          <div className="metric">
            <div className="metric-header">
              <span>ディスク</span>
              <span>
                {((metrics.disk_used / metrics.disk_total) * 100).toFixed(1)}%
              </span>
            </div>
            <div className="progress-bar">
              <div
                className="progress-fill fill-disk"
                style={{
                  width: `${(metrics.disk_used / metrics.disk_total) * 100}%`,
                }}
              />
            </div>
            <div className="metric-sub">
              {formatBytes(metrics.disk_used)} /{" "}
              {formatBytes(metrics.disk_total)}
            </div>
          </div>

          <div className="metric">
            <div className="metric-header">
              <span>温度</span>
              <span>{metrics.temperature.toFixed(1)}°C</span>
            </div>
            <div className="progress-bar">
              <div
                className="progress-fill fill-temp"
                style={{ width: `${Math.min(metrics.temperature, 100)}%` }}
              />
            </div>
          </div>

          <div className="metric">
            <div className="metric-header">
              <span>稼働時間</span>
              <span>{formatUptime(metrics.uptime_secs)}</span>
            </div>
          </div>
        </>
      )}
    </main>
  );
}

export default App;
