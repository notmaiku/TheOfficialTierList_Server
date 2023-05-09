use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
// #[sea_orm(table_name = "notes")]
// #[graphql(concrete(name = "Note", params()))]
// pub struct Model {
//     #[sea_orm(primary_key)]
//     #[serde(skip_deserializing)]
//     pub id: i32,
//     pub title: String,
//     pub text: String,
// }

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
// pub enum Relation {}

// impl ActiveModelBehavior for ActiveModel {}

// impl Entity {
//     pub fn find_by_id(id: i32) -> Select<Entity> {
//         Self::find().filter(Column::Id.eq(id))
//     }

//     pub fn find_by_title(title: &str) -> Select<Entity> {
//         Self::find().filter(Column::Title.eq(title))
//     }

//     pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
//         Self::delete_many().filter(Column::Id.eq(id))
//     }
// }

#[sea_orm(table_name = "tiers")]
#[graphql(concrete(name = "Tiers", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub image: Option<String>,
    pub tier: String,
    pub kind: Option<String>,
    pub game: String,
    pub hori: Option<i32>,
    pub user_id: Option<String>,
    pub list_id: Option<i32>,
    pub role: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
       #[sea_orm(has_one = "super::lists::Entity")]
    Lists,
}

impl Related<super::lists::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lists.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_title(title: &str) -> Select<Entity> {
        Self::find().filter(Column::Title.eq(title))
    }

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}