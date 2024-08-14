use anyhow::Result;
use tokio::{io::{self, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Dredis: listening on: {}", addr);

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from: {}", raddr);
        
        tokio::spawn(async move {
            if let Err(e) = process_redis_connection(stream).await {
                warn!("Error process connection with: addr={}, e={}", raddr, e);
            }
        });
    }
}

const BUF_SIZE: usize = 4096;

async fn process_redis_connection(mut stream: TcpStream) -> Result<()> {
    loop {
        stream.readable().await?;
        
        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);

                let line = String::from_utf8_lossy(&buf);
                info!("{:?}", line);

                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into())
            }
        }
    }

    Ok(())
}
