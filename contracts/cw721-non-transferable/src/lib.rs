pub use crate::msg::InstantiateMsg;
use crate::state::{Config, CONFIG};
use cosmwasm_std::Empty;
pub use cw721_base::{
    entry::execute as _execute, ContractError, Cw721Contract, ExecuteMsg, Extension,
    InstantiateMsg as Cw721BaseInstantiateMsg, MintMsg, MinterResponse,
};

pub mod msg;
pub mod state;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw721-non-transferable";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type QueryMsg = cw721_base::QueryMsg<Empty>;
pub type Cw721NonTransferableContract<'a> = Cw721Contract<'a, Extension, Empty, Empty, Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;
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

        let config = Config { admin: msg.admin };

        CONFIG.save(deps.storage, &config)?;

        let cw721_base_instantiate_msg = Cw721BaseInstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            minter: msg.minter,
        };

        Cw721NonTransferableContract::default().instantiate(
            deps,
            env,
            info,
            cw721_base_instantiate_msg,
        )?;

        Ok(Response::default()
            .add_attribute("contract_name", CONTRACT_NAME)
            .add_attribute("contract_version", CONTRACT_VERSION))
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension, Empty>,
    ) -> Result<Response, cw721_base::ContractError> {
        let config = CONFIG.load(deps.storage)?;
        match config.admin {
            Some(admin) => {
                if admin == info.sender {
                    _execute(deps, env, info, msg)
                } else {
                    Err(ContractError::Unauthorized {})
                }
            }
            None => match msg {
                ExecuteMsg::Mint(msg) => {
                    Cw721NonTransferableContract::default().mint(deps, env, info, msg)
                }
                _ => Err(ContractError::Unauthorized {}),
            },
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721NonTransferableContract::default().query(deps, env, msg)
    }
}
