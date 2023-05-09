use async_graphql::{Context, Object, Result};
use entity::{async_graphql, tiers};
use graphql_example_core::{Query};

use crate::db::Database;

#[derive(Default)]
pub struct TierQuery;

#[Object]
impl TierQuery {
    async fn get_tiers(&self, ctx: &Context<'_>) -> Result<Vec<tiers::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        Ok(Query::get_all_tiers(conn)
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_tier(&self, ctx: &Context<'_>, id: i32) -> Result<Option<tiers::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        Ok(Query::find_tier_by_id(conn, id)
            .await
            .map_err(|e| e.to_string())?)
    }

    pub async fn get_all_tiers(
        &self,
        ctx: &Context<'_>,
        list_id: i32,
    ) -> Result<Vec<tiers::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        Ok(Query::get_tiers_by_list(conn, list_id)
            .await
            .map_err(|e| e.to_string())?)
    }
}
