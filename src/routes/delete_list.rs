use axum::{Extension, extract::Path};
use http::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::{Deserialize};

use crate::database::lists::{Entity as Lists};


#[derive(Deserialize)]
pub struct ReqList {
    pub title: Option<String>,
    pub user_id: Option<String>,
    pub game: Option<String>,
}

pub async fn delete_list(
    Path(list_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
)  -> Result<(), StatusCode>{
    Lists::delete_by_id(list_id).exec(&database).await.map_err(|_error|StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}