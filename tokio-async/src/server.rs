use tokio::net::TcpListener;
use tokio::prelude::*;

#[macro_use]
extern crate log;

/// Bind to IP and port and starts listening
/// 
/// It read socket and return same data as response
/// 
/// # Bash test
/// 
/// `nc localhost 8080` -> it will open connection -> possible to sand any data.alloc
/// 
///  1. When connection is estabilished -> "Connection estabilished" is printed
/// 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG","debug,tokioserver=trace");
    env_logger::init();
    let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

    match listener.accept().await {
        Ok((_socket, addr)) => println!("new client: {:?}", addr),
        Err(e) => println!("couldn't get client: {:?}", e),
    }

    loop {
        let (mut socket, addr) = listener.accept().await?;

        info!("Connection estabilished with: {:?}", addr);

        // Spawn async in ThreadPool ( default executor )
        tokio::spawn(async move {
            // In a loop, read data from the socket and write the data back.
            loop {
                let mut buf = [0; 1024];
                let bytes_read = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(bytes_read) if bytes_read == 0 => return,
                    Ok(bytes_read) => bytes_read,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                info!("data: {}", String::from_utf8_lossy(&buf));


                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..bytes_read]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}