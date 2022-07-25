use cosmwasm_std::{to_binary, BankMsg, Coin, CosmosMsg, QueryRequest, Uint128, WasmQuery};
use osmo_bindings::{OsmosisMsg, OsmosisQuery, Swap, SpotPriceResponse};
use primitives::{
    functions::{_is_valid_cdp, _cr}, vault::{functions::query_spot_price, self},
    vault_manager::msg::VaultConfigResponse,
};

use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<OsmosisQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Liquidate {} => try_liquidate(deps, env, info),
        ExecuteMsg::WithdrawCollateral { amount } => {
            try_withdraw_collateral(deps, env, info, amount)
        }
        ExecuteMsg::DepositCollateral { } => try_deposit_collateral(deps, env, info),
        ExecuteMsg::BorrowMore { amount } => todo!(),
        ExecuteMsg::Paydebt { amount } => try_pay_debt(deps, info, amount),
        ExecuteMsg::CloseVault { amount } => todo!(),
    }
}

pub fn try_liquidate(
    deps: DepsMut<OsmosisQuery>,
    env: Env,
    info: MessageInfo
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    
    let c = deps
        .querier
        .query_balance(&env.contract.address, state.collateral)?;
    let d = deps
        .querier
        .query_balance(&env.contract.address, state.debt.clone())?;

    let vault_config: VaultConfigResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager,
            msg: to_binary(&primitives::vault_manager::msg::QueryMsg::GetVaultConfig {
                clt: c.denom.clone(),
            })?,
        }))?;
        let spot_price = OsmosisQuery::spot_price(vault_config.pool_id, &c.denom, "g-usdc");
        let query = QueryRequest::from(spot_price);
        let c_price  = deps.querier.query(&query)?;
        let spot_priced = OsmosisQuery::spot_price(vault_config.pool_id, &d.denom, "g-usdc");
        let query = QueryRequest::from(spot_priced);
        let d_price  = deps.querier.query(&query)?;
    
    if _is_valid_cdp(
        c_price,
        d_price,
        c.amount,
        d.amount,
        vault_config.c_decimal,
        vault_config.mcr,
    ) {
       return Err(ContractError::ValidCDP {input: _cr(c_price, d_price, c.amount, d.amount, vault_config.c_decimal, vault_config.mcr), mcr: vault_config.mcr});
    }

    // add msg_join_pool
    //let msg_join_pool: CosmosMsg = MsgJoinPool {}.into();

    Ok(Response::new().add_attribute("method", "liquidate"))
}

pub fn try_deposit_collateral(
    deps: DepsMut<OsmosisQuery>,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let resp: primitives::nft::msg::OwnerOfResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager,
            msg: to_binary(&primitives::nft::msg::QueryMsg::OwnerOf {
                token_id: state.vault_id.to_string(),
                include_expired: Some(true),
            })?,
        }))?;

    if info.sender != resp.owner {
        return Err(ContractError::Unauthorized {});
    }
    let deposit = info.funds[0].clone();
    if state.collateral != deposit.denom {
        return Err(ContractError::NotRegisteredCollateral {
            registered: state.collateral,
            input: info.funds[0].clone().denom,
        });
    }

    Ok(Response::new()
        .add_attribute("method", "deposit_collateral")
        .add_attribute("denom", deposit.denom)
        .add_attribute("amount", deposit.amount.to_string()))
}

pub fn try_withdraw_collateral(
    deps: DepsMut<OsmosisQuery>,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let resp: primitives::nft::msg::OwnerOfResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager.clone(),
            msg: to_binary(&primitives::nft::msg::QueryMsg::OwnerOf {
                token_id: state.vault_id.to_string(),
                include_expired: Some(true),
            })?,
        }))?;

    if info.sender != resp.owner {
        return Err(ContractError::Unauthorized {});
    }

    let c = deps
        .querier
        .query_balance(&env.contract.address, state.collateral)?;
    let d = deps
        .querier
        .query_balance(&env.contract.address, state.debt.clone())?;

    let vault_config: VaultConfigResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager,
            msg: to_binary(&primitives::vault_manager::msg::QueryMsg::GetVaultConfig {
                clt: c.denom.clone(),
            })?,
        }))?;
        let spot_price = OsmosisQuery::spot_price(vault_config.pool_id, &c.denom, "g-usdc");
        let query = QueryRequest::from(spot_price);
        let c_price  = deps.querier.query(&query)?;
        let spot_priced = OsmosisQuery::spot_price(vault_config.pool_id, &d.denom, "g-usdc");
        let query = QueryRequest::from(spot_priced);
        let d_price  = deps.querier.query(&query)?;
    
    if !_is_valid_cdp(
        c_price,
        d_price,
        c.amount,
        state.borrow - amount,
        vault_config.c_decimal,
        vault_config.mcr,
    ) {
       return Err(ContractError::InvalidCDP {input: _cr(c_price, d_price, c.amount, state.borrow - amount, vault_config.c_decimal, vault_config.mcr), mcr: vault_config.mcr});
    }
    Ok(Response::new()
        .add_attribute("method", "withdraw_collateral")
        .add_messages(vec![CosmosMsg::Bank(BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: state.debt,
                amount: amount,
            }],
        })]))
}

pub fn try_pay_debt(
    deps: DepsMut<OsmosisQuery>,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let resp: primitives::nft::msg::OwnerOfResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager.clone(),
            msg: to_binary(&primitives::nft::msg::QueryMsg::OwnerOf {
                token_id: state.vault_id.to_string(),
                include_expired: Some(true),
            })?,
        }))?;

    if info.sender != resp.owner {
        return Err(ContractError::Unauthorized {});
    }
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.borrow = amount;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "pay_debt"))
}

pub fn try_borrow_more(
    deps: DepsMut<OsmosisQuery>,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let resp: primitives::nft::msg::OwnerOfResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: state.manager.clone(),
            msg: to_binary(&primitives::nft::msg::QueryMsg::OwnerOf {
                token_id: state.vault_id.to_string(),
                include_expired: Some(true),
            })?,
        }))?;

    if info.sender != resp.owner {
        return Err(ContractError::Unauthorized {});
    }
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.borrow = amount;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "borrow_more"))
}
