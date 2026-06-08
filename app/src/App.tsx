import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

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
        host: "192.168.10.171:22",
        user: "yufu",
      });
      setMetrics(result);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  function formatBytes(bytes: number): string {
    return (bytes / 1024 / 1024 / 1024).toFixed(1) + " GB";
  }

  function formatUptime(secs: number): string {
    const days = Math.floor(secs / 86400);
    const hours = Math.floor((secs % 86400) / 3600);
    const mins = Math.floor((secs % 3600) / 60);
    return `${days}日 ${hours}時間 ${mins}分`;
  }

  return (
    <main className="container">
      <h1>Kestrel</h1>

      <button onClick={fetchMetrics} disabled={loading}>
        {loading ? "取得中..." : "メトリクス取得"}
      </button>

      {error && <p style={{ color: "red" }}>{error}</p>}

      {metrics && (
        <table>
          <tbody>
            <tr>
              <td>CPU使用率</td>
              <td>{metrics.cpu_usage.toFixed(1)}%</td>
            </tr>
            <tr>
              <td>メモリ</td>
              <td>
                {formatBytes(metrics.memory_used)} /{" "}
                {formatBytes(metrics.memory_total)}
              </td>
            </tr>
            <tr>
              <td>温度</td>
              <td>{metrics.temperature.toFixed(1)}°C</td>
            </tr>
            <tr>
              <td>ディスク</td>
              <td>
                {formatBytes(metrics.disk_used)} /{" "}
                {formatBytes(metrics.disk_total)}
              </td>
            </tr>
            <tr>
              <td>稼働時間</td>
              <td>{formatUptime(metrics.uptime_secs)}</td>
            </tr>
          </tbody>
        </table>
      )}
    </main>
  );
}

export default App;
