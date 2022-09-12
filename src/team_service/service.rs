use std::error::Error;

use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use tracing::{instrument, warn};
use uuid::Uuid;

use crate::{
    entities::{team, team_driver},
    DATABASE,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TeamError {
    QueryTeamError(String),
    DbQueryError,
    DbInsertError,
}

impl Error for TeamError {}

impl std::fmt::Display for TeamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamError::QueryTeamError(team_id) => {
                write!(
                    f,
                    "Database get info error, empty team info. team id is {}",
                    team_id
                )
            }
            TeamError::DbQueryError => {
                write!(f, "Database query error")
            }
            TeamError::DbInsertError => {
                write!(f, "Database insert error")
            }
        }
    }
}

#[derive(Debug)]
pub struct Team {
    id: Uuid,
    name: String,
    user_id: String,
}

impl Team {
    #[instrument]
    pub async fn from_id(id: String) -> Result<Self, TeamError> {
        let db = DATABASE.get().unwrap();
        if let Ok(team_id) = Uuid::parse_str(&id) {
            if let Ok(query_result) = team::Entity::find_by_id(team_id).one(db).await {
                if let Some(team_model) = query_result {
                    Ok(Team {
                        id: team_model.id,
                        name: team_model.team_name,
                        user_id: team_model.user_id,
                    })
                } else {
                    Err(TeamError::QueryTeamError(id))
                }
            } else {
                Err(TeamError::QueryTeamError(id))
            }
        } else {
            warn!("uuid parse error, id is {}", id);
            Err(TeamError::QueryTeamError(id))
        }
    }

    #[instrument]
    pub async fn add_driver(&self, user_id: String) -> Result<(), TeamError> {
        let team_id = self.id;
        let db = DATABASE.get().unwrap();
        let insert_result = team_driver::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            team_id: Set(team_id),
        }
        .insert(db)
        .await;
        if let Ok(_insert) = insert_result {
            Ok(())
        } else {
            Err(TeamError::DbInsertError)
        }
    }
}
