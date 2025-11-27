#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Authentication {
    NoAuthentication = 0x00,
    GssApi = 0x01,
    UsernamePassword = 0x02,
    ChallengeHandshakeAuthenticationProtocol = 0x03,
    Unassigned,
    ChallengeResonseAuthenticationMethod = 0x05,
    SecureSocketsLayer = 0x06,
    NdsAuthentication = 0x07,
    MultiAuthenticationFramework = 0x08,
    JsonParameterBlock = 0x09,
    PrivateMethods,
    NoAcceptableMethods = 0xFF,
}


impl Authentication {
    pub fn from_u8(value: u8) -> Self { 
        match value {
            0x00 => Self::NoAuthentication,
            0x01 => Self::GssApi,
            0x02 => Self::UsernamePassword,
            0x03 => Self::ChallengeHandshakeAuthenticationProtocol,
            0x04 => Self::Unassigned,
            0x05 => Self::ChallengeResonseAuthenticationMethod,
            0x06 => Self::SecureSocketsLayer,
            0x07 => Self::NdsAuthentication,
            0x08 => Self::MultiAuthenticationFramework,
            0x09 => Self::JsonParameterBlock,
            0xFF => Self::NoAcceptableMethods,
            0x0A..=0x7F => Self::Unassigned,
            0x80..=0xFE => Self::PrivateMethods,
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}