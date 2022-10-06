use poem_openapi::{param::Path, payload::Json, ApiResponse, Object, OpenApi, Tags};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use tracing::error;
use uuid::Uuid;

use crate::team_service::service::{TeamCar, TeamUser};
use crate::{entities::team, DATABASE};

use super::service::Team;

#[derive(Tags)]
enum ApiTags {
    Team,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct TeamCreateDTO {
    #[oai(validator(max_length = 128))]
    pub name: String,
}

#[derive(ApiResponse)]
enum CreateTeamResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 500)]
    Error,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct TeamQueryDTO {
    #[oai(validator(max_length = 128))]
    pub team_name: String,

    pub team_id: String,
}

impl From<team::Model> for TeamQueryDTO {
    fn from(team_model: team::Model) -> Self {
        TeamQueryDTO {
            team_name: team_model.team_name,
            team_id: team_model.id.to_string(),
        }
    }
}

#[derive(ApiResponse)]
enum QueryTeamResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<TeamQueryDTO>>),

    #[oai(status = 500)]
    Error,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct TeamUserDTO {
    #[oai(validator(max_length = 128))]
    pub user_id: String,
}

#[derive(ApiResponse)]
enum TeamAddUserResponse {
    #[oai(status = 201)]
    Ok,

    #[oai(status = 500)]
    Error,
}

#[derive(ApiResponse)]
enum TeamDeleteUserResponse {
    #[oai(status = 204)]
    Ok,

    #[oai(status = 500)]
    Error,
}

#[derive(ApiResponse)]
enum TeamGetUserResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<TeamUserResponseEntity>>),

    #[oai(status = 500)]
    Error,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct TeamUserResponseEntity {
    user_id: String,
}

impl From<TeamUser> for TeamUserResponseEntity {
    fn from(team_user: TeamUser) -> Self {
        TeamUserResponseEntity {
            user_id: team_user.user_id,
        }
    }
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct TeamCarDTO {
    #[oai(validator(max_length = 128))]
    pub car_id: String,
}

#[derive(ApiResponse)]
enum TeamAddCarResponse {
    #[oai(status = 201)]
    Ok,

    #[oai(status = 500)]
    Error,
}

#[derive(ApiResponse)]
enum TeamDeleteCarResponse {
    #[oai(status = 204)]
    Ok,

    #[oai(status = 500)]
    Error,
}

#[derive(ApiResponse)]
enum TeamGetCarResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<TeamCarResponseEntity>>),

    #[oai(status = 500)]
    Error,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct TeamCarResponseEntity {
    car_id: String,
}

impl From<TeamCar> for TeamCarResponseEntity {
    fn from(team_car: TeamCar) -> Self {
        TeamCarResponseEntity {
            car_id: team_car.car_id.to_string(),
        }
    }
}

pub struct TeamRouter;

#[OpenApi]
impl TeamRouter {
    #[oai(path = "/user/:user_id/team", method = "post", tag = "ApiTags::Team")]
    async fn create_team(
        &self,
        user_id: Path<String>,
        team: Json<TeamCreateDTO>,
    ) -> CreateTeamResponse {
        let db = DATABASE.get().unwrap();
        let user_id = user_id.0;
        let team_name = team.0.name;
        let result = team::ActiveModel {
            id: Set(Uuid::new_v4()),
            team_name: Set(team_name),
            user_id: Set(user_id),
        }
        .insert(db)
        .await;
        if let Err(_err) = result {
            return CreateTeamResponse::Error;
        }
        CreateTeamResponse::Ok
    }

    #[oai(path = "/user/:user_id/team", method = "get", tag = "ApiTags::Team")]
    async fn query_team(&self, user_id: Path<String>) -> QueryTeamResponse {
        let db = DATABASE.get().unwrap();
        let user_id = user_id.0;
        let query_result = team::Entity::find()
            .filter(team::Column::UserId.eq(user_id.clone()))
            .all(db)
            .await;
        if let Err(err) = query_result {
            error!(
                "query team db error, err is {}, user id is {}",
                err,
                user_id.clone()
            );
            return QueryTeamResponse::Error;
        }
        let query_models = query_result.unwrap();
        let mut response = vec![];
        for model in query_models {
            response.push(model.into());
        }
        return QueryTeamResponse::Ok(Json(response));
    }

    #[oai(path = "/team/:team_id/user", method = "delete", tag = "ApiTags::Team")]
    async fn team_delete_user(
        &self,
        team_id: Path<String>,
        team_dto: Json<TeamUserDTO>,
    ) -> TeamDeleteUserResponse {
        let team_id = team_id.0;
        if let Ok(team) = Team::from_id(team_id).await {
            if let Ok(_res) = team.delete_driver(team_dto.user_id.clone()).await {
                TeamDeleteUserResponse::Ok
            } else {
                TeamDeleteUserResponse::Error
            }
        } else {
            TeamDeleteUserResponse::Error
        }
    }

    #[oai(path = "/team/:team_id/user", method = "post", tag = "ApiTags::Team")]
    async fn team_add_user(
        &self,
        team_id: Path<String>,
        team_dto: Json<TeamUserDTO>,
    ) -> TeamAddUserResponse {
        let team_id = team_id.0;
        if let Ok(team) = Team::from_id(team_id).await {
            if let Ok(_res) = team.add_driver(team_dto.user_id.clone()).await {
                TeamAddUserResponse::Ok
            } else {
                TeamAddUserResponse::Error
            }
        } else {
            TeamAddUserResponse::Error
        }
    }

    #[oai(path = "/team/:team_id/user", method = "get", tag = "ApiTags::Team")]
    async fn team_get_user(&self, team_id: Path<String>) -> TeamGetUserResponse {
        let team_id = team_id.0;
        if let Ok(team) = Team::from_id(team_id).await {
            if let Ok(res) = team.get_drivers().await {
                let mut response: Vec<TeamUserResponseEntity> = vec![];
                for team_user in res {
                    response.push(team_user.into());
                }
                TeamGetUserResponse::Ok(Json(response))
            } else {
                TeamGetUserResponse::Error
            }
        } else {
            TeamGetUserResponse::Error
        }
    }

    #[oai(path = "/team/:team_id/car", method = "post", tag = "ApiTags::Team")]
    async fn team_add_car(
        &self,
        team_id: Path<String>,
        team_dto: Json<TeamCarDTO>,
    ) -> TeamAddCarResponse {
        let team_id = team_id.0;
        if let Ok(team) = Team::from_id(team_id).await {
            if let Ok(_res) = team.add_car(team_dto.car_id.clone()).await {
                TeamAddCarResponse::Ok
            } else {
                TeamAddCarResponse::Error
            }
        } else {
            TeamAddCarResponse::Error
        }
    }

    #[oai(path = "/team/:team_id/car", method = "delete", tag = "ApiTags::Team")]
    async fn team_delete_car(
        &self,
        team_id: Path<String>,
        team_dto: Json<TeamCarDTO>,
    ) -> TeamDeleteCarResponse {
        let team_id = team_id.0;
        if let Ok(team) = Team::from_id(team_id).await {
            if let Ok(_res) = team.add_car(team_dto.car_id.clone()).await {
                TeamDeleteCarResponse::Ok
            } else {
                TeamDeleteCarResponse::Error
            }
        } else {
            TeamDeleteCarResponse::Error
        }
    }

    #[oai(path = "/team/:team_id/car", method = "get", tag = "ApiTags::Team")]
    async fn team_get_car(&self, team_id: Path<String>) -> TeamGetCarResponse {
        let team_id = team_id.0;
        if let Ok(team) = Team::from_id(team_id).await {
            if let Ok(res) = team.get_cars().await {
                let mut response: Vec<TeamCarResponseEntity> = vec![];
                for team_car in res {
                    response.push(team_car.into());
                }
                TeamGetCarResponse::Ok(Json(response))
            } else {
                TeamGetCarResponse::Error
            }
        } else {
            TeamGetCarResponse::Error
        }
    }
}
