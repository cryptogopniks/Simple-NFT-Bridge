use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct Collection {
    pub collection_in: Addr,
    pub collection_out: Addr,
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub worker: Option<Addr>,
    pub nft_minter: Addr,
    pub lending_platform: Addr,
}
