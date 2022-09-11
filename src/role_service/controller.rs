use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi, Tags};

use super::service::UserRoleAggregate;

#[derive(Tags)]
enum ApiTags {
    /// Operations about user role
    UserRole,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserRoleDTO {
    /// UserId
    #[oai(validator(max_length = 128))]
    pub user_id: String,

    #[oai]
    pub role_type: String,
}

#[derive(ApiResponse)]
enum AddUserRoleResponse {
    #[oai(status = 200)]
    Ok(Json<String>),

    #[oai(status = 500)]
    Error,
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
}
