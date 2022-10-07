use poem_openapi::{param::Path, payload::Json, ApiResponse, Object, OpenApi, Tags};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, IntoActiveModel};
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

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct TeamDeleteDTO {
    #[oai(validator(max_length = 128))]
    pub team_id: String,
}

#[derive(ApiResponse)]
enum CreateTeamResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 500)]
    Error,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct TeamEntityDTO {
    #[oai(validator(max_length = 128))]
    pub team_name: String,

    pub team_id: String,
}

impl From<team::Model> for TeamEntityDTO {
    fn from(team_model: team::Model) -> Self {
        TeamEntityDTO {
            team_name: team_model.team_name,
            team_id: team_model.id.to_string(),
        }
    }
}

#[derive(ApiResponse)]
enum QueryTeamResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<TeamEntityDTO>>),

    #[oai(status = 500)]
    Error,
}

#[derive(ApiResponse)]
enum UpdateTeamResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 500)]
    Error,
}

#[derive(ApiResponse)]
enum DeleteTeamResponse {
    #[oai(status = 204)]
    Ok,

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
pub struct TeamCarCreateDTO {
    #[oai(validator(max_length = 128))]
    pub car_plate_number: String,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct TeamCarDeleteDTO {
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
    car_plate_number: String,
}

impl From<TeamCar> for TeamCarResponseEntity {
    fn from(team_car: TeamCar) -> Self {
        TeamCarResponseEntity {
            car_id: team_car.car_id.to_string(),
            car_plate_number: team_car.car_plate_number,
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

    #[oai(path = "/user/:user_id/team", method = "put", tag = "ApiTags::Team")]
    async fn update_team(
        &self,
        user_id: Path<String>,
        team: Json<TeamEntityDTO>,
    ) -> UpdateTeamResponse {
        let db = DATABASE.get().unwrap();
        let team_id = team.team_id.clone();
        let team_id_uuid = Uuid::parse_str(&team_id);
        if let Err(err) = team_id_uuid {
            error!("parse id to uuid error, err is {}", err);
            return UpdateTeamResponse::Error;
        }
        let team_id_uuid = team_id_uuid.unwrap();
        let model_result = team::Entity::find_by_id(team_id_uuid).one(db).await;
        if let Err(err) = model_result {
            error!("Query Team error, error is {}, team id is {}", err, team_id.clone());
            return UpdateTeamResponse::Error;
        }
        let model = model_result.unwrap();
        if let Some(team_model) = model {
            let mut team_active_model = team_model.into_active_model();
            team_active_model.team_name = Set(team.team_name.clone());
            let update_result = team_active_model.update(db).await;
            if update_result.is_err() {
                error!("update team name error");
                return UpdateTeamResponse::Error;
            }
            return UpdateTeamResponse::Ok;
        }
        UpdateTeamResponse::Error
    }

    #[oai(path = "/user/:user_id/team", method = "delete", tag = "ApiTags::Team")]
    async fn delete_team(
        &self,
        user_id: Path<String>,
        team: Json<TeamDeleteDTO>,
    ) -> DeleteTeamResponse {
        let db = DATABASE.get().unwrap();
        let team_id = team.0.team_id;
        let team_aggreagte = Team::from_id(team_id).await;
        if let Err(err) = team_aggreagte {
            error!("Get team form db error, err is {}", err);
            return DeleteTeamResponse::Error;
        }
        let team_aggreagte = team_aggreagte.unwrap();
        let team_delte_result = team_aggreagte.delete().await;
        if let Err(err) = team_delte_result {
            error!("Team delete error. Error is {}", err);
            return  DeleteTeamResponse::Error;
        }
        DeleteTeamResponse::Ok
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
        team_dto: Json<TeamCarCreateDTO>,
    ) -> TeamAddCarResponse {
        let team_id = team_id.0;
        let team_result = Team::from_id(team_id).await;
        if let Ok(team) = team_result {
            let add_car_result = team.add_car(team_dto.car_plate_number.clone()).await;
            if let Ok(_res) = add_car_result {
                TeamAddCarResponse::Ok
            } else {
                error!("add car error");
                TeamAddCarResponse::Error
            }
        } else {
            error!("get team error, error is {}", team_result.unwrap_err());
            TeamAddCarResponse::Error
        }
    }

    #[oai(path = "/team/:team_id/car", method = "delete", tag = "ApiTags::Team")]
    async fn team_delete_car(
        &self,
        team_id: Path<String>,
        team_car_dto: Json<TeamCarDeleteDTO>,
    ) -> TeamDeleteCarResponse {
        let team_id = team_id.0;
        if let Ok(team) = Team::from_id(team_id).await {
            if let Ok(_res) = team.delete_car(team_car_dto.car_id.clone()).await {
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
