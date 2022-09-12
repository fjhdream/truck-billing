use std::str::FromStr;

use poem_openapi::Enum;
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, Set};

use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    entities::{role, sea_orm_active_enums::RoleType},
    DATABASE,
};

use super::controller::UserRoleDTO;

#[derive(Debug)]
pub struct UserRoleAggregate {
    id: Uuid,
    user_id: String,
    role_type: UserRoleType,
}

#[derive(Debug, Enum, Clone, Eq, PartialEq)]
pub enum UserRoleType {
    Admin,
    Driver,
    Owner,
    None,
}

impl Into<RoleType> for UserRoleType {
    fn into(self) -> RoleType {
        match self {
            UserRoleType::Admin => RoleType::Admin,
            UserRoleType::Driver => RoleType::Driver,
            UserRoleType::Owner => RoleType::Owner,
            UserRoleType::None => RoleType::Driver,
        }
    }
}

impl From<RoleType> for UserRoleType {
    fn from(role_type: RoleType) -> Self {
        match role_type {
            RoleType::Admin => UserRoleType::Admin,
            RoleType::Driver => UserRoleType::Driver,
            RoleType::Owner => UserRoleType::Owner,
        }
    }
}

impl FromStr for UserRoleType {
    type Err = ();

    fn from_str(input: &str) -> Result<UserRoleType, Self::Err> {
        match input {
            "Admin" => Ok(UserRoleType::Admin),
            "Owner" => Ok(UserRoleType::Owner),
            "Driver" => Ok(UserRoleType::Driver),
            _ => Ok(UserRoleType::None),
        }
    }
}

impl UserRoleAggregate {
    pub fn from_user_id(user_id: String) -> Self {
        UserRoleAggregate {
            id: Uuid::new_v4(),
            user_id,
            role_type: UserRoleType::Driver,
        }
    }

    #[instrument]
    pub async fn add_user_role(self) -> Result<Uuid, DbErr> {
        let db = DATABASE.get().unwrap();
        if self.role_type == UserRoleType::None {
            info!("[UserRoleService] convert all other type to Driver")
        }
        let insert_result = role::ActiveModel {
            id: Set(self.id),
            user_id: Set(self.user_id.clone()),
            r#type: ActiveValue::Set(self.role_type.into()),
        }
        .insert(db)
        .await;
        if let Err(err) = insert_result {
            return Err(err);
        }
        Ok(self.id)
    }

    #[instrument]
    pub async fn save(self) -> Result<Uuid, DbErr> {
        let db = DATABASE.get().unwrap();
        let insert_result = role::ActiveModel {
            id: Set(self.id),
            user_id: Set(self.user_id.clone()),
            r#type: ActiveValue::Set(self.role_type.into()),
        }
        .insert(db)
        .await?;
        Ok(insert_result.id)
    }

    pub async fn get(self) -> Result<(), DbErr> {
        Ok(())
    }
}

impl From<UserRoleDTO> for UserRoleAggregate {
    fn from(user: UserRoleDTO) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: user.user_id,
            role_type: UserRoleType::from_str(&user.role_type).unwrap(),
        }
    }
}
