use ::entity::{tiers, tiers::Entity as Tiers};
use sea_orm::*;

pub struct ReqList {
    pub title: Option<String>,
    pub user_id: Option<String>,
    pub game: Option<String>,
}

pub struct Mutation;

impl Mutation {
    pub async fn create_tier(db: &DbConn, form_data: tiers::Model) -> Result<tiers::Model, DbErr> {
        let active_model = tiers::ActiveModel {
            title: Set(form_data.title.to_owned()),
            image: Set(form_data.image.to_owned()),
            tier: Set(form_data.tier.to_owned()),
            hori: Set(form_data.hori.to_owned()),
            kind: Set(form_data.kind.to_owned()),
            game: Set(form_data.game.to_owned()),
            user_id: Set(form_data.user_id.to_owned()),
            list_id: Set(form_data.list_id.to_owned()),
            role: Set(form_data.role.to_owned()),
            ..Default::default()
        };
        let res = Tiers::insert(active_model).exec(db).await?;

        Ok(tiers::Model {
            id: res.last_insert_id,
            ..form_data
        })
    }

    pub async fn create_multiple_tiers(db: &DbConn, form_data: Vec<tiers::Model>
        ) -> Result<i32, DbErr> {
        let  copy = form_data.clone();
        let active_model = copy
            .iter()
            .map(|t| tiers::ActiveModel {
                title: Set(t.title.to_owned()),
                image: Set(t.image.to_owned()),
                tier: Set(t.tier.to_owned()),
                kind: Set(t.kind.to_owned()),
                game: Set(t.game.to_owned()),
                hori: Set(t.hori),
                user_id: Set(t.user_id.to_owned()),
                list_id: Set(t.list_id.to_owned()),
                role: Set(t.role.to_owned()),
                ..Default::default()
            });
            
        let insert_result = Tiers::insert_many(active_model).exec(db).await?;
        Ok(insert_result.last_insert_id)
    }

    pub async fn update_tiers_by_id(
        db: &DbConn,
        id: i32,
        form_data: tiers::Model,
    ) -> Result<tiers::Model, DbErr> {
        let _tiers: tiers::ActiveModel = Tiers::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find tiers.".to_owned()))
            .map(Into::into)?;

        tiers::ActiveModel {
            id: Set(id),
            title: Set(form_data.title),
            image: Set(form_data.image),
            tier: Set(form_data.tier),
            hori: Set(form_data.hori),
            kind: Set(form_data.kind),
            game: Set(form_data.game),
            user_id: Set(form_data.user_id),
            list_id: Set(form_data.list_id),
            role: Set(form_data.role),
        }
        .update(db)
        .await
    }

    pub async fn update_multiple_tiers(
        db: &DbConn,
        body: Vec<tiers::Model>,
    ) -> Result<UpdateResult, DbErr> {
        let tasks = body.into_iter().map(|t| tiers::Model {
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
            let res = Self::update_tiers_by_id(db, t.id, t)
                .await
                .map_err(|_| DbErr::Custom("Could not update tier".to_owned()));
            match res {
                Ok(_) => c += 1,
                Err(_) => c = 0,
            }
        }
        Ok(UpdateResult { rows_affected: c })
    }

    pub async fn delete_tiers(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
    let del_rows = Tiers::delete_many().filter(tiers::Column::ListId.eq(id))
        .exec(db)
        .await
        .map_err(|_| DbErr::Custom("Could not update tier".to_owned()))?;
    Ok(DeleteResult { rows_affected: del_rows.rows_affected }.into())
    }

    pub async fn delete_all_tiers(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Tiers::delete_many().exec(db).await
    }
}
