use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Uint128};
use cw0::Duration;
use cw_controllers::Claims;
use cw_storage_plus::Item;

pub const CLAIMS: Claims = Claims::new("claims");

/// strategy info
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StratInfo {
    pub start_time: Time,
    pub period: Duration
}

/// Supply is dynamic and tracks the current supply of staked and ERC20 tokens.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Default)]
pub struct Supply {
    /// issued is how many derivative tokens this contract has issued
    pub issued: Uint128,
    /// bonded is how many native tokens exist bonded to the validator
    pub bonded: Uint128,
    /// claims is how many tokens need to be reserved paying back those who unbonded
    pub claims: Uint128,
}

pub const STRATEGY: Item<StratInfo> = Item::new("strategy");
pub const TOTAL_SUPPLY: Item<Supply> = Item::new("total_supply");
