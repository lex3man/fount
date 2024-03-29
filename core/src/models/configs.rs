use serde::{Deserialize, Serialize};

use super::{Connector, DBconfig, Drivers, User};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub port: String,
    pub admin: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub settings: Settings,
    pub db: DBconfig,
}

impl Config {
    pub fn by_default() -> Self {
        Config {
            db: DBconfig {
                connector: Connector {
                    driver: Drivers::SQLite,
                    host: String::from("localhost"),
                    port: String::from("0000"),
                },
                db_name: String::from("main"),
                db_login: String::from("admin"),
                db_password: String::from("admin"),
            },
            settings: Settings {
                port: String::from("8080"),
                admin: User {
                    username: String::from("admin"),
                    login: String::from("root"),
                    password: String::from("admin"),
                },
            },
        }
    }

    pub fn get_port(&self) -> u16 {
        self.settings.port.parse().unwrap()
    }
}
