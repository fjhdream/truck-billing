use poem_openapi::{param::Path, payload::Json, ApiResponse, Object, OpenApi, Tags};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{entities::role, DATABASE};

use super::service::{UserRoleAggregate, UserRoleType};

#[derive(Tags)]
enum ApiTags {
    /// Operations about user role
    UserRole,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserRoleDTO {
    #[oai]
    pub role_type: String,

    /// UserId
    #[oai(validator(max_length = 128))]
    pub user_id: String,
}

#[derive(ApiResponse)]
enum AddUserRoleResponse {
    #[oai(status = 200)]
    Ok(Json<String>),

    #[oai(status = 500)]
    Error,
}

#[derive(ApiResponse)]
enum GetUserRoleResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<UserRoleResponseEntity>>),

    #[oai(status = 500)]
    Error,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct UserRoleResponseEntity {
    user_id: String,
    role_type: UserRoleType,
}

impl From<role::Model> for UserRoleResponseEntity {
    fn from(role: role::Model) -> Self {
        UserRoleResponseEntity {
            user_id: role.user_id,
            role_type: role.r#type.into(),
        }
    }
}

pub struct UserRoleRouter;

#[OpenApi]
impl UserRoleRouter {
    #[oai(path = "/user/role", method = "post", tag = "ApiTags::UserRole")]
    async fn create(&self, user: Json<UserRoleDTO>) -> AddUserRoleResponse {
        let user_role_dto = user.0;
        let user_aggregate: UserRoleAggregate = user_role_dto.into();
        let add_result = user_aggregate.add_user_role().await;
        if let Ok(id) = add_result {
            return AddUserRoleResponse::Ok(Json(id.to_string()));
        }
        AddUserRoleResponse::Error
    }

    #[oai(
        path = "/user/role/:user_id",
        method = "get",
        tag = "ApiTags::UserRole"
    )]
    async fn get(&self, user_id: Path<String>) -> GetUserRoleResponse {
        let db = DATABASE.get().unwrap();
        let user_id = user_id.0;
        let user_entity_result = role::Entity::find()
            .filter(role::Column::UserId.eq(user_id))
            .all(db)
            .await;
        if let Ok(models) = user_entity_result {
            let mut response: Vec<UserRoleResponseEntity> = vec![];
            for model in models {
                response.push(model.into())
            }
            GetUserRoleResponse::Ok(Json(response))
        } else {
            GetUserRoleResponse::Error
        }
    }
}
