use serde::{Deserialize, Serialize};

use crate::configs::config_dir;

#[derive(Serialize, Deserialize, Debug)]
pub enum Drivers {
    SQLite,
    MySQL,
    PostgreSQL,
    MongoDB,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Connector {
    pub driver: Drivers,
    pub host: String,
    pub port: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DBconfig {
    pub connector: Connector,
    pub db_name: String,
    pub db_login: String,
    pub db_password: String,
}

impl DBconfig {
    pub fn connect(&self) {
        println!("Connecting...");
        match self.connector.driver {
            Drivers::SQLite => {
                println!("SQLite");
            }
            Drivers::MySQL => {
                println!("MySQL");
            }
            Drivers::MongoDB => {
                println!("Mongo");
            }
            Drivers::PostgreSQL => {
                println!("Postgres");
            }
        };
        println!("Connected");
    }
}
