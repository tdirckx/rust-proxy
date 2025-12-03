use std::error::Error;
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt };
use tokio::net::TcpStream;
use crate::proxy::socks5::statics::{General, Command, Authentication, AddressType};
use crate::proxy::{Connection, Config};
use std::collections::BTreeMap;

use crate::proxy::socks5::modules::Modules;

/**
 * Socks5 struct representing a SOCKS5 connection.
 */
pub struct Socks5 {
    connection: Connection,
    first_buffer: Vec<u8>
}


/**
 * TODO: 
 *- if there is logic that is common between SOCKS versions, consider creating a base struct or trait to encapsulate shared functionality.
  - Implement error handling for unsupported authentication methods and connection failures.
  - Expand the implementation to handle the full SOCKS5 protocol, including command processing and data forwarding.
  - refactoring
  - error handling
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
            Authentication::NoAuthentication => self.no_authentication(authentication_method.as_u8()).await?,
            Authentication::UsernamePassword => self.authentication(authentication_method.as_u8()).await?,
            _ => {
                return Err("Unsupported authentication method".into());
            }
        };

        Ok(())
    }


    /**
     * Validate the client's authentication methods.
     */
    async fn validate_authentication(&mut self) -> Result<Authentication, Box<dyn Error + Send + Sync>> {

        let no_acceptable_methods: u8 = Authentication::NoAcceptableMethods.as_u8();
        let allowed_methods: BTreeMap<u8, u8> = Config::get_allowed_authentication_method();

        for &method in self.first_buffer[2..].iter().rev() {
            for &allowed_method in allowed_methods.values() {
                if method == allowed_method {
                    return Ok(Authentication::from_u8(method));
                }
            }
        }

        self.connection.write(vec![no_acceptable_methods]).await?;
        return Err("No acceptable authentication method found".into());
    }


    /**
     * method for no authentication
     */
    async fn no_authentication(&mut self, authentication_method: u8) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.connection.write(vec![General::Socks5.as_u8(), authentication_method]).await?;
        
        self.handle_request().await?;

        Ok(())
    }


    /**
     * authenticate using username and password
     */
    async fn authentication(&mut self, authentication_method: u8) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.connection.write(vec![General::Socks5.as_u8(), authentication_method]).await?;

        let buffer = self.connection.read(513).await?;

        if buffer.len() == 0 || buffer[0] != 0x01 {
            self.connection.write(vec![0x01, 0x01]).await?;
            return Err("Invalid username/password authentication version".into());
        }

        let username_length = buffer[1] as usize;
        let username = buffer[2..2 + username_length].to_vec();
        let password_length = buffer[2 + username_length] as usize;
        let password = buffer[3 + username_length..3 + username_length + password_length].to_vec();

        if !self.check_credentials(username, password) {
            self.connection.write(vec![0x01, 0x01]).await?;
            return Err("Invalid username or password".into());
        }

        println!("User authenticated successfully");

        self.connection.write(vec![0x01, 0x00]).await?;

        self.handle_request().await?;

        Ok(())
    }


    /**
     * check the provided username and password
     */
    fn check_credentials(&mut self, username: Vec<u8>, password: Vec<u8>) -> bool {
        println!("Username and Password received for authentication");

        // Here you would normally check the credentials against a database or predefined values
        if username == b"username" && password == b"password" {
            return true;
        }
        
        false
    }


    /**
     * handle the SOCKS5 request after authentication
     */
    async fn handle_request(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (upstream, host, port) = self.handle_command().await?;

        self.tunnel(upstream, host, port).await?;

        Ok(())
    }


    /**
     * handle the SOCKS5 command from the client
     */
    async fn handle_command(&mut self) -> Result<(Connection, String, u16), Box<dyn Error + Send + Sync>> {
        let request = self.connection.read(258).await?;
        if request.len() < 4 {
            return Err("Invalid SOCKS5 request".into());
        }
        
        if request[0] != General::Socks5.as_u8() {
            return Err("Invalid SOCKS5 version".into());
        }

        let command = request[1];
        match Command::from_u8(command) {
            Command::CONNECT => {},
            _ =>  return Err("Only CONNECT command supported".into()),
        }

        let (host, port) = match AddressType::from_u8(request[3]) {
            AddressType::IPv4 => self.fetch_host_and_port_ipv4(request).await?,
            AddressType::DomainName => self.fetch_host_and_port_domain(request).await?,
            AddressType::IPv6 => { return Err("IPv6 not supported yet".into()); },
            _ => { return Err("Invalid address type".into()); },    
        };

        println!("Connecting to {}:{}", host, port);
    
        let upstream = Connection::new( TcpStream::connect(format!("{}:{}", host, port)).await? );

        self.connection.write(vec![General::Socks5.as_u8(), 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;    

        Ok((upstream, host, port))
    }

    
    /**
     * create a tunnel between client and upstream server
     */
    async fn tunnel(&mut self, mut upstream: Connection, host: String, port: u16) -> Result<(),  Box<dyn Error + Send + Sync>> {

        let (client_read, client_write) = self.connection.stream.split();
        let (upstream_read, upstream_write) = upstream.stream.split();

        tokio::try_join!(
            Self::forward(client_read, upstream_write, &host, &port), 
            Self::forward(upstream_read, client_write, &host, &port)
        )?;

        Ok(())
    }


    /**
     * fetch ipv4 and port from request 
     */
    async fn fetch_host_and_port_ipv4(&mut self, request: Vec<u8>) -> Result<(String, u16), Box<dyn Error + Send + Sync>> {
        if request.len() != 10 {
            return Err("Invalid IPv4 request".into());
        }

        let addr_bytes = &request[4..8];
        let port_bytes = &request[8..10];
        let ip = format!("{}.{}.{}.{}", addr_bytes[0], addr_bytes[1], addr_bytes[2], addr_bytes[3]);
        let port = u16::from_be_bytes([port_bytes[0], port_bytes[1]]);
        Ok((ip, port))
    }


    /**
     * fetch domain and port from request
     */
    async fn fetch_host_and_port_domain(&mut self, request: Vec<u8>) -> Result<(String, u16), Box<dyn Error + Send + Sync>> {

        println!("Domain request: {:?}", request);
        let domain_length = request[4] as usize;

        println!("Domain length: {}", domain_length);

        let end_domain = 5 + domain_length;

        println!("End Domain: {}", end_domain);

        let domain_bytes = request[5..end_domain].to_vec();

        println!("Domain bytes: {:?}", domain_bytes);


        let port_bytes = &request[end_domain..];
        let domain = String::from_utf8(domain_bytes).map_err(|_| "Invalid domain name")?;
        let port = u16::from_be_bytes([port_bytes[0], port_bytes[1]]);
        Ok((domain, port))
    }



    /**
     * forward data between incoming and outgoing streams
     */
    async fn forward<R, W>(
        mut incoming: R,
        mut outgoing: W,
        host: &str,
        port: &u16
    ) -> Result<(), Box<dyn Error + Send + Sync>> 
    where 
        R: AsyncRead + Unpin,
        W: AsyncWrite + Unpin,
    {
        loop {
            let mut buffer = vec![0u8; 8192];
            let n = incoming.read(&mut buffer).await?;
            if n == 0 {
                break;
            }

            let data = &buffer[..n];

            Self::implement_modules(data, host, port).await?;
            outgoing.write_all(data).await?;
        }

        Ok(())
    }


    /**
     * Hook for implementing modules
     */
    async fn implement_modules(buffer: &[u8], host: &str, port: &u16) -> Result<(), Box<dyn Error + Send + Sync>> {

        let modules = Modules::new(&buffer, host.to_string(), *port);
        modules.run()?;

        Ok(())
    }

}