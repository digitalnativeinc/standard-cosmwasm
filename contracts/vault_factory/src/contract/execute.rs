use cosmwasm_std::Uint128;

use super::*;
use crate::msg::{DepositCollateralResponse};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
        ExecuteMsg::Liquidate {} => todo!(),
        ExecuteMsg::WithdrawCollateral { amount } => try_withdraw_collateral(deps, info, amount),
        ExecuteMsg::DepositCollateral { amount } => todo!(),
        ExecuteMsg::BorrowMore { amount } => todo!(),
        ExecuteMsg::Paydebt { amount } => todo!(),
        ExecuteMsg::CloseVault { amount } => todo!(),
    }
}


pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

pub fn try_liquidate(deps: DepsMut, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.borrow = amount;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "liquidate"))
}

pub fn try_deposit_collateral(deps: DepsMut, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.borrow = amount;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

pub fn try_withdraw_collateral(deps: DepsMut, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.borrow = amount;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

pub fn try_pay_debt(deps: DepsMut, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.borrow = amount;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

pub fn try_borrow_more(deps: DepsMut, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.borrow = amount;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

pub fn try_create_vault(
    deps: DepsMut,
    info: MessageInfo,
    d_amount: Uint128,
) -> Result<Response, ContractError> {
    // dAmount in 9 decimal precision
    let config = CONFIG
        .may_load(deps.storage)?
        .ok_or(ContractError::Uninitialized {})?;
    // TODO: get asset value of submitting collateral and stablecoin with decimal, set asset price with 3 decimal and get token decimal from the submitted token
    // TODO: get asset price
    let c_price = Uint128::from(1u64);
    let d_price = Uint128::from(1u64);
    let c_amount: Uint128 = Uint128::from(2u64);
    // end
    let input = info.funds[0];
    let vault_config = VAULTCONFIG
        .may_load(deps.storage, input.denom)?
        .ok_or(ContractError::CollateralNotRegistered { denom: input.denom })?;
    // TODO: get asset
    // calculate cdp
    let messages: Vec<CosmosMsg> = match _is_valid_cdp(
        c_price,
        d_price,
        input.amount,
        d_amount,
        vault_config.c_decimal,
        vault_config.mcr,
    ) {
        true => {
            let config = CONFIG
                .may_load(deps.storage)?
                .ok_or(ContractError::Uninitialized {})?;
            Ok(vec![
                // Call factory to
                
                // Issue V1
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: config.v1.to_string(),
                    funds: vec![],
                    msg: to_binary(&primitives::nft::msg::ExecuteMsg::Mint::<Extension> {
                        token_id: token_id.clone(),
                        owner: info.sender.to_string(),
                        token_uri: Some(token_uri.clone()),
                        extension: None,
                    })?,
                }),
                // Mint Stablecoin
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: config.stablecoin.to_string(),
                    funds: vec![],
                    msg: to_binary(&primitives::token::msg::Mint {
                        recipient: info.sender.to_string(),
                        amount: d_amount,
                    })?,
                }),
            ]);
        }
        false => Err(ContractError::InvalidCDP {})
    }?;
    Ok(Response::new()
        .add_attribute("method", "try_create_vault")
        .add_submessage(SubMsg {
            id: 1,
            gas_limit: None,
            msg: CosmosMsg::Wasm(WasmMsg::Instantiate {
                code_id: config.vault_code_id,
                funds: vec![],
                admin: Some(env.contract.address.to_string()),
                label: "pair".to_string(),
                msg: to_binary(&PairInstantiateMsg {
                    asset_infos,
                    token_code_id: config.token_code_id,
                    asset_decimals,
                })?,
            }),
            reply_on: ReplyOn::Success,
        })
}
