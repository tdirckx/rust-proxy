use tokio::net::{TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};


/**
 * Represents a client connection.
 */
pub struct Connection {
    stream: TcpStream,
}


/**
 * Implementation of Connection methods.
 */
impl Connection {

    /**
     * Create a new Connection instance.
     */
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }


    /**
     * Read data from the client.
     */
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
}