use axum::{extract::Path, Extension, http::StatusCode};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde::Serialize;

use crate::database::tiers::{Entity as Tasks, self};

#[derive(Serialize)]
pub struct RespTask{
    pub title: String,
    pub image: String,
    pub tier: String,
    pub kind: String,
}

pub async fn delete_tier(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<(), StatusCode>{
    // let task = if let Some(task)=Tasks::find_by_id(task_id).one(&database)
    // .await
    // .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR) ? {
    //     task.into_active_model()
    // }else {
    //     return Err(StatusCode::NOT_FOUND);
    // };
    // Tasks::delete(task).exec(&database).await.map_err(|_error|{
    //     StatusCode::INTERNAL_SERVER_ERROR
    // })?;
    Tasks::delete_by_id(task_id).exec(&database).await.map_err(|_error|StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

pub async fn delete_many_tasks(
    Path(tier_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>
) -> Result<(), StatusCode>{
    Tasks::delete_many()
        .filter(tiers::Column::Id.eq(tier_id))
        .exec(&database)
        .await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

pub async fn soft_delete(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>
){
    
}
