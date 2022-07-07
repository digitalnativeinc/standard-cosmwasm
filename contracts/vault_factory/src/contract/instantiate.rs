use super::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:stnd-vault-factory";
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