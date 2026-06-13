use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::sync::Once;
use std::time::Duration;

static LOG_ONCE: Once = Once::new();

/// SSH セッションを確立してコマンドを実行し、標準出力を返す
pub fn run_command(host: &str, user: &str, command: &str) -> Result<String, String> {
    let addr = host
        .to_socket_addrs()
        .map_err(|e| e.to_string())?
        .find(|a| a.is_ipv4())
        .ok_or("IPv4アドレスが見つかりません".to_string())?;
    LOG_ONCE.call_once(|| {
        eprintln!("[SSH] 接続先: {} (ユーザー: {})", addr, user);
    });
    let tcp =
        TcpStream::connect_timeout(&addr, Duration::from_secs(3)).map_err(|e| e.to_string())?;
    LOG_ONCE.call_once(|| {
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
