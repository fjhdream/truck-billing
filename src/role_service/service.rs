use std::str::FromStr;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    #[instrument]
    pub async fn add_user_role(self) -> Result<Uuid, DbErr> {
        let db = DATABASE.get().unwrap();
        if self.role_type == UserRoleType::None {
            info!("[UserRoleService] convert all other type to Driver")
        }
        let insert_result = role::ActiveModel {
            id: Set(Uuid::new_v4()),
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
