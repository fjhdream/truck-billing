use sea_orm::{ActiveModelTrait, DbErr, Set};
use tracing::instrument;

use crate::{entities::user, DATABASE};

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
        let insert_result = user::ActiveModel {
            id: Set(self.id.to_owned()),
            user_name: Set(self.name.to_owned()),
            avatar_url: Set(self.avatar_url.to_owned()),
        }
        .insert(db)
        .await;
        if let Err(err) = insert_result {
            return Err(err);
        }
        Ok(self.id)
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
