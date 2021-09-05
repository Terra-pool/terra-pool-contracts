#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, coin, to_binary, Addr, BankMsg, Binary, Decimal, Deps, DepsMut, DistributionMsg, Env, CanonicalAddr,
    MessageInfo, QuerierWrapper, Response, StakingMsg, StdError, StdResult, Uint128, WasmMsg, Timestamp, WasmQuery
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, InvestmentResponse, QueryMsg};
use crate::state::{StratInfo, STRATEGY};

const FALLBACK_RATIO: Decimal = Decimal::one();

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:prize-strategy";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = StratInfo {
        start_time: msg.start_time,
        period: msg.period,
        terrand_contract_address: deps.api.addr_canonicalize(&msg.terrand_contract_address)?,
    };
    STRATEGY.save(deps.storage, &state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Award {} => award(deps, env, info),
        ExecuteMsg::BeforeTokenTransfer { from, to, amount } => before_token_transfer(deps, env, info, from, to, amount),
        ExecuteMsg::BeforeMint { from, to, amount } => before_mint(deps, env, info, from, to, amount),
        ExecuteMsg::BeforeBurn { from, to, amount } => before_burn(deps, env, info, from, to, amount)
    }
}

// assert if current time is in pool interval
fn assert_interval(supply: &Supply, bonded: Uint128) -> Result<(), ContractError> {
    if supply.bonded != bonded {
        Err(ContractError::BondedMismatch {
            stored: supply.bonded,
            queried: bonded,
        })
    } else {
        Ok(())
    }
}

pub fn award(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // assert last interval is over
    // if env.block.time > Timestamp::from_seconds(state.block_time_play) {

    //}
    // Load state
    let mut state = read_state(deps.storage)?;
    // draw call to random number generator
    let msg = terrand::msg::QueryMsg::GetRandomness { round: next_round };
    let terrand_human = deps.api.addr_humanize(&state.terrand_contract_address)?;
    let wasm = WasmQuery::Smart {
        contract_addr: terrand_human.to_string(),
        msg: to_binary(&msg)?,
    };
    let res: terrand::msg::GetRandomResponse = deps.querier.query(&wasm.into())?;
    let randomness_hash = hex::encode(res.randomness.as_slice());

    let n = randomness_hash
        .char_indices()
        .rev()
        .nth(state.combination_len as usize - 1)
        .map(|(i, _)| i)
        .unwrap();
    // load balance array

    // use random number to find winner from stored array of balances

    // bond them to the validator
    let res = Response::new()
        .add_message(StakingMsg::Delegate {
            validator: invest.validator,
            amount: payment.clone(),
        })
        .add_attribute("action", "bond")
        .add_attribute("from", info.sender)
        .add_attribute("bonded", payment.amount)
        .add_attribute("minted", to_mint);
    Ok(res)
}

pub fn before_token_transfer(deps: DepsMut, env: Env, info: MessageInfo, from: Addr, to: Addr, amount: Uint128) -> Result<Response, ContractError> {

}

pub fn before_mint(deps: DepsMut, env: Env, info: MessageInfo, from: Addr, to: Addr, amount: Uint128) -> Result<Response, ContractError> {

}

pub fn before_burn(deps: DepsMut, env: Env, info: MessageInfo, from: Addr, to: Addr, amount: Uint128) -> Result<Response, ContractError> {

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // custom queries
        QueryMsg::Claims { address } => {
            to_binary(&CLAIMS.query_claims(deps, &deps.api.addr_validate(&address)?)?)
        }
        QueryMsg::Investment {} => to_binary(&query_investment(deps)?),
        // inherited from cw20-base
        QueryMsg::TokenInfo {} => to_binary(&query_token_info(deps)?),
        QueryMsg::Balance { address } => to_binary(&query_balance(deps, address)?),
        QueryMsg::Allowance { owner, spender } => {
            to_binary(&query_allowance(deps, owner, spender)?)
        }
    }
}

pub fn query_investment(deps: Deps) -> StdResult<InvestmentResponse> {
    let invest = INVESTMENT.load(deps.storage)?;
    let supply = TOTAL_SUPPLY.load(deps.storage)?;

    let res = InvestmentResponse {
        owner: invest.owner.to_string(),
        exit_tax: invest.exit_tax,
        validator: invest.validator,
        min_withdrawal: invest.min_withdrawal,
        token_supply: supply.issued,
        staked_tokens: coin(supply.bonded.u128(), &invest.bond_denom),
        nominal_value: if supply.issued.is_zero() {
            FALLBACK_RATIO
        } else {
            Decimal::from_ratio(supply.bonded, supply.issued)
        },
    };
    Ok(res)
}
