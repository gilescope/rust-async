//! TCP client.
//!
//! First start the echo server:
//!
//! ```sh
//! $ cargo run --example tcp-echo
//! ```
//!
//! Then run the client:
//!
//! ```sh
//! $ cargo run --example tcp-client
//! ```

#[macro_use]
extern crate log;

use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,tokioclient=trace");
    env_logger::init();

    let matches = clap::App::new("Async Server")
        .version("1.0")
        .author("Filip Bucek <fbucek@invloop.cz>")
        .about("Listen on specific IP address and Sends data to specific IP address")
        .arg(
            clap::Arg::with_name("DATA")
                .help("Data to send")
                .required(true)
                .index(1),
        )
        .arg(
            clap::Arg::with_name("ip")
                .short("i")
                .help("IP address")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("port")
                .short("p")
                .help("port")
                .takes_value(true),
        )
        .get_matches();

    let ip = matches.value_of("ip").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("8080");
    let address = format!("{}:{}", ip, port);

    task::block_on(async {
        let mut stream = TcpStream::connect(&address).await?;
        info!("Connected to {}", &stream.peer_addr()?);

        let msg = "hello world";
        println!("<- {}", msg);
        stream.write_all(msg.as_bytes()).await?;

        let mut buf = vec![0u8; 1024];
        let n = stream.read(&mut buf).await?;
        println!("-> {}\n", String::from_utf8_lossy(&buf[..n]));

        Ok(())
    })
}
