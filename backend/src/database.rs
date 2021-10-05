use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::sync::Arc;
use std::time::Duration;

pub type DB = Pool<MySql>;

pub async fn connect(database_url: &str) -> Result<DB, crate::Error> {
    MySqlPoolOptions::new()
        .max_connections(100)
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(database_url)
        .await
        .map_err(|err| crate::Error::ConnectingToDatabase(err.to_string()))
}
