use tokio::net::TcpStream;
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
    std::env::set_var("RUST_LOG","trace");
    env_logger::init();

    let address = "127.0.0.1:8080";
    trace!("Running client on: {}", &address);


    let mut stream = TcpStream::connect(&address).await?;

    trace!("Keep alive: {:?}", stream.keepalive()?);
    trace!("Connecting to server");
    stream.write_all(b"00  1121|8|3|78718|").await.unwrap();
    trace!("Wrote something to stream");
    Ok(())
}
