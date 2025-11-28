#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum General {
    Socks5 = 0x05,
    Socks4 = 0x04,
    HttpConnect = 0x43,
    _Uknown = 0x00,
}

impl General {

    pub fn from_u8(value: u8) -> Self { 
        match value {
            0x04 => Self::Socks4,
            0x05 => Self::Socks5,
            0x43 => Self::HttpConnect,
            _ => Self::_Uknown,
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}