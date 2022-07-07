use super::*;

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


pub fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}