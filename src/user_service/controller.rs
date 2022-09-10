use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse, Object, OpenApi, Tags,
};

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
}

pub struct UserRouter;

#[OpenApi]
impl UserRouter {
    #[oai(path = "/", method = "post", tag = "ApiTags::User")]
    async fn create(&self, user: Json<UserDTO>) -> CreateUserResponse {
        return CreateUserResponse::Ok(Json(user.0.id));
    }
}
