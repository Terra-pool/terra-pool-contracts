use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Uint128, Timestamp, CanonicalAddr};
use cw_storage_plus::Item;

/// strategy info
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StratInfo {
    pub start_time: Timestamp,
    pub period: Uint128,
    pub terrand_contract_address: CanonicalAddr
}

pub const STRATEGY: Item<StratInfo> = Item::new("strategy");
