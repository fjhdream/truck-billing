use crate::{entities::user, DATABASE};
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi, Tags};
use sea_orm::{ActiveModelTrait, Set};
use tracing::log::warn;

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    User,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct UserDTO {
    /// Id
    #[oai(validator(max_length = 128))]
    id: String,
    /// Name
    #[oai(validator(max_length = 128))]
    name: String,

    #[oai]
    avatar_url: Option<String>,
}

#[derive(ApiResponse)]
enum CreateUserResponse {
    #[oai(status = 200)]
    Ok(Json<String>),

    #[oai(status = 500)]
    Error,
}

pub struct UserRouter;

#[OpenApi]
impl UserRouter {
    #[oai(path = "/", method = "post", tag = "ApiTags::User")]
    async fn create(&self, user: Json<UserDTO>) -> CreateUserResponse {
        let db = DATABASE.get().unwrap();
        let insert_result = user::ActiveModel {
            id: Set(user.id.to_owned()),
            user_name: Set(user.name.to_owned()),
            avatar_url: Set(user.avatar_url.to_owned()),
        }
        .insert(db)
        .await;
        if let Err(err) = insert_result {
            warn!("insert user met error {}", err);
            return CreateUserResponse::Error;
        }
        return CreateUserResponse::Ok(Json(user.0.id));
    }
}
