#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};
pub mod executor;
use executor::{try_increment, try_reset};
pub mod querier;
use querier::query_count;

use self::executor::try_withdraw_collateral;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:osmo";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
        manager: todo!(),
        factory: todo!(),
        debt: todo!(),
        collateral: todo!(),
        vault_id: todo!(),
        borrow: todo!(),
        last_updated: todo!(),
        ex_sfr: todo!(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Set NFT lock
    STATE.save(deps.storage, &state)?;
    // initialize
    /*
      // called once by the factory at time of deployment
    function initialize(
      address manager_,
      uint256 vaultId_,
      address collateral_,
      address debt_,
      address v1_,
      uint256 amount_,
      address v2Factory_,
      address weth_
    ) external override initializer {
      vaultId = vaultId_;
      collateral = collateral_;
      debt = debt_;
      v1 = v1_;
      borrow = amount_;
      v2Factory = v2Factory_;
      WETH = weth_;
      manager = manager_;
      factory = msg.sender;
      lastUpdated = block.timestamp;
      createdAt = block.timestamp;
      ex_sfr = IVaultManager(manager).getSFR(collateral_);
    }

      */

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::Factory {} => todo!(),
        QueryMsg::Manager {} => todo!(),
        QueryMsg::Debt {} => todo!(),
        QueryMsg::V1 {} => todo!(),
        QueryMsg::Collateral {} => todo!(),
        QueryMsg::VaultId {} => todo!(),
        QueryMsg::Borrow {} => todo!(),
        QueryMsg::LastUpdated {} => todo!(),
        QueryMsg::CreatedAt {} => todo!(),
        QueryMsg::OutstandingPayment {} => todo!(),
    }
}

