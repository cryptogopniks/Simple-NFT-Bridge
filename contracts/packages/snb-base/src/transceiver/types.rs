use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

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
    pub min_ntrn_ibc_fee: Uint128,
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

#[cw_serde]
pub enum IbcMemo<M> {
    Forward {
        channel: String,
        port: String,
        receiver: String,
        retries: u8,
        timeout: u64,
    },
    Wasm {
        contract: String,
        msg: M,
    },
}

#[cw_serde]
pub struct Channel {
    pub prefix: String,
    pub from_hub: String,
    pub to_hub: String,
}

impl Channel {
    pub fn new(prefix: &str, from_hub: &str, to_hub: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            from_hub: from_hub.to_string(),
            to_hub: to_hub.to_string(),
        }
    }
}
