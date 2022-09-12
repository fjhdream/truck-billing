use sea_orm::{ActiveModelTrait, DbErr, Set};
use tracing::instrument;

use crate::{entities::user, role_service::service::UserRoleAggregate, DATABASE};

use super::controller::UserDTO;

#[derive(Debug)]
pub struct UserAggregate {
    id: String,
    name: String,
    avatar_url: Option<String>,
}

impl UserAggregate {
    #[instrument]
    pub async fn create_user(self) -> Result<String, DbErr> {
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
