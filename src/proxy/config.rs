pub struct Config {

}

impl Config {
    pub fn new() -> Self {
        Config {}
    }

    pub fn get_expected_authentication_method() -> u8 {
        0x02 // should come from config
    }
}