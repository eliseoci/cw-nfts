use cosmwasm_schema::cw_serde;

use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<Addr>,
    pub name: String,
    pub symbol: String,
    pub minter: String,
}
