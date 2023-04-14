use axum::{extract::Path, Extension, http::StatusCode};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde::Serialize;

use crate::database::tiers::{Entity as Tiers, self};

#[derive(Serialize)]
pub struct ReqTier {
    pub title: String,
    pub image: Option<String>,
    pub tier: String,
    pub hori: Option<i32>,
    pub kind: Option<String>,
    pub game: String,
    pub user_id: Option<String>,
    pub list_id: Option<String>,
    pub role: Option<String>,
}

pub async fn delete_tier(
    Path(tier_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<(), StatusCode>{
    Tiers::delete_by_id(tier_id).exec(&database).await.map_err(|_error|StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}

pub async fn delete_many_tiers(
    Path(list_id): Path<String>,
    Extension(database): Extension<DatabaseConnection>
) -> Result<(), StatusCode>{
    Tiers::delete_many()
        .filter(tiers::Column::ListId.eq(list_id))
        .exec(&database)
        .await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}