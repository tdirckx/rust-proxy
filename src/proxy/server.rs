use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use std::net::SocketAddr;
use log::{error, debug, info};
use env_logger;

use crate::proxy::Client;
use crate::proxy::socks5::{Socks5, General};

/**
 * Represents the proxy server.
 */
pub struct Server {
    listener: TcpListener,
}


/**
 *  Implementation of Server methods.
 */
impl Server {

    /**
     *  Create a new Server instance.
     */
    pub async fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        env_logger::init();

        let listener = TcpListener::bind("127.0.0.1:55260").await?;
        
        let addr: SocketAddr = listener.local_addr()?;
        println!("Listening on {}", addr);

        Ok(Server { listener })
    }


    /**
     *  Run the server to accept incoming connections.
     */
    pub async fn run(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        loop {
            let (mut client_socket, addr) = self.listener.accept().await?;
            println!("Accepted connection from {}", addr);

            tokio::spawn(async move {
                println!("Handling connection from {}", addr);

                if let Err(e) = Self::handle_client(client_socket).await {
                    println!("Error handling client {}: {}", addr, e);
                }              
            });            
        }
    }


    /**
     *  Handle an individual client connection.
     */
    pub async fn handle_client(mut client: TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {

        let mut client = Client::new(client).await?;

        if client.is_socks5() {
            println!("Received SOCKS5 handshake");
            Socks5::new(client).run().await?;

            return Ok(());
        } else if client.is_socks4() {
            return Err("SOCKS4 not supported yet".into());
        }

        client.write_chosen_authentication(General::UnknownVersionIndentifier.as_u8()).await?;
        return Err("Unknown protocol".into());

        // let mut buffer = [0u8; 262];
        // let n = client.read(&mut buffer).await?;
        // if n == 0  {
        //     println!("Connection close");
        // }

        // println!("Client read {} bytes", n);

        // if buffer[0] == 0x05 {
        //     println!("Received SOCKS5 handshake");
        // } else if buffer[0] == 0x04 {
        //     Err("SOCKS4 not supported yet")?;
        // } else {
        //     println!("Received unknown protocol");
        // }

        // let expectedMethod: u8 = 0x02; // No authentication

        // let clientMethod: u8 = buffer[1];

        // let mut hasAuthentication: bool = false;

        // for(i, &method) in buffer[2..n].iter().enumerate() {
        //     println!("Method {}: 0x{:02x}", i, method);
        //     if method == expectedMethod {
        //         hasAuthentication = true;
                
        //     }
        // }

        // if !hasAuthentication {
        //     let response = [0x05, 0xFF]; // No acceptable methods
        //     client.write_all(&response).await?;
        //     return Err("No acceptable authentication method found".into());
        // }

        // println!("Read {} bytes from client", n);

        // let b: u8 = buffer[1];
        // println!("Data: 0x{:02x}", b);

        // let b: u8 = buffer[2];
        // println!("Data: 0x{:02x}", b);

        // let b: u8 = buffer[3];
        // println!("Data: 0x{:02x}", b);

        // let b: u8 = buffer[4];
        // println!("Data: 0x{:02x}", b);

       Ok(())       
    }
}