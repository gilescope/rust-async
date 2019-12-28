//! TCP echo server.
//!
//! To send messages, do:
//!
//! ```sh
//! $ nc localhost 8080
//! ```

#[macro_use]
extern crate log;

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

async fn process(stream: TcpStream) -> io::Result<()> {
    println!("Accepted from: {}", stream.peer_addr()?);

    let (reader, writer) = &mut (&stream, &stream);

    let mut buf = vec![0u8; 1024];
    let bytes_read = reader.read(&mut buf).await?;

    trace!("data:{}", String::from_utf8_lossy(&buf));

    // println!(reader);
    writer.write_all(&buf[0..bytes_read]).await?;
    //io::copy(&buf[0..bytes_read], writer).await?;

    Ok(())
}

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,asyncserver=trace");
    env_logger::init();

    let matches = clap::App::new("Tokio Client")
        .version("1.0")
        .author("Filip Bucek <fbucek@invloop.cz>")
        .about("Sends data to specific IP address")
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
        let listener = TcpListener::bind(&address).await?;
        info!("Listening on {}", listener.local_addr()?);

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            task::spawn(async {
                process(stream).await.unwrap();
            });
        }
        Ok(())
    })
}
