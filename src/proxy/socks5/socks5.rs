use std::error::Error;
use crate::proxy::socks5::statics::Authentication;
use crate::proxy::{Connection, Config};


/**
 * Represents a SOCKS5 proxy handler.
 */
pub struct Socks5 {
    connection: Connection,
    first_buffer: Vec<u8>,
}


/**
 * Implementation of Socks5 methods.    
 */
impl Socks5 {

    /**
     *  Create a new Socks5 instance.
     */
    pub fn new(connection: Connection, first_buffer: Vec<u8>) -> Self {
        println!("Received SOCKS5 handshake");

        Socks5 { 
            connection,
            first_buffer
        }
    }

    /**
     * Run the SOCKS5 protocol handling.
     */
    pub async fn run(mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let authentication_method = self.validate_authentication().await?;
        println!("Selected authentication method: {:?}", authentication_method);


        Ok(())
    }


    /**
     * Send the authentication method to the connection.
     */
    async fn validate_authentication(&mut self) -> Result<Authentication, Box<dyn Error + Send + Sync>> {

        let no_acceptable_methods: u8 = Authentication::NoAcceptableMethods.as_u8();
        let allowed_methods = Config::get_allowed_authentication_method();

        for &method in &self.first_buffer[2..] {
            if allowed_methods.contains(&method) {
                return Ok(Authentication::from_u8(method));
            }
        }

        self.connection.write(vec![no_acceptable_methods]).await?;
        return Err("No acceptable authentication method found".into());
    }


}