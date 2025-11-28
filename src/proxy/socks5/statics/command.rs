#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    CONNECT = 0x01,
    BIND = 0x02,
    UDPAssociate = 0x03,
    Unassigned = 0x00,
}

impl Command {

    pub fn from_u8(value: u8) -> Self { 
        match value {
            0x01 => Self::CONNECT,
            0x02 => Self::BIND,
            0x03 => Self::UDPAssociate,
            _ => Self::Unassigned,
        }
    }
}