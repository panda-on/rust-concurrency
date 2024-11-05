use std::{io, net::SocketAddr};

use anyhow::Result;
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";

    let listener = TcpListener::bind(addr).await?;
    info!("Dredis server listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, addr).await {
                warn!("error processing connection: {}", e);
            };
        });
    }
}

async fn process_redis_conn(mut stream: tokio::net::TcpStream, addr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes from {}", n, addr);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                warn!("error reading from connection: {}", e);
            }
        }
        info!("received request from {}", addr);
        println!("{}", String::from_utf8_lossy(&buf));
        stream.write_all(b"+Ok\r\n").await?;
    }
    warn!("connection {} closed", addr);
    Ok(())
}
