use tokio::net::{TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use log::{error, info};


/**
 * Represents a connected Connection.
 */
pub struct Connection {
    stream: TcpStream,
}


/**
 * Implementation of Connection methods.
 */
impl Connection {

    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    pub async fn read(&mut self, size: usize) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let mut buffer = vec![0u8; size];
        let bytes_read = self.stream.read(&mut buffer).await?;

        if bytes_read == 0 {
            return Err("Connection closed".into());
        }

        buffer.truncate(bytes_read);

        Ok(buffer)
    }


    /**
     * Write messages to the client.
     */
    pub async fn write(&mut self, bytes: Vec<u8> ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.stream.write_all(&bytes).await?;

        Ok(())
    }


    // /**
    //  * Create a new Connection instance by reading from the TcpStream.
    //  */
    // pub async fn new(stream: TcpStream) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> { 
    //     let mut connection = Connection { 
    //         buffer: vec![0u8; 262],
    //         bytes_read: 0,
    //         closed: false,
    //         stream
    //     };

    //     Connection.intialize().await?;
        
    //     Ok(connection)
    // }
    

    // pub async fn intialize(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //    self.bytes_read = self.stream.read(&mut self.buffer).await?;

    //     if self.bytes_read == 0 {
    //         self.closed = true;
    //         return Err("Connection closed".into());
    //     }

    //     Ok(())
    // }

    
    // /**
    //  * Get the internal buffer.
    //  */
    // pub fn get_buffer(&self) -> &[u8] {
    //     &self.buffer
    // }

    // pub fn get_bytes_read(&self) -> usize {
    //     self.bytes_read
    // }

    // /**
    //  * Check if the client is using SOCKS5 protocol.
    //  */
    // pub fn is_socks5(&self) -> bool {
    //     !self.closed && self.buffer[0] == General::Socks5.as_u8()
    // }


    // /**
    //  * Check if the client is using SOCKS4 protocol.
    //  */
    // pub fn is_socks4(&self) -> bool {
    //     !self.closed && self.buffer[0] == General::Socks4.as_u8()
    // }


    // /**
    //  * Check if the client connection is valid.
    //  */
    // pub fn is_valid(&self) -> bool {
    //     self.is_socks5() || self.is_socks4()
    // }


    // /**
    //  * Get the protocol method as u8.
    //  */
    // pub fn get_version(&self) -> u8 {
    //     return if self.is_socks5() {
    //         General::Socks5.as_u8()
    //     } else if self.is_socks4() {
    //         General::Socks4.as_u8()
    //     } else {
    //         General::UnknownVersionIndentifier.as_u8()
    //     }
    // }


    // /**
    //  * Write the chosen authentication method to the client.
    //  */
    // pub async fn write_chosen_authentication(&mut self, chosenMethod: u8) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //     let mut message_to_send = Vec::with_capacity(2);
    //     message_to_send.push(self.get_version());
    //     message_to_send.push(chosenMethod);

    //     println!(
    //         "Sending chosen authentication method: {}",
    //         message_to_send
    //             .iter()
    //             .map(|b| format!("0x{:02X}", b))
    //             .collect::<Vec<_>>()
    //             .join(" ")
    //     );

    //     self.write(message_to_send).await?;
        
    //     Ok(())
    // }


    // /**
    //  * Write messages to the client.
    //  */
    // pub async fn write(&mut self, bytes: Vec<u8> ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //     if self.closed {
    //         return Err("Attempted write on closed connection".into());
    //     }

    //     self.stream.write_all(&bytes).await?;

    //     Ok(())
    // }

}