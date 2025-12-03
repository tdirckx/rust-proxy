pub struct Modules {
    buffer: Vec<u8>,
    host: String, 
    port: u16
}

impl Modules {
    
    pub fn new(buffer: &[u8], host: String, port: u16) -> Self {
        Modules {
            buffer: buffer.to_vec(),
            host,
            port
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

        println!("--- Inspecting Data ---");
        println!("Hook for adding modules ...");
        println!("Inspecting data: 0x{:02X}", self.buffer[0]);
        println!("Encrypted: {}", (self.port == 443));
        println!("Host: {}, Port: {}", self.host, self.port);
        println!("Data Length: {}", self.buffer.len());

         // Example: Check for TLS ClientHello (first byte 0x16)
        if self.buffer[0] != 0x16 {
            return Ok(());
        }
       

        // think about tls, most is encrypted, so we need to decode what is possible
        let mut data = Vec::new();
        let mut slice = Vec::new();

        for &byte in &self.buffer {
            if (32..=126).contains(&byte) {
                slice.push(byte);
            } else {
                if !slice.is_empty() && slice.len() >= 6 {
                    data.push(String::from_utf8_lossy(&slice).to_string());
                    slice.clear();
                }
                slice.clear();
            }
        }

        if slice.len() > 0 {
            data.push(String::from_utf8_lossy(&slice).to_string());
        }

        println!("Data: found {:?}", data);


        Ok(())
    }
}