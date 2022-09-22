use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set};
use std::{error::Error, vec};
use tracing::instrument;

use crate::{
    entities::{role, sea_orm_active_enums::RoleType, user},
    role_service::service::UserRoleAggregate,
    DATABASE,
};

use super::controller::UserDTO;

#[derive(Debug)]
pub enum UserError {
    EmptyUserError,
    DbError,
}

impl Error for UserError {}

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::EmptyUserError => {
                write!(f, "There is nothing with this man.")
            }
            UserError::DbError => {
                write!(f, "Connect with Db Error")
            }
        }
    }
}

impl From<DbErr> for UserError {
    fn from(_: DbErr) -> Self {
        UserError::DbError
    }
}

impl ToString for RoleType {
    fn to_string(&self) -> String {
        match self {
            RoleType::Admin => "ADMIN".to_owned(),
            RoleType::Driver => "DRIVER".to_owned(),
            RoleType::Owner => "OWNER".to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct UserAggregate {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

impl UserAggregate {
    #[instrument]
    pub async fn from_user_id(user_id: String) -> Result<UserAggregate, UserError> {
        let db = DATABASE.get().unwrap();
        let query_result = user::Entity::find_by_id(user_id).one(db).await?;
        if query_result.is_none() {
            return Err(UserError::EmptyUserError);
        }
        let query_model = query_result.unwrap();
        let user = UserAggregate {
            id: query_model.id,
            name: query_model.user_name,
            avatar_url: query_model.avatar_url,
        };
        return Ok(user);
    }

    #[instrument]
    pub async fn create_user(self) -> Result<String, UserError> {
        let db = DATABASE.get().unwrap();
        user::ActiveModel {
            id: Set(self.id.to_owned()),
            user_name: Set(self.name.to_owned()),
            avatar_url: Set(self.avatar_url.to_owned()),
        }
        .insert(db)
        .await?;

        let role = UserRoleAggregate::default_from_user_id(self.id.clone());
        role.save().await?;
        Ok(self.id.clone())
    }

    #[instrument]
    pub async fn get_user_role(&self) -> Result<UserAggregateRole, UserError> {
        let db = DATABASE.get().unwrap();
        let query_result = role::Entity::find()
            .filter(role::Column::UserId.eq(self.id.clone()))
            .all(db)
            .await?;
        let mut role_array: Vec<String> = vec![];
        for query_model in query_result {
            role_array.push(query_model.r#type.to_string());
        }
        Ok(UserAggregateRole { roles: role_array })
    }
}

impl From<UserDTO> for UserAggregate {
    fn from(user: UserDTO) -> Self {
        Self {
            id: user.id,
            name: user.name,
            avatar_url: user.avatar_url,
        }
    }
}

pub struct UserAggregateRole {
    pub roles: Vec<String>,
}
