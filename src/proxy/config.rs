use crate::proxy::socks5::statics::Authentication;
use std::collections::BTreeMap;

pub struct Config {}

impl Config {

    pub fn get_allowed_authentication_method() -> BTreeMap<u8, u8> {
        let mut map = BTreeMap::new();
        map.insert(0, Authentication::UsernamePassword.as_u8());
        map.insert(100, Authentication::NoAuthentication.as_u8());

        map
    }
}