//#![windows_subsystem = "windows"]
mod proxy;

use proxy::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let server = Server::new().await?;
    server.run().await?;
    Ok(())
}


/*
## Testing commands:
- curl --socks5  127.0.0.1:55260 https://dumpert.nl
- curl --socks5-hostname username:password@127.0.0.1:55260 https://dumpert.nl
- curl --socks5 username:password@127.0.0.1:55260 https://dumpert.nl
- curl --socks4 username:password@127.0.0.1:55260 https://dumpert.nl
- curl --proxy username:password@127.0.0.1:55260 https://dumpert.nl
*/









