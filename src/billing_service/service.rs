use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use uuid::Uuid;

struct Team {
    id: Uuid,
    team_name: String,
    billings: Option<Vec<Billing>>,
}

struct Billing {
    id: Uuid,
    name: String,
    start_time: DateTime<Local>,
    end_time: DateTime<Local>,
    billing_items: Vec<BillingItem>,
}

struct BillingItem {
    id: Uuid,
    name: String,
    item_type: String,
    cost: Decimal,
}

impl Team {
    async fn get_by_id(id: Uuid) -> Self {
        Team {
            id,
            team_name: todo!(),
            billings: todo!(),
        }
    }
}
