use rand::prelude::*;
use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}};

mod packet;
mod card;
mod game;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(std::env::var("SOCKET_ADDRESS").unwrap()).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                
                if let Err(e) = socket.write_all(&buf).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }

    Ok(())
}
