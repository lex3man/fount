use crate::models::Config;
use futures::executor::block_on;
use mongodb;
use sea_orm::{Database, DbErr};

pub fn connect(config: Config) {
    let db = config.db;
    db.connect();
}
