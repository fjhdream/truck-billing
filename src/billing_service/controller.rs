use chrono::Local;
use poem_openapi::{param::Path, payload::Json, ApiResponse, Object, OpenApi, Tags};
use tracing::log::error;
use uuid::{uuid, Uuid};

use crate::{billing_service::service::Team, team_service::service::TeamError};

use super::service::TeamService;

#[derive(Tags)]
enum ApiTags {
    Billing,
}

#[derive(Debug, Object)]
struct BillingCreateDTO {
    name: Option<String>,
}

#[derive(ApiResponse)]
enum BillingResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 201)]
    Created,

    #[oai(status = 500)]
    Error,
}

impl From<TeamError> for BillingResponse {
    fn from(_: TeamError) -> Self {
        BillingResponse::Error
    }
}

pub struct BillingRouter;

#[OpenApi]
impl BillingRouter {
    #[oai(
        path = "/team/:team_id/billing",
        method = "post",
        tag = "ApiTags::Billing"
    )]
    async fn create_billing(
        &self,
        team_id: Path<String>,
        team_billing: Json<BillingCreateDTO>,
    ) -> BillingResponse {
        let team_uuid_result = Uuid::parse_str(&team_id.0);
        if team_uuid_result.is_err() {
            error!("Error uuid string parse! id is {}", team_id.0);
            return BillingResponse::Error;
        }
        let team_uuid = team_uuid_result.unwrap();
        let billing_name = team_billing
            .0
            .name
            .unwrap_or(Local::now().format("%Y-%m-%d").to_string());
        if let Ok(team) = Team::get_by_id(team_uuid).await {
            if let Ok(_) = team.create_billing(billing_name).await {
                return BillingResponse::Created;
            } else {
                return BillingResponse::Error;
            }
        } else {
            return BillingResponse::Error;
        }
    }

    #[oai(
        path = "/team/:team_id/billing",
        method = "put",
        tag = "ApiTags::Billing"
    )]
    async fn end_billing(
        &self,
        team_id: Path<String>,
        team: Json<BillingCreateDTO>,
    ) -> BillingResponse {
        BillingResponse::Ok
    }

    #[oai(
        path = "/team/:team_id/billing",
        method = "patch",
        tag = "ApiTags::Billing"
    )]
    async fn update_billing(
        &self,
        team_id: Path<String>,
        team: Json<BillingCreateDTO>,
    ) -> BillingResponse {
        BillingResponse::Ok
    }
}
