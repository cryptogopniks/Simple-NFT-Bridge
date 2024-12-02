use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub enum TransceiverType {
    Hub,
    Outpost,
}

#[cw_serde]
pub struct Collection {
    pub home_chain_address: String,
    pub hub_chain_address: String,
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub nft_minter: String,
    pub hub_address: String,
    pub transceiver_type: TransceiverType,
}

#[cw_serde]
pub struct TransferAdminState {
    pub new_admin: Addr,
    pub deadline: u64,
}

#[cw_serde]
pub struct Packet {
    pub hub_chain_collection: String,
    pub token_id_list: Vec<String>,
    pub recipient: String,
}
