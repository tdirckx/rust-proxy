#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum General {
    UnknownVersionIndentifier = 0xFF,
    Socks5 = 0x05,
    Socks4 = 0x04,    
}

impl General {
    pub fn from_u8(value: u8) -> Self { 
        match value {
            0xFF => Self::UnknownVersionIndentifier,
            0x05 => Self::Socks5,
            0x04 => Self::Socks4,
            _ => Self::UnknownVersionIndentifier,
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}