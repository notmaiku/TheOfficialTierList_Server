use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize};

use crate::database::lists;

// use crate::database::lists;

#[derive(Deserialize)]
pub struct ReqList {
    pub title: Option<String>,
    pub user_id: Option<String>,
    pub game: Option<String>,
}

pub async fn create_list(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_list): Json<ReqList>,
) {
    let new_task = lists::ActiveModel {
        title: Set(request_list.title),
        user_id: Set(request_list.user_id),
        game: Set(request_list.game),
        ..Default::default()
    };

    let result = new_task.save(&database).await.unwrap();
    dbg!(result);
}