//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "billing_item")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub billing_id: Option<Uuid>,
    pub cost: Decimal,
    pub item_id: Option<Uuid>,
    pub time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::billing::Entity",
        from = "Column::BillingId",
        to = "super::billing::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Billing,
    #[sea_orm(
        belongs_to = "super::item::Entity",
        from = "Column::ItemId",
        to = "super::item::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Item,
}

impl Related<super::billing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Billing.def()
    }
}

impl Related<super::item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
