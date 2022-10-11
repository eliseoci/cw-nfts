use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<Addr>,
    pub cw721_instantiate_msg: Cw721InstantiateMsg,
}
