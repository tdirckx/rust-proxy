use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use std::net::SocketAddr;
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
     * Handle an incoming connection.
     */
    pub async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {

        let mut connection = Connection::new(stream);
        let buffer = connection.read(258).await?;

        const SOCKS5 : u8 = General::Socks5 as u8;
        const SOCKS4 : u8 = General::Socks4 as u8;

        match buffer[0] {
            SOCKS5 => Socks5::new(connection, buffer).run().await,
            SOCKS4 => Err("SOCKS4 not supported yet".into()),
            _ => Err("Unknown protocol".into())
        }        
    }
}