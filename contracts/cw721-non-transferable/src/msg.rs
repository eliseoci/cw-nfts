use cosmwasm_schema::cw_serde;
use cw721_base::MintMsg;

#[cw_serde]
pub enum ExecuteMsg<T> {
    /// Mint a new NFT, can only be called by the contract minter
    Mint(MintMsg<T>),
}
