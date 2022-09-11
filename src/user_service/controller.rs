use crate::{entities::user, DATABASE};
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi, Tags};
use sea_orm::{ActiveModelTrait, Set};
use tracing::log::warn;

use super::service::UserAggregate;

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    User,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserDTO {
    /// Id
    #[oai(validator(max_length = 128))]
    pub id: String,
    /// Name
    #[oai(validator(max_length = 128))]
    pub name: String,

    #[oai]
    pub avatar_url: Option<String>,
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
    #[oai(path = "/user", method = "post", tag = "ApiTags::User")]
    async fn create(&self, user: Json<UserDTO>) -> CreateUserResponse {
        let user_dto = user.0;
        let user_aggregate: UserAggregate = user_dto.into();
        let create_result = user_aggregate.create_user().await;
        if let Err(err) = create_result {
            warn!("create user met error {}", err);
            return CreateUserResponse::Error;
        }
        return CreateUserResponse::Ok(Json(create_result.unwrap()));
    }
}
