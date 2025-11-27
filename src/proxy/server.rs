use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use std::net::SocketAddr;
use env_logger;

use crate::proxy::Connection;
use crate::proxy::socks5::{Socks5, General};

/**
 * Represents the proxy server.
 */
pub struct Server {
    listener: TcpListener,
}

/**
 * Implementation of Server methods.
 */
impl Server {

    /**
     * Create a new Server instance.
     */
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        env_logger::init();

        let listener = TcpListener::bind("127.0.0.1:55260").await?;
        
        let addr: SocketAddr = listener.local_addr()?;
        println!("Listening on {}", addr);

        Ok(Self { 
            listener,
        })
    }


    /**
     * Run the server to accept incoming connections.
     */
    pub async fn run(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        loop {
            let (stream, addr) = self.listener.accept().await?;
            println!("Accepted connection from {}", addr);

            tokio::spawn(async move {
                 if let Err(e) = Server::handle_connection(stream).await {
                    println!("Error: {e}");
                }
            });            
        }
    }

    /**
     * Check if the version is SOCKS5
     */
    pub fn is_socks5(version: u8) -> bool {
        version == General::Socks5.as_u8()
    }


    /**
     *  Check if the version is SOCKS4
     */
    pub fn is_socks4(version: u8) -> bool {
        version == General::Socks4.as_u8()
    }


    /**
     * Handle an incoming connection.
     */
    pub async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {

        let mut connection = Connection::new(stream);
        let buffer = connection.read(262).await?;

        if Server::is_socks5(buffer[0]) {
            Socks5::new(connection, buffer).run().await?;

            return Ok(());
        } else if Server::is_socks4(buffer[0]) {
            return Err("SOCKS4 not supported yet".into());
        }

        Err("Unknown protocol".into())
    }
}