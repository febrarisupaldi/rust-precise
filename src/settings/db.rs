use std::env;

use sqlx::{mysql::MySqlConnectOptions, MySqlPool};
use dotenv::dotenv;

#[derive(Debug)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String
}

impl DatabaseSettings{
    pub fn parse_conn_string(&self) -> MySqlConnectOptions {
        MySqlConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
    }

    pub fn db_conn_string(&self) -> MySqlConnectOptions{
        self.parse_conn_string().database(&self.database_name)
    }
}

pub async fn db_pool() -> MySqlPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Failed to get settings database");
    let connection = MySqlPool::connect(&db_url).await.expect("Cannot connect to database");
    connection
}