use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}