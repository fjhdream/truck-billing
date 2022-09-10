//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use super::sea_orm_active_enums::ItemType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub r#type: ItemType,
    pub name: String,
    pub team_id: Option<Uuid>,
    #[sea_orm(column_type = "Text", nullable)]
    pub icon_url: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::team::Entity",
        from = "Column::TeamId",
        to = "super::team::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Team,
    #[sea_orm(has_many = "super::billing_item::Entity")]
    BillingItem,
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl Related<super::billing_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BillingItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}