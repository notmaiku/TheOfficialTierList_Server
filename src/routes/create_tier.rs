use axum::{Extension, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize};

use crate::database::tiers;

#[derive(Deserialize)]
pub struct ReqTier {
    pub title: String,
    pub image: String,
    pub tier: String,
    pub kind: Option<String>,
    pub game: String,
}

pub async fn create_tier(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_tier): Json<ReqTier>,
) {
    let new_task = tiers::ActiveModel {
        title: Set(request_tier.title),
        image: Set(Some(request_tier.image.into_bytes())),
        tier: Set(request_tier.tier),
        kind: Set(request_tier.kind),
        game: Set(request_tier.game),
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
        kind: t.kind,
        game: t.game,
    });
    for t in tasks{
        create_tier(Extension(database.to_owned()) , Json(t)).await;
    }
}
