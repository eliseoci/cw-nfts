use crate::{msg::AdminResponse, state::CONFIG};
use cosmwasm_std::{Deps, StdResult};

pub fn admin(deps: Deps) -> StdResult<AdminResponse> {
    let config = CONFIG.load(deps.storage)?;
    let admin = match config.admin {
        Some(admin) => Some(admin.to_string()),
        None => None,
    };
    Ok(AdminResponse { admin })
}
