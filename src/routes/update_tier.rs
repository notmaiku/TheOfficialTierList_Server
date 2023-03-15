use axum::http::StatusCode;
use axum::{extract::Path, Extension, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{prelude::DateTime, DatabaseConnection, Set};
use serde::Deserialize;
use crate::database::tiers::{Entity as Tasks, self};

#[derive(Deserialize)]
pub struct ReqTier {
    pub id: i32,
    pub title: String,
    pub image: Option<Vec<u8>>,
    pub tier: String,
    pub kind: Option<String>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub user_id: Option<i32>,
    pub game: String
}

pub async fn atomic_update(
    Path(tier_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_tier): Json<ReqTier>,
) -> Result<(),StatusCode>{
    let update_task = tiers::ActiveModel {
         id: Set(tier_id),
         title: Set(request_tier.title),
         image: Set(request_tier.image),
         tier: Set(request_tier.tier),
         kind: Set(request_tier.kind),
         updated_at: Set(request_tier.updated_at),
         deleted_at: Set(request_tier.deleted_at),
         user_id: Set(request_tier.user_id),
         game: Set(request_tier.game)
    };
    // Tasks::update(update_task)
    Tasks::update(update_task)
        .filter(tiers::Column::Id.eq(tier_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    return Ok(())
}
