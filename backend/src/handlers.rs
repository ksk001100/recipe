use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::*;
use serde::Deserialize;
use std::sync::Arc;

use crate::auth::Claims;
use crate::models::user;

pub async fn get_users(
    claims: Claims,
    Extension(db): Extension<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    println!("{:?}", claims);
    let users = user::Entity::find().all(&db).await.unwrap();

    Json(users)
}

pub async fn get_user_by_id(
    Path(id): Path<i64>,
    Extension(db): Extension<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    let user = user::Entity::find_by_id(id).one(&db).await.unwrap();

    Json(user)
}

#[derive(Deserialize)]
pub struct AddUser {
    name: String,
    email: String,
}

pub async fn add_user(
    Json(payload): Json<AddUser>,
    Extension(db): Extension<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    let user = user::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        ..Default::default()
    };

    let res: user::ActiveModel = user.insert(&db).await.unwrap();
}

pub async fn delete_user(
    Path(id): Path<i64>,
    Extension(db): Extension<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    let user: Option<user::Model> = user::Entity::find_by_id(id).one(&db).await.unwrap();
    let user: user::ActiveModel = user.unwrap().into();

    let res: DeleteResult = user.delete(&db).await.unwrap();

    StatusCode::OK
}
