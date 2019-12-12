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
    std::env::set_var("RUST_LOG","debug,tokioclient=trace");
    env_logger::init();

    let matches = clap::App::new("tokioclient")
        .version("1.0")
        .author("Filip Bucek <fbucek@invloop.cz>")
        .about("Send data to specific IP address")
        .arg(clap::Arg::with_name("DATA")
            .help("Sets the binary input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let data = matches.value_of("DATA").unwrap_or("");



    let address = "127.0.0.1:8080";
    info!("Running client on: {}", &address);

    let mut stream = TcpStream::connect(&address).await?;

    trace!("Keep alive: {:?}", stream.keepalive()?);
    trace!("Connecting to server");
    stream.write_all(data.as_bytes()).await.unwrap();

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    trace!("returned data -> size: {} data: {}", n, String::from_utf8_lossy(&buf[0..n]));
    Ok(())
}
