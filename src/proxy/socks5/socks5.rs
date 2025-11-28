use std::error::Error;
use tokio::io::{AsyncRead, AsyncWrite, AsyncReadExt, AsyncWriteExt };
use tokio::net::TcpStream;
use crate::proxy::socks5::statics::{General, Command, Authentication, AddressType};
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
  - refactoring
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
        let allowed_methods = Config::get_allowed_authentication_method();

        for &method in &self.first_buffer[2..] {
            if allowed_methods.contains(&method) {
                return Ok(Authentication::from_u8(method));
            }
        }

        self.connection.write(vec![no_acceptable_methods]).await?;
        return Err("No acceptable authentication method found".into());
    }


    /**
     * 
     */
    async fn no_authentication(&mut self, authentication_method: u8) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.connection.write(vec![General::Socks5.as_u8(), authentication_method]).await?;
        
        self.handle_request().await?;

        Ok(())
    }


    /**
     * 
     */
    async fn handle_request(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let upstream = self.handle_command().await?;

        self.tunnel(upstream).await?;

        Ok(())
    }


    /**
     * 
     */
    async fn handle_command(&mut self) -> Result<Connection, Box<dyn Error + Send + Sync>> {
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
    
        let upstream = Connection::new( TcpStream::connect(format!("{}:{}", host, port)).await? );

        self.connection.write(vec![General::Socks5.as_u8(), 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?;    

        Ok(upstream)
    }

    
    /**
     * 
     */
    async fn tunnel(&mut self, mut upstream: Connection) -> Result<(),  Box<dyn Error + Send + Sync>> {

        let (client_read, client_write) = self.connection.stream.split();
        let (upstream_read, upstream_write) = upstream.stream.split();

        let _ = tokio::try_join!(
            Self::forward(client_read, upstream_write), 
            Self::forward(upstream_read, client_write)
        );

        Ok(())
    }


    /**
     * 
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
     * 
     */
    async fn fetch_host_and_port_domain(&mut self, request: Vec<u8>) -> Result<(String, u16), Box<dyn Error + Send + Sync>> {
        let domain_length = request[4] as usize;
        let end_domain = 4 + domain_length;
        let domain_bytes = request[4..end_domain].to_vec();
        let port_bytes = &request[end_domain..];
        let domain = String::from_utf8(domain_bytes).map_err(|_| "Invalid domain name")?;
        let port = u16::from_be_bytes([port_bytes[0], port_bytes[1]]);
        Ok((domain, port))
    }



    /**
     * 
     */
    async fn forward<R, W>(
        mut incoming: R,
        mut outgoing: W,
    ) -> Result<(), Box<dyn Error + Send + Sync>> 
    where 
        R: AsyncRead + Unpin,
        W: AsyncWrite + Unpin,
    {
        loop {
            let mut buffer = vec![0u8; 8192];
            let n = incoming.read(&mut buffer).await?;
            if n == 0 {
                return Err("Unexpected disconnected!".into());
            }

            Self::inspect(&buffer).await?;
            outgoing.write_all(&buffer[..n]).await?;
        }

    }


    /**
     * 
     */
    async fn inspect(_buffer: &Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync>> {
        println!("Hook for adding modules ...");

        Ok(())
    }

}