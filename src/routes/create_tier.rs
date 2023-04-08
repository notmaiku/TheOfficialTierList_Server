use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize};

use crate::database::tiers;

#[derive(Deserialize)]
pub struct ReqTier {
    pub title: String,
    pub image: Option<String>,
    pub tier: String,
    pub x: Option<i32>,
    pub kind: Option<String>,
    pub game: String,
    pub user_id: Option<String>,
    pub list_id: Option<String>,
    pub role: Option<String>,
}

pub async fn create_tier(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_tier): Json<ReqTier>,
) {
    let new_task = tiers::ActiveModel {
        title: Set(request_tier.title),
        image: Set(request_tier.image),
        tier: Set(request_tier.tier),
        x: Set(request_tier.x),
        kind: Set(request_tier.kind),
        game: Set(request_tier.game),
        user_id: Set(request_tier.user_id),
        list_id: Set(request_tier.list_id),
        role: Set(request_tier.role),
        ..Default::default()
    };

    let result = new_task.save(&database).await.unwrap();
    dbg!(result);
}

pub async fn create_multiple_tiers(
    Extension(database): Extension<DatabaseConnection>,
    Json(body): Json<Vec<ReqTier>>,
) {
    let tasks = body.into_iter().map(|t| ReqTier {
        title: t.title,
        image: t.image,
        tier: t.tier,
        x: t.x,
        kind: t.kind,
        game: t.game,
        user_id: t.user_id,
        list_id: t.list_id,
        role: t.role,
    });
    for t in tasks{
        create_tier(Extension(database.to_owned()) , Json(t)).await;
    }
}
