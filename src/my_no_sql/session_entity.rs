use serde::{Deserialize, Serialize};

pub const PARTITION_KEY_VALUE: &str = "t";

#[my_no_sql_macros::my_no_sql_entity("crypto-deposit-settings")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionEntity {
    #[serde(rename = "traderId")]
    pub trader_id: String,
}
