use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub login: String,
    pub password: String,
}

impl User {
    pub fn create(username: String, password: String) -> Self {
        User {
            username: username.clone(),
            login: username,
            password,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub user: User,
    pub phone: String,
    pub telegram: Telegram,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Telegram {
    pub id: String,
    pub username: String,
    pub name: String,
    pub contact: String,
    pub lang: String,
}

impl Person {
    pub fn by_default() -> Self {
        Person {
            user: User::create("user".to_string(), "passwd".to_string()),
            phone: "".to_string(),
            telegram: Telegram {
                id: "".to_string(),
                username: "".to_string(),
                name: "".to_string(),
                contact: "".to_string(),
                lang: "".to_string(),
            },
        }
    }
}
