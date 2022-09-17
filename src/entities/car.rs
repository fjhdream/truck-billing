//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "car")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub car_plate_number: String,
    pub team_id: Option<Uuid>,
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
    #[sea_orm(has_many = "super::team_car::Entity")]
    TeamCar,
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Team.def()
    }
}

impl Related<super::team_car::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TeamCar.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
