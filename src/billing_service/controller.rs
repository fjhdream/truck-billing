use poem_openapi::{param::Path, payload::Json, ApiResponse, Object, OpenApi, Tags};

#[derive(Tags)]
enum ApiTags {
    Billing,
}

#[derive(Debug, Object)]
struct BillingCreateDTO {}

#[derive(ApiResponse)]
enum BillingResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 201)]
    Created,

    #[oai(status = 500)]
    Error,
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
        team: Json<BillingCreateDTO>,
    ) -> BillingResponse {
        BillingResponse::Created
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
