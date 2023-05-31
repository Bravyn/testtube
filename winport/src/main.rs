#![allow(unused)]

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            if let Ok(n) = socket.read(&mut buffer).await {
                if let Ok(request) = String::from_utf8(buffer[..n].to_vec()) {
                    println!("Received node registration: {}", request);
                    // Process the node registration and maintain a list of known nodes
                }
            }
            let response = "Node registration successful";
            if let Err(e) = socket.write_all(response.as_bytes()).await {
                eprintln!("Failed to send response: {}", e);
            }
        });
    }
}
