use std::result;

use async_trait::async_trait;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use rust_decimal::Decimal;
use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DbErr, EntityTrait, Set};
use tracing::log::error;
use uuid::Uuid;

use crate::{
    entities::{self, billing, billing_item, team},
    DATABASE,
};

pub enum TeamError {
    DBError(DbErr),
    EmptyTeamError,
}

impl From<DbErr> for TeamError {
    fn from(dbErr: DbErr) -> Self {
        TeamError::DBError(dbErr)
    }
}

pub enum TeamBillingError {
    DBError(DbErr),
    EmptyBillingError,
}

impl From<DbErr> for TeamBillingError {
    fn from(dbErr: DbErr) -> Self {
        TeamBillingError::DBError(dbErr)
    }
}

pub struct Team {
    id: Uuid,
    team_name: String,
    user_id: String,
    billings: Option<Vec<Billing>>,
}

#[async_trait]
pub trait TeamBillingService {
    async fn create_billing(&self, name: String) -> Result<Billing, TeamError>;
}

#[async_trait]
pub trait BillingItemService {
    async fn end_billing(&self) -> Result<(), TeamBillingError>;
    async fn add_billing_item(&self, item: BillingItem) -> Result<(), TeamBillingError>;
    async fn delete_billing_item(&self, item_id: Uuid) -> Result<(), TeamBillingError>;
}

pub struct Billing {
    id: Uuid,
    name: String,
    start_time: Option<DateTime<Local>>,
    end_time: Option<DateTime<Local>>,
    billing_items: Option<Vec<BillingItem>>,
}

impl From<billing::Model> for Billing {
    fn from(billing_model: billing::Model) -> Self {
        Billing {
            id: billing_model.id,
            name: billing_model.name,
            start_time: parse_navie_time_to_data_time(billing_model.start_time),
            end_time: parse_navie_time_to_data_time(billing_model.end_time),
            billing_items: None,
        }
    }
}

#[async_trait]
impl BillingItemService for Billing {
    async fn end_billing(&self) -> Result<(), TeamBillingError> {
        let db = DATABASE.get().unwrap();
        let billing: Option<billing::Model> = entities::billing::Entity::find_by_id(self.id)
            .one(db)
            .await?;
        if let Some(bill) = billing {
            let mut bill_model: entities::billing::ActiveModel = bill.into();
            bill_model.end_time = Set(Some(Local::now().naive_local()));
            bill_model.update(db).await?;
            Ok(())
        } else {
            error!("can not find billing info");
            return Err(TeamBillingError::EmptyBillingError);
        }
    }

    async fn add_billing_item(&self, item: BillingItem) -> Result<(), TeamBillingError> {
        todo!()
    }
    async fn delete_billing_item(&self, item_id: Uuid) -> Result<(), TeamBillingError> {
        todo!()
    }
}

fn parse_navie_time_to_data_time(time: Option<NaiveDateTime>) -> Option<DateTime<Local>> {
    if let Some(naive_date_time) = time {
        match Local.from_local_datetime(&naive_date_time) {
            chrono::LocalResult::None => None,
            chrono::LocalResult::Single(data_time) => Some(data_time),
            chrono::LocalResult::Ambiguous(_, _) => None,
        }
    } else {
        None
    }
}

struct BillingItem {
    id: Uuid,
    name: String,
    item_type: String,
    cost: Decimal,
}

impl Team {
    pub async fn get_by_id(id: Uuid) -> Result<Self, TeamError> {
        let db = DATABASE.get().unwrap();
        let team_result = team::Entity::find_by_id(id).one(db).await?;
        if let Some(team_model) = team_result {
            let team = Team {
                id: team_model.id,
                team_name: team_model.team_name,
                user_id: team_model.user_id,
                billings: None,
            };
            return Ok(team);
        } else {
            return Err(TeamError::EmptyTeamError);
        }
    }
}

#[async_trait]
impl TeamBillingService for Team {
    async fn create_billing(&self, name: String) -> Result<Billing, TeamError> {
        let db = DATABASE.get().unwrap();
        let billing_model = billing::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(name),
            team_id: Set(Some(self.id.clone())),
            start_time: Set(Some(Local::now().naive_local())),
            end_time: NotSet,
        };
        let insert_result = billing_model.insert(db).await?;
        Ok(insert_result.into())
    }
}
