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
    pub image: Option<String>,
    pub tier: String,
    pub column: Option<i32>,
    pub kind: Option<String>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
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
         column: Set(request_tier.column),
         kind: Set(request_tier.kind),
         updated_at: Set(request_tier.updated_at),
         deleted_at: Set(request_tier.deleted_at),
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


pub async fn update_multiple_tiers(
    Extension(database): Extension<DatabaseConnection>,
    Json(body): Json<Vec<ReqTier>>,
) -> Result<(), StatusCode>{
    let tasks = body.into_iter().map(|t| ReqTier {
        id: t.id,
        title: t.title,
        image: t.image,
        tier: t.tier,
        column: t.column,
        kind: t.kind,
        updated_at: t.updated_at,
        deleted_at: t.deleted_at,
        game: t.game,
    });
    for t in tasks{
        atomic_update(Path(t.id), Extension(database.to_owned()) , Json(t))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    return Ok(())
}
    