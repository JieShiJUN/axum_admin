//! SeaORM Entity. Generated by sea-orm-codegen 0.4.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "sys_dict_type")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub dict_id: String,
    pub dict_name: String,
    #[sea_orm(unique)]
    pub dict_type: String,
    pub status: i8,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub remark: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
