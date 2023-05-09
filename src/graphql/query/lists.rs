use async_graphql::{Context, Object, Result};
use entity::{async_graphql, lists};
use graphql_example_core::Query;

use crate::db::Database;

#[derive(Default)]
pub struct ListQuery;

#[Object]
impl ListQuery {
    async fn get_list(&self, ctx: &Context<'_>, id: i32) -> Result<Option<lists::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        Ok(Query::find_list_by_id(conn, id)
            .await
            .map_err(|e| e.to_string())?)
    }
    async fn get_lists(&self, ctx: &Context<'_>) -> Result<Vec<lists::Model>> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        Ok(Query::get_all_lists(conn)
            .await
            .map_err(|e| e.to_string())?)
    } 
}