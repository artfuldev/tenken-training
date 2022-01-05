use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let response = "HTTP/1.1 200 OK\r\n";
            // Copy the data back to socket
            socket.write_all(response.as_bytes()).await;
            return;
        });
    }
}
