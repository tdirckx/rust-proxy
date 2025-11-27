use crate::proxy::socks5::statics::Authentication;

pub struct Config {}

impl Config {

    pub fn get_allowed_authentication_method() -> Vec<u8> {
        vec![Authentication::UsernamePassword.as_u8()]
    }
}