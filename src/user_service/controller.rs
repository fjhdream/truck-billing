use std::{env, vec};

use crate::entities::{role, user};
use crate::DATABASE;
use poem::web::Path;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi, Tags};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use tracing::info;
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

#[derive(Debug, Object, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct UserWxLoginDTO {
    pub code: String,
}

#[derive(Debug, Object, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct WxLoginDTO {
    pub openid: Option<String>,
    pub session_key: Option<String>,
    pub unionid: Option<String>,
    pub errcode: Option<i32>,
    pub errmsg: Option<String>,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserWxLoginResponseDTO {
    pub code: String,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserWxLoginErrorResponse {
    pub err_code: Option<String>,
    pub err_msg: Option<String>,
}

#[derive(ApiResponse)]
enum UserLoginResponse {
    #[oai(status = 200)]
    Ok(Json<UserWxLoginResponseDTO>),

    #[oai(status = 500)]
    Error(Json<UserWxLoginErrorResponse>),
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

    #[oai(path = "/user/login", method = "post", tag = "ApiTags::User")]
    async fn login(&self, user: Json<UserWxLoginDTO>) -> UserLoginResponse {
        let (app_id, secret) = (env::var("APP_ID").unwrap(), env::var("APP_SECRET").unwrap());
        let code = user.0.code;
        let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={app_id}&secret={secret}&js_code={code}&grant_type=authorization_code", 
                          app_id=app_id, secret = secret, code = code);
        info!("query url is {}", url);
        let resp = reqwest::get(url).await.unwrap();
        if let reqwest::StatusCode::OK = resp.status() {
            match resp.json::<WxLoginDTO>().await {
                Ok(wx_resp) => match &wx_resp.errcode {
                    Some(errcode) => match errcode {
                        -1 => {
                            error!(
                                "wx system is busy, err is {}",
                                wx_resp.errmsg.clone().unwrap_or("empty".to_owned())
                            );
                            UserLoginResponse::Error(Json(UserWxLoginErrorResponse {
                                err_code: Some(errcode.to_string()),
                                err_msg: wx_resp.errmsg.clone(),
                            }))
                        }
                        0 => UserLoginResponse::Ok(Json(UserWxLoginResponseDTO {
                            code: wx_resp.openid.unwrap(),
                        })),
                        40029 => {
                            error!(
                                "code is can not be used. err or msg is {}",
                                wx_resp.errmsg.clone().unwrap_or("empty".to_owned())
                            );
                            UserLoginResponse::Error(Json(UserWxLoginErrorResponse {
                                err_code: Some(errcode.to_string()),
                                err_msg: wx_resp.errmsg,
                            }))
                        }
                        45011 => {
                            error!(
                                "call api too frequently. err msg is {}",
                                wx_resp.errmsg.clone().unwrap_or("empty".to_owned())
                            );
                            UserLoginResponse::Error(Json(UserWxLoginErrorResponse {
                                err_code: Some(errcode.to_string()),
                                err_msg: wx_resp.errmsg,
                            }))
                        }
                        40226 => {
                            error!(
                                "high risk level user. err msg is {}",
                                wx_resp.errmsg.clone().unwrap_or("empty".to_owned())
                            );
                            UserLoginResponse::Error(Json(UserWxLoginErrorResponse {
                                err_code: Some(errcode.to_string()),
                                err_msg: wx_resp.errmsg,
                            }))
                        }
                        _ => {
                            error!(
                                "wx unused error code. err code is {}, err msg is {}",
                                errcode,
                                wx_resp.errmsg.clone().unwrap_or("empty".to_owned())
                            );
                            UserLoginResponse::Error(Json(UserWxLoginErrorResponse {
                                err_code: Some(errcode.to_string()),
                                err_msg: wx_resp.errmsg,
                            }))
                        }
                    },
                    None => match wx_resp.openid {
                        Some(openid) => {
                            UserLoginResponse::Ok(Json(UserWxLoginResponseDTO { code: openid }))
                        }
                        None => UserLoginResponse::Error(Json(UserWxLoginErrorResponse {
                            err_code: None,
                            err_msg: None,
                        })),
                    },
                },
                Err(err) => {
                    error!("response is not correctly deserialize. error is {}", err);
                    UserLoginResponse::Error(Json(UserWxLoginErrorResponse {
                        err_code: None,
                        err_msg: None,
                    }))
                }
            }
        } else {
            error!("Wx login response failed.");
            UserLoginResponse::Error(Json(UserWxLoginErrorResponse {
                err_code: None,
                err_msg: None,
            }))
        }
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
