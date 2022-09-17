use std::error::Error;

use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, ModelTrait, QueryFilter, Set};
use tracing::{info, instrument, warn};
use uuid::Uuid;

use crate::{
    entities::{team, team_driver},
    DATABASE,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TeamError {
    QueryTeamError(String),
    DbError,
}

impl From<DbErr> for TeamError {
    fn from(_: DbErr) -> Self {
        TeamError::DbError
    }
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
            TeamError::DbError => {
                write!(f, "Database query error")
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

#[derive(Debug)]
pub struct TeamUser {
    pub user_id: String,
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
        let _insert_result = team_driver::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            team_id: Set(team_id),
        }
        .insert(db)
        .await?;
        Ok(())
    }

    #[instrument]
    pub async fn delete_driver(&self, user_id: String) -> Result<(), TeamError> {
        let _team_id = self.id;
        let db = DATABASE.get().unwrap();
        let query_result = team_driver::Entity::find()
            .filter(team_driver::Column::TeamId.eq(self.id))
            .filter(team_driver::Column::UserId.eq(user_id))
            .one(db)
            .await?;
        if let Some(query_model) = query_result {
            let delete_result = query_model.delete(db).await?;
            let affect_row = delete_result.rows_affected;
            info!("Delete affected row is {}", affect_row);
        }
        Ok(())
    }

    #[instrument]
    pub async fn get_drivers(&self) -> Result<Vec<TeamUser>, TeamError> {
        let team_id = self.id;
        let db = DATABASE.get().unwrap();
        let query_result = team_driver::Entity::find()
            .filter(team_driver::Column::TeamId.eq(team_id))
            .all(db)
            .await?;

        let mut res: Vec<TeamUser> = vec![];
        for query in query_result {
            let team_user = TeamUser {
                user_id: query.user_id,
            };
            res.push(team_user);
        }
        Ok(res)
    }
}
