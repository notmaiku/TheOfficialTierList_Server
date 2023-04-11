use axum::{Extension, Json, extract::Path};
use http::StatusCode;
use sea_orm::{DatabaseConnection, Set, EntityTrait, QueryFilter,ColumnTrait};
use serde::Deserialize;                
use crate::database::{lists};          
use crate::database::lists::{Entity as Lists};

#[derive(Deserialize)]
pub struct ReqList {
    pub title: Option<String>,
    pub user_id: Option<String>,
    pub game: Option<String>,
}


pub async fn update_list(
    Path(list_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_list): Json<ReqList>,
) -> Result<(), StatusCode> {
    let update_list = lists::ActiveModel {
        title: Set(request_list.title),
        user_id: Set(request_list.user_id),
        game: Set(request_list.game),
        ..Default::default()
    };
       Lists::update(update_list)
        .filter(lists::Column::Id.eq(list_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    return Ok(())
}
