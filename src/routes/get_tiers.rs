use core::str;

use axum::{extract::{Path, Query}, http::StatusCode, Extension, Json};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Condition, QueryFilter, ColumnTrait};
use serde::{Serialize, Deserialize};


use crate::database::tiers::{Entity as Tiers};
use crate::database::tiers;

#[derive(Serialize)]
pub struct RespTier {
    id: i32,
    title: String,
    image: Option<String>,
    tier: String,
    kind: Option<String>,
    game: String,
}

pub async fn get_one_tier(
    Path(tier_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<RespTier>, StatusCode> {
    let tier = Tiers::find_by_id(tier_id).one(&database).await.unwrap();
    if let Some(tier) = tier {
        Ok(Json(RespTier {
            id: tier.id,
            title: tier.title,
            image: tier.image,
            tier: tier.tier,
            kind: tier.kind,
            game: String,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
pub struct GetTiersQueryParams {
    kind: Option<String>,
    game: Option<String>
}

pub async fn get_all_tiers(
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<GetTiersQueryParams>
) -> Result<Json<Vec<RespTier>>, StatusCode> {
    let mut kind_filter = Condition::all();
    if let Some(kind) = query_params.kind {
        kind_filter = kind_filter.add(tiers::Column::Kind.eq(kind));
    }
    let mut game_filter = Condition::all();
    if let Some(game) = query_params.game {
        game_filter = game_filter.add(tiers::Column::Game.eq(game));
    }
    let tiers = Tiers::find()
        .filter(kind_filter)
        .filter(game_filter)
        .all(&database)
        .await
        .map_err(|_err: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_tier| RespTier {
            id: db_tier.id,
            title: db_tier.title,
            image: db_tier.image,
            tier: db_tier.tier,
            kind: db_tier.kind,
            game: db_tier.game
        })
        .collect();
    Ok(Json(tiers))
}
