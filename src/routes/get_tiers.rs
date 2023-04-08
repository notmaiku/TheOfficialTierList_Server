use core::str;
use std::collections::HashMap;

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
    x: Option<i32>,
    kind: Option<String>,
    game: String,
    user_id: Option<String>
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
            x: tier.x,
            kind: tier.kind,
            game: tier.game,
            user_id: tier.user_id
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Deserialize)]
pub struct GetTiersQueryParams {
    user_id: Option<String>,
    game: Option<String>,
    kind: Option<String>
}

pub async fn get_all_tiers(
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<GetTiersQueryParams>,
    Path(params): Path<HashMap<String, String>>
) -> Result<Json<Vec<RespTier>>, StatusCode> {
    let mut kind_filter = Condition::all();
    if let Some(kind) = query_params.kind {
        kind_filter = kind_filter.add(tiers::Column::Kind.eq(kind));
    }
    let mut game_filter = Condition::all();
    if let Some(params) = params.get("game") {
        game_filter = game_filter.add(tiers::Column::Game.eq(params));
    }
    let mut user_id_filter = Condition::all();
    if let Some(params) = params.get("user_id") {
        user_id_filter = user_id_filter.add(tiers::Column::UserId.eq(params));
    }
    let tiers = Tiers::find()
        .filter(kind_filter)
        .filter(game_filter)
        .filter(user_id_filter)
        .all(&database)
        .await
        .map_err(|_err: DbErr| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_tier| RespTier {
            id: db_tier.id,
            title: db_tier.title,
            image: db_tier.image,
            tier: db_tier.tier,
            x: db_tier.x,
            kind: db_tier.kind,
            game: db_tier.game,
            user_id: db_tier.user_id
        })
        .collect();
    Ok(Json(tiers))
}
