#[derive(Debug)]
pub struct Config {
    pub connection_string: String,
    pub address_and_port: String
    // For authentication
    //pub jwt_secret: String
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let connection_string = std::env::var("DB_CONNECTION_STRING").expect("DB_CONNECTION_STRING must be set.");
        let address_and_port = std::env::var("ADDRESS_AND_PORT").expect("ADDRESS_AND_PORT must be set.");

        Config { connection_string, address_and_port }
    }
}