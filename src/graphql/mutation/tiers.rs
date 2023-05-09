use ::entity::{tiers, tiers::Entity as Tiers};
use async_graphql::{Context, Object, Result};
use entity::async_graphql::{self, InputObject, InputType, SimpleObject};
use graphql_example_core::sea_orm::{ActiveModelTrait, Set, InsertResult};
use graphql_example_core::Mutation;

use crate::db::Database;

#[derive(InputObject)]
pub struct TierInput {
    pub id: Option<i32>,
    pub title: String,
    pub image: Option<String>,
    pub tier: String,
    pub hori: Option<i32>,
    pub kind: Option<String>,
    pub game: String,
    pub user_id: Option<String>,
    pub list_id: Option<i32>,
    pub role: Option<String>,
}

impl TierInput {
    fn into_model_with_arbitrary_id(self) -> tiers::Model {
        tiers::Model {
            id: self.id.unwrap(),
            title: self.title,
            image: self.image,
            tier: self.tier,
            hori: self.hori,
            kind: self.kind,
            game: self.game,
            list_id: self.list_id,
            user_id: self.user_id,
            role: self.role,
        }
    }
}

#[derive(SimpleObject)]
pub struct MultiResult {
    pub success: bool,
    pub rows_affected: u64,
}

#[derive(Default)]
pub struct TiersMutation;

#[Object]
impl TiersMutation {
    pub async fn create_tiers(
        &self,
        ctx: &Context<'_>,
        input: Vec<TierInput>,
    ) -> Result<MultiResult> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let tiers_input: Vec<TierInput> = input;
        let tiers_model: Vec<tiers::Model> = tiers_input
            .into_iter()
            .map(|input| input.into_model_with_arbitrary_id())
            .collect();
        let res = Mutation::create_multiple_tiers(conn, tiers_model).await?;
        if res > 0 {
            Ok(MultiResult { success: true, rows_affected: res as u64 })
        }else{
            unimplemented!()
        }
    }

    pub async fn delete_tiers(&self, ctx: &Context<'_>, id: i32) -> Result<MultiResult> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();

        let res = Mutation::delete_tiers(conn, id)
            .await
            .expect("Cannot delete tiers");

        if res.rows_affected <= 1 {
            Ok(MultiResult {
                success: true,
                rows_affected: res.rows_affected,
            })
        } else {
            unimplemented!()
        }
    }

    pub async fn update_multi_tiers(
        &self,
        ctx: &Context<'_>,
        input: Vec<TierInput>,
    ) -> Result<MultiResult> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let tiers_input: Vec<TierInput> = input;
        let tiers_model: Vec<tiers::Model> = tiers_input
            .into_iter()
            .map(|input| input.into_model_with_arbitrary_id())
            .collect();
        let tasks = tiers_model.into_iter().map(|t| tiers::Model {
            id: t.id,
            title: t.title,
            image: t.image,
            tier: t.tier,
            hori: t.hori,
            kind: t.kind,
            game: t.game,
            list_id: t.list_id,
            user_id: t.user_id,
            role: t.role,
        });
        let mut c = 0;
        for t in tasks {
            let res = Mutation::update_tiers_by_id(conn, t.id, t)
                .await
                .map_err(|_| DbErr::Custom("Could not update tier".to_owned()));
            match res {
                Ok(r) => c += 1,
                Err(e) => c = 0,
            }
        }
        let rows = c;
        if rows > 1 {
            Ok(MultiResult {
                success: true,
                rows_affected: rows,
            })
        } else {
            unimplemented!()
        }
    }

    pub async fn update_tiers_by_id(
        &self,
        ctx: &Context<'_>,
        input: TierInput,
    ) -> Result<tiers::Model, DbErr> {
        let db = ctx.data::<Database>().unwrap();
        let conn = db.get_connection();
        let tiers: tiers::ActiveModel = Tiers::find_by_id(input.id.unwrap())
            .one(conn)
            .await?
            .ok_or(DbErr::Custom("Cannot find tiers.".to_owned()))
            .map(Into::into)?;

        tiers::ActiveModel {
            id: Set(input.id.unwrap()),
            title: Set(input.title),
            image: Set(input.image),
            tier: Set(input.tier),
            hori: Set(input.hori),
            kind: Set(input.kind),
            game: Set(input.game),
            user_id: Set(input.user_id),
            list_id: Set(input.list_id),
            role: Set(input.role),
        }
        .update(conn)
        .await
    }
}
