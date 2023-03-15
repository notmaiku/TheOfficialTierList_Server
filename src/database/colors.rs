//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "colors")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub kind: Option<String>,
    pub start_color: Option<String>,
    pub end_color: Option<String>,
    pub game: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}