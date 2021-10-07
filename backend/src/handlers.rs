use axum::{
    response::IntoResponse,
};

pub async fn get_users() -> impl IntoResponse {
    "Hello from get users"
}

pub async fn get_user_by_id() -> impl IntoResponse {
    "Hello from get users by id"
}

pub async fn add_user() -> impl IntoResponse {
    "Hello from add user"
}

pub async fn delete_user() -> impl IntoResponse {
    "Hello from delete user"
}