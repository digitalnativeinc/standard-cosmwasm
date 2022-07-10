use cosmwasm_std::{BankMsg, Uint128, to_binary, WasmQuery, QueryRequest};
use primitives::{functions::_is_valid_cdp, vault_manager::msg::VaultConfigResponse, vault::functions::query_spot_price};
use osmo_bindings::{OsmosisQuery, Swap};

use super::*;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<OsmosisQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Liquidate {} => todo!(),
        ExecuteMsg::WithdrawCollateral { amount } => {
            try_withdraw_collateral(deps, env, info, amount)
        }
        ExecuteMsg::DepositCollateral { amount } => todo!(),
        ExecuteMsg::BorrowMore { amount } => todo!(),
        ExecuteMsg::Paydebt { amount } => todo!(),
        ExecuteMsg::CloseVault { amount } => todo!(),
    }
}

pub fn try_liquidate(
    deps: DepsMut<OsmosisQuery>,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.borrow = amount;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "liquidate"))
}

pub fn try_deposit_collateral(
    deps: DepsMut<OsmosisQuery>,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let deposit = info.funds[0];
    if state.collateral != deposit.denom {
        return Err(ContractError::NotRegisteredCollateral {
            registered: state.collateral,
            input: info.funds[0].denom,
        });
    }

    // Deposit collateral

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
    let c = deps.querier.query_balance(&env.contract.address, state.collateral)?;

    //end
    let vault_config: VaultConfigResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: state.manager,
        msg: to_binary(&primitives::vault_manager::msg::QueryMsg::GetVaultConfig {
            clt: c.denom,
        })?,
    }))?;

    
    // TODO: get price of an collateral and debt
    let c_price = query_spot_price(deps, Swap::new(vault_config.pool_id, c.denom, "g-usdc".to_string()), true)?.price;
    let d_price = query_spot_price(deps, Swap::new(vault_config.pool_id, c.denom, "g-usdc".to_string()), true)?.price;

    if _is_valid_cdp(c_price, d_price, c.amount, state.borrow - amount, vault_config.c_decimal, vault_config.mcr) {
        
    }
    Ok(Response::new().add_attribute("method", "withdraw_collateral"))
}

pub fn try_pay_debt(
    deps: DepsMut<OsmosisQuery>,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
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
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.borrow = amount;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "borrow_more"))
}
