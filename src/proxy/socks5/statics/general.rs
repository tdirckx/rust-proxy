#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum General {
    Socks5 = 0x05,
    Socks4 = 0x04,    
}

impl General {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}