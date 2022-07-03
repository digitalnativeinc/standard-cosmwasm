use cosmwasm_std::Uint128;

use super::*;
use crate::msg::{DepositCollateralResponse};


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

