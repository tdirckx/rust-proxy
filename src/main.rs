//#![windows_subsystem = "windows"]
mod proxy;

use proxy::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let server = Server::new().await?;
    server.run().await?;
    Ok(())
}

//terminal: curl --socks5 username:password@127.0.0.1:55260 https://dumpert.nl
//example: https://github.com/m1nuzz/RustySocks/blob/master/src/main.rs