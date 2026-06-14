use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::sync::Once;
use std::time::Duration;

static SSH_LOG_ONCE: Once = Once::new();
static TCP_LOG_ONCE: Once = Once::new();

/// SSH コマンド実行の抽象インターフェース
pub trait SshRunner {
    fn run(&self, host: &str, user: &str, command: &str) -> Result<String, String>;
}

/// 実際の SSH 接続を使う実装
pub struct RealSsh;

impl SshRunner for RealSsh {
    fn run(&self, host: &str, user: &str, command: &str) -> Result<String, String> {
        run_command(host, user, command)
    }
}

/// SSH セッションを確立してコマンドを実行し、標準出力を返す
pub fn run_command(host: &str, user: &str, command: &str) -> Result<String, String> {
    let addr = host
        .to_socket_addrs()
        .map_err(|e| e.to_string())?
        .find(|a| a.is_ipv4())
        .ok_or_else(|| format!("IPv4アドレスが見つかりません: host={} user={}", host, user))?;
    SSH_LOG_ONCE.call_once(|| {
        eprintln!("[SSH] 接続先: {} (ユーザー: {})", addr, user);
    });
    let tcp =
        TcpStream::connect_timeout(&addr, Duration::from_secs(3)).map_err(|e| e.to_string())?;
    TCP_LOG_ONCE.call_once(|| {
        eprintln!("[TCP] 接続先: {} (ユーザー: {})", addr, user);
    });
    let mut sess = Session::new().map_err(|e| e.to_string())?;
    sess.set_tcp_stream(tcp);
    sess.handshake().map_err(|e| e.to_string())?;
    sess.userauth_agent(user).map_err(|e| e.to_string())?;

    let mut channel = sess.channel_session().map_err(|e| e.to_string())?;
    channel.exec(command).map_err(|e| e.to_string())?;

    let mut output = String::new();
    channel
        .read_to_string(&mut output)
        .map_err(|e| e.to_string())?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    // run_command
    #[test]
    fn run_command_ipv6_only_host_returns_ipv4_not_found_error() {
        let host = "[::1]:22";
        let user = "user";
        let command = "cmd";
        let expected_err = format!("IPv4アドレスが見つかりません: host={} user={}", host, user);
        assert_eq!(run_command(host, user, command), Err(expected_err));
    }
    #[test]
    fn run_command_invalid_host_format_returns_error() {
        let host = "invalid-host-no-port";
        let user = "user";
        let command = "cmd";
        assert!(run_command(host, user, command).is_err());
    }
    #[test]
    fn run_command_unreachable_host_returns_error() {
        let host = "unreachable-host";
        let user = "user";
        let command = "cmd";
        assert!(run_command(host, user, command).is_err());
    }
}
