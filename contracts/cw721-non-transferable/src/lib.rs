use cosmwasm_std::Empty;
pub use cw721_base::{
    ContractError, Cw721Contract, Extension, InstantiateMsg, MintMsg, MinterResponse,
};
pub mod msg;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw721-non-transferable";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type QueryMsg = cw721_base::QueryMsg<Empty>;
pub type Cw721NonTransferableContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use crate::msg::ExecuteMsg;
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let res = Cw721NonTransferableContract::default().instantiate(deps, env, info, msg)?;

        Ok(res
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, cw721_base::ContractError> {
        match msg {
            ExecuteMsg::Mint(msg) => {
                Cw721NonTransferableContract::default().mint(deps, env, info, msg)
            }
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721NonTransferableContract::default().query(deps, env, msg)
    }
}
