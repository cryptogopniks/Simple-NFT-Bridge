use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub enum TransceiverType {
    Hub,
    Outpost,
}

#[cw_serde]
pub struct Collection {
    pub home_collection: String,
    pub hub_collection: String,
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub nft_minter: String,
    pub hub_address: String,
    pub transceiver_type: TransceiverType,
    pub token_limit: u8,
}

#[cw_serde]
pub struct TransferAdminState {
    pub new_admin: Addr,
    pub deadline: u64,
}

#[cw_serde]
pub struct Packet {
    pub sender: String,
    pub recipient: String,
    pub hub_collection: String,
    pub home_collection: String,
    pub token_list: Vec<String>,
}
