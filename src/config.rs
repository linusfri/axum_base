pub struct Config {
    pub database_url: String,
    pub address_and_port: String
    // For authentication
    //pub jwt_secret: String
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();

        let (database_url, address_and_port) = match (std::env::var("DB_CONNECTION_STRING"), std::env::var("ADDRESS_AND_PORT")) {
            (Ok(database_url), Ok(address_and_port)) => (database_url, address_and_port),
            _ => panic!("Check env file. DB_CONNECTION_STRING and ADDRESS_AND_PORT must be set.")
        };

        Config { database_url, address_and_port }
    }
}