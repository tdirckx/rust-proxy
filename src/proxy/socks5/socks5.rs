use std::error::Error;
use crate::proxy::socks5::General;
use crate::proxy::socks5::statics::Authentication;
use crate::proxy::{Connection, Config};


/**
 * Socks5 struct representing a SOCKS5 connection.
 */
pub struct Socks5 {
    connection: Connection,
    first_buffer: Vec<u8>,
}


/**
 * TODO: 
 * - if there is logic that is common between SOCKS versions, consider creating a base struct or trait to encapsulate shared functionality.
  - Implement error handling for unsupported authentication methods and connection failures.
  - Expand the implementation to handle the full SOCKS5 protocol, including command processing and data forwarding.
 */


/**
 * Implementation of Socks5 methods.
 */
impl Socks5 {

    /**
     * Create a new Socks5 instance.
     */
    pub fn new(connection: Connection, first_buffer: Vec<u8>) -> Self {
        println!("Received SOCKS5 handshake");

        Socks5 { 
            connection,
            first_buffer
        }
    }


    /**
     * Run the SOCKS5 authentication process.
     */
    pub async fn run(mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let authentication_method = self.validate_authentication().await?;
        println!("Selected authentication method: {:?}", authentication_method);

        //working on it a.t.m
        match authentication_method {
            Authentication::NoAuthentication => {
                println!("No authentication required, proceeding...");
            },
            Authentication::UsernamePassword => {
                self.connection.write(vec![General::Socks5.as_u8(), authentication_method.as_u8()]).await?;
                let security_response = self.connection.read(262).await?;
                println!("Received username/password authentication data: {:?}", String::from_utf8(security_response[1..].to_vec()) );
            },
            _ => {
                return Err("Unsupported authentication method".into());
            }
        }

        Ok(())
    }


    /**
     * Validate the client's authentication methods.
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