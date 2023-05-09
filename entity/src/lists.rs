use sea_orm::{entity::prelude::*, DeleteMany};
use async_graphql::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[graphql(concrete(name = "Lists", params()))]
#[sea_orm(table_name = "lists")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: Option<String>,
    pub user_id: Option<String>,
    pub game: Option<String>,
    // pub created_at: Option<DateTime>,
    // pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::tiers::Entity")]
    Tiers,
}

impl Related<super::tiers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tiers.def()
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