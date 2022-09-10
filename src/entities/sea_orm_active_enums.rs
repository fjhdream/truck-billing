//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "item_type")]
pub enum ItemType {
    #[sea_orm(string_value = "BASIC")]
    Basic,
    #[sea_orm(string_value = "COUSTOM")]
    Coustom,
}
#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "role_type")]
pub enum RoleType {
    #[sea_orm(string_value = "ADMIN")]
    Admin,
    #[sea_orm(string_value = "DRIVER")]
    Driver,
    #[sea_orm(string_value = "OWNER")]
    Owner,
}