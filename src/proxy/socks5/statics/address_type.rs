#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    IPv4 = 0x01,
    DomainName = 0x03,
    IPv6 = 0x04,
    Unassigned = 0x00,
}

impl AddressType {
    pub fn from_u8(value: u8) -> Self { 
        match value {
            0x01 => Self::IPv4,
            0x03 => Self::DomainName,
            0x04 => Self::IPv6,
            _ => Self::Unassigned,
        }
    }
}