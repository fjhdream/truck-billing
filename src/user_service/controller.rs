use std::vec;

use crate::{
    entities::{role, user},
    role_service::service::UserRoleType,
    DATABASE,
};
use poem::web::Path;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi, Tags};
use sea_orm::{EntityTrait, ModelTrait};
use tracing::{log::error, log::warn};

use super::service::{UserAggregate, UserAggregateRole};

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    User,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserCreateDTO {
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
pub struct UserQueryDTO {
    pub id: String,

    pub name: String,

    pub avatar_url: Option<String>,

    pub roles: Option<Vec<RoleDTO>>,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct RoleDTO {
    pub role_id: String,

    pub role_type: String,
}

impl From<UserAggregateRole> for RoleDTO {
    fn from(user_role: UserAggregateRole) -> Self {
        RoleDTO {
            role_id: user_role.id,
            role_type: user_role.role,
        }
    }
}

impl From<role::Model> for RoleDTO {
    fn from(role: role::Model) -> Self {
        RoleDTO {
            role_id: role.id.to_string(),
            role_type: role.r#type.to_string(),
        }
    }
}

#[derive(ApiResponse)]
enum GetUserResponse {
    #[oai(status = 200)]
    Ok(Json<UserQueryDTO>),

    #[oai(status = 500)]
    Error,
}

#[derive(ApiResponse)]
enum GetAllUserResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<UserQueryDTO>>),

    #[oai(status = 401)]
    Forbidden,

    #[oai(status = 500)]
    Error,
}

pub struct UserRouter;

#[OpenApi]
impl UserRouter {
    #[oai(path = "/user", method = "post", tag = "ApiTags::User")]
    async fn create(&self, user: Json<UserCreateDTO>) -> CreateUserResponse {
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
        let query_roles = query_role_result.unwrap();
        let mut roles = vec![];
        for role in query_roles {
            roles.push(role.into());
        }
        let user_entity = UserQueryDTO {
            id: user_aggregate.id,
            name: user_aggregate.name,
            avatar_url: user_aggregate.avatar_url,
            roles: Some(roles),
        };
        GetUserResponse::Ok(Json(user_entity))
    }

    #[oai(path = "/user/:user_id/get", method = "get", tag = "ApiTags::User")]
    async fn get_users(&self, user_id: Path<String>) -> GetAllUserResponse {
        let db = DATABASE.get().unwrap();
        let user_id = user_id.0;
        let user_aggregate_result = UserAggregate::from_user_id(user_id.clone()).await;
        if let Err(err) = user_aggregate_result {
            error!(
                "Get user form db error {}! user is is {}",
                err,
                user_id.clone()
            );
            return GetAllUserResponse::Error;
        }
        let user_aggregate = user_aggregate_result.unwrap();
        let is_user_admin_result = user_aggregate.is_admin().await;
        if let Err(err) = is_user_admin_result {
            error!(
                "Get user form db error {}! user is is {}",
                err,
                user_id.clone()
            );
            return GetAllUserResponse::Error;
        }
        if !is_user_admin_result.unwrap() {
            warn!("User ({}) is not ADMIN.", user_id.clone());
            return GetAllUserResponse::Forbidden;
        }
        let user_roles_result = user::Entity::find()
            .find_with_related(role::Entity)
            .all(db)
            .await;
        if user_roles_result.is_err() {
            error!("query all user form db error!");
            return GetAllUserResponse::Error;
        }
        let user_roles = user_roles_result.unwrap();
        let mut response = vec![];
        for (user, roles) in user_roles {
            let mut role_array = vec![];
            for role in roles {
                role_array.push(role.into());
            }
            response.push(UserQueryDTO {
                id: user.id,
                name: user.user_name,
                avatar_url: user.avatar_url,
                roles: Some(role_array),
            })
        }
        GetAllUserResponse::Ok(Json(response))
    }
}
