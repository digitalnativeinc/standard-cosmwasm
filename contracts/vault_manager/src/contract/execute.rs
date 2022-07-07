use cosmwasm_std::{Coin, CosmosMsg, Uint128, WasmMsg, SubMsg, Empty};
use primitives::functions::_is_valid_cdp;

use super::*;
use crate::state::{Config, VAULTCONFIG, CONFIG};

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
        ExecuteMsg::Initialize { vault_code_id } => todo!(),
        ExecuteMsg::CreateVault { dAmount } => todo!(),
        ExecuteMsg::InitializeConfig {
            clt,
            c_decimal_,
            pool_id_,
            mcr_,
            lfr_,
            sfr_,
        } => try_initialize_config(deps, info, clt, c_decimal_, pool_id_, mcr_, lfr_, sfr_),
    }
}

pub fn try_initialize_config(
    deps: DepsMut,
    info: MessageInfo,
    clt: String,
    c_decimal_: u64,
    pool_id_: u64,
    mcr_: u64,
    lfr_: u64,
    sfr_: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG
        .may_load(deps.storage)?
        .ok_or(ContractError::Uninitialized {})?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // TODO: check whether the pool includes stablecoin

    // Add config for the collateral
    VAULTCONFIG.update(
        deps.storage,
        clt.clone(),
        |config_opt| -> Result<_, ContractError> {
            let mut config = config_opt.unwrap_or_default();
            config.c_decimal = c_decimal_;
            config.mcr = mcr_;
            config.lfr = lfr_;
            config.sfr = sfr_;
            config.pool_id = pool_id_;
            Ok(config)
        },
    )?;
    // TODO: send event for initializing a vault config
    Ok(Response::new()
        .add_attribute("method", "try_initialize_config")
        .add_attribute("clt", clt)
        .add_attribute("mcr", mcr_.to_string())
        .add_attribute("lfr", lfr_.to_string())
        .add_attribute("sfr", sfr_.to_string())
        .add_attribute("pool_id", pool_id_.to_string()))
}

// This is a simple type to let us handle empty extensions
pub type Extension = Option<Empty>;

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
                // Instantiate Vault contract
                
                // Issue V1
                CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: config.v1.to_string(),
                    funds: vec![],
                    msg: to_binary(&primitives::nft::msg::Mint::<Extension> {
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
