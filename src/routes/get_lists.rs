use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
};
use serde::{Serialize};

use crate::database::lists;
use crate::database::lists::Entity as Lists;


#[derive(Serialize)]
pub struct RespList {
    id: i32,
    title: Option<String>,
    game: Option<String>,
    user_id: Option<String>,
}

pub async fn get_one_list(
    Path(list_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<RespList>, StatusCode> {
    let list = Lists::find_by_id(list_id).one(&database).await.unwrap();
    if let Some(list) = list {
        Ok(Json(RespList {
            id: list.id,
            title: list.title,
            game: list.game,
            user_id: list.user_id,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_list_by_name_n_game(
    Path(params): Path<(String, String)>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<RespList>, StatusCode> {
    let list = Lists::find()
        .filter(lists::Column::Title.eq(params.0))
        .filter(lists::Column::Game.eq(params.1))
        .one(&database)
        .await
        .unwrap();
    if let Some(list) = list {
        Ok(Json(RespList {
            id: list.id,
            title: list.title,
            game: list.game,
            user_id: list.user_id,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_users_list(
    Extension(database): Extension<DatabaseConnection>,
    Path(user_id): Path<String>,
) -> Result<Json<Vec<RespList>>, StatusCode> {
    let lists = Lists::find()
        .filter(Condition::all().add(lists::Column::UserId.eq(user_id)))
        .all(&database)
        .await
        .map_err(|_err: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_list| RespList {
            id: db_list.id,
            title: db_list.title,
            game: db_list.game,
            user_id: db_list.user_id,
        })
        .collect();
    Ok(Json(lists))
}

pub async fn get_list_rows(
    Extension(database): Extension<DatabaseConnection>,
) -> Result<String, StatusCode> {
    let rows = Lists::find().count(&database).await.unwrap();
    if let Some(rows) = Some(rows) {
        Ok(rows.to_string())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
