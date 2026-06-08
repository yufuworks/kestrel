use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

/// SSH セッションを確立してコマンドを実行し、標準出力を返す
pub fn run_command(host: &str, user: &str, command: &str) -> Result<String, String> {
    let tcp = TcpStream::connect(host).map_err(|e| e.to_string())?;
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
