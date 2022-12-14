//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "team")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub team_name: String,
    pub user_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
    #[sea_orm(has_many = "super::item::Entity")]
    Item,
    #[sea_orm(has_many = "super::billing::Entity")]
    Billing,
    #[sea_orm(has_many = "super::team_driver::Entity")]
    TeamDriver,
    #[sea_orm(has_many = "super::team_car::Entity")]
    TeamCar,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Item.def()
    }
}

impl Related<super::billing::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Billing.def()
    }
}

impl Related<super::team_driver::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamDriver.def()
    }
}

impl Related<super::team_car::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamCar.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
