use ::entity::{lists::Entity as Lists};
use async_graphql::{Object, Result};
use entity::{async_graphql::{self, Context, InputObject}};
use graphql_example_core::{
    sea_orm::{ActiveModelTrait, Set}
};
use migration::DbErr;

use crate::db::Database ;

use super::tiers::MultiResult;

#[derive(InputObject)]
pub struct ListInput {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub user_id: Option<String>,
    pub game: Option<String>,
}

#[derive(Default)]
pub struct ListMutation;

#[Object]
impl ListMutation {
    pub async fn create_list(
        &self,
        ctx: &Context<'_>,
        input: ListInput,
    ) -> Result<entity::lists::Model, DbErr> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let new_task = entity::lists::ActiveModel {
            title: Set(input.title),
            user_id: Set(input.user_id),
            game: Set(input.game),
            ..Default::default()
        };

        let result = new_task.insert(conn).await.unwrap();
        Ok(result)
    }
    pub async fn delete_list(&self, ctx: &Context<'_>, id: i32) -> Result<MultiResult, DbErr> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let del_rest = Lists::delete_by_id(id).exec(conn).await?;
        Ok(MultiResult{
            success: true,
            rows_affected: del_rest.rows_affected 
        })
    }

}
