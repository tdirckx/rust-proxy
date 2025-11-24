use tokio::net::{TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::{error, info};

use crate::proxy::socks5::{Socks5, General};

/**
 * Represents a connected client.
 */
pub struct Client {
    buffer: [u8; 262],
    bytes_read: usize,
    closed: bool,
    client: TcpStream,
}


/**
 * Implementation of Client methods.
 */
impl Client {

    /**
     * Create a new Client instance by reading from the TcpStream.
     */
    pub async fn new(mut client: TcpStream) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let mut buffer = [0u8; 262];
        let bytes_read = client.read(&mut buffer).await?;
    
        let closed = bytes_read == 0;
        if closed  {
            return Err("Connection closed".into());
        } else {
            println!("Client read {} bytes", bytes_read);
        }

        Ok(Client { 
            buffer,
            bytes_read,
            closed,
            client
        })
    }


    /**
     * Get the internal buffer.
     */
    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn get_bytes_read(&self) -> usize {
        self.bytes_read
    }

    /**
     * Check if the client is using SOCKS5 protocol.
     */
    pub fn is_socks5(&self) -> bool {
        !self.closed && self.buffer[0] == General::Socks5.as_u8()
    }


    /**
     * Check if the client is using SOCKS4 protocol.
     */
    pub fn is_socks4(&self) -> bool {
        !self.closed && self.buffer[0] == General::Socks4.as_u8()
    }


    /**
     * Check if the client connection is valid.
     */
    pub fn is_valid(&self) -> bool {
        self.is_socks5() || self.is_socks4()
    }


    /**
     * Get the protocol method as u8.
     */
    pub fn get_version(&self) -> u8 {
        return if self.is_socks5() {
            General::Socks5.as_u8()
        } else if self.is_socks4() {
            General::Socks4.as_u8()
        } else {
            General::UnknownVersionIndentifier.as_u8()
        }
    }


    /**
     * Write the chosen authentication method to the client.
     */
    pub async fn write_chosen_authentication(&mut self, chosenMethod: u8) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut message_to_send = Vec::with_capacity(2);
        message_to_send.push(self.get_version());
        message_to_send.push(chosenMethod);

        self.write(message_to_send).await?;

        Ok(())
    }


    /**
     * Write messages to the client.
     */
    pub async fn write(&mut self, bytes: Vec<u8> ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.closed {
            return Err("Attempted write on closed connection".into());
        }

        self.client.write_all(&bytes).await?;

        Ok(())
    }

}