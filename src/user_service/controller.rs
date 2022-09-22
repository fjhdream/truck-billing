use poem::web::Path;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi, Tags};
use tracing::{log::error, log::warn};

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
    #[oai(status = 201)]
    Ok(Json<String>),

    #[oai(status = 500)]
    Error,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserEntity {
    pub id: String,

    pub name: String,

    pub avatar_url: Option<String>,

    pub role: Vec<String>,
}

#[derive(ApiResponse)]
enum GetUserResponse {
    #[oai(status = 200)]
    Ok(Json<UserEntity>),

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
        CreateUserResponse::Ok(Json(create_result.unwrap()))
    }

    #[oai(path = "/user/:user_id", method = "get", tag = "ApiTags::User")]
    async fn get(&self, user_id: Path<String>) -> GetUserResponse {
        let user_id = user_id.0;
        let user_aggregate_result = UserAggregate::from_user_id(user_id.clone()).await;
        if let Err(err) = user_aggregate_result {
            error!(
                "Get user form db error {}! user is is {}",
                err,
                user_id.clone()
            );
            return GetUserResponse::Error;
        }
        let user_aggregate = user_aggregate_result.unwrap();
        let query_role_result = user_aggregate.get_user_role().await;
        if let Err(err) = query_role_result {
            warn!("create user met error {}", err);
            return GetUserResponse::Error;
        }
        let query_role = query_role_result.unwrap();
        let user_entity = UserEntity {
            id: user_aggregate.id,
            name: user_aggregate.name,
            avatar_url: user_aggregate.avatar_url,
            role: query_role.roles,
        };
        GetUserResponse::Ok(Json(user_entity))
    }
}
