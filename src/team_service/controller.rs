use poem_openapi::{param::Path, payload::Json, ApiResponse, Object, OpenApi, Tags};
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

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
pub struct TeamUserDTO {
    #[oai(validator(max_length = 128))]
    pub user_id: String,
}

#[derive(ApiResponse)]
enum TeamAddUserResponse {
    #[oai(status = 200)]
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
    #[oai(status = 201)]
    Ok,

    #[oai(status = 500)]
    Error,
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
}
