use poem_openapi::{param::Path, payload::Json, ApiResponse, Object, OpenApi, Tags};
use sea_orm::{ActiveModelTrait, Set};
use uuid::Uuid;

use crate::{entities::team, DATABASE};

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    Team,
}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct TeamDTO {
    /// Name
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

pub struct TeamRouter;

#[OpenApi]
impl TeamRouter {
    #[oai(path = "/user/:user_id/team", method = "post", tag = "ApiTags::Team")]
    async fn create_team(&self, user_id: Path<String>, team: Json<TeamDTO>) -> CreateTeamResponse {
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
}
