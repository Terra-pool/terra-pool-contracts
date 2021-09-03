use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Uint128, Time};
use cw0::Duration;
use cw_storage_plus::Item;

/// strategy info
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StratInfo {
    pub start_time: Time,
    pub period: Duration
}

pub const STRATEGY: Item<StratInfo> = Item::new("strategy");
