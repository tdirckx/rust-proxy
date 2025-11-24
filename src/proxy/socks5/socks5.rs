use tokio::net::{TcpStream};

use std::error::Error;
use crate::proxy::socks5::statics::Authentication;
use crate::proxy::{Client, Config};


/**
 * Represents a SOCKS5 proxy handler.
 */
pub struct Socks5 {
    client: Client
}


/**
 * Implementation of Socks5 methods.    
 */
impl Socks5 {

    /**
     *  Create a new Socks5 instance.
     */
    pub fn new(mut client: Client) -> Self {
        Socks5 { client }
    }

    /**
     * Run the SOCKS5 protocol handling.
     */
    pub async fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.send_authentication_method().await?;

        Ok(())
    }


    /**
     * Send the authentication method to the client.
     */
    async fn send_authentication_method(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let buffer = self.client.get_buffer();

        let mut hasAuthentication: bool = false;
        for(i, &method) in buffer[2..self.client.get_bytes_read()].iter().enumerate() {

            println!("--- auth check ---");
            print!("Method {}: {}\n", i, method);
            print!("Expected Method: {}\n", Config::get_expected_authentication_method());

            hasAuthentication = method == Config::get_expected_authentication_method();
        }

        println!("hasAuthentication: {}", hasAuthentication);

        if !hasAuthentication {
            self.client.write_chosen_authentication(0xFF).await?;
            return Err("No acceptable authentication method found".into());
        }

        self.client.write_chosen_authentication(Config::get_expected_authentication_method()).await?;

        Ok(())
    }


}