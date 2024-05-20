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

        let (connection_string, address_and_port) = match (std::env::var("DB_CONNECTION_STRING"), std::env::var("ADDRESS_AND_PORT")) {
            (Ok(connection_string), Ok(address_and_port)) => (connection_string, address_and_port),
            _ => panic!("Check env file. DB_CONNECTION_STRING and ADDRESS_AND_PORT must be set.")
        };

        Config { connection_string, address_and_port }
    }
}