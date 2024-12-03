use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Timestamp, Uint128};

use super::types::TransceiverType;

#[cw_serde]
pub struct MigrateMsg {
    pub version: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub nft_minter: Option<String>,
    pub hub_address: Option<String>,
    pub transceiver_type: TransceiverType,
    pub token_limit: Option<u8>,
    pub min_ntrn_ibc_fee: Option<Uint128>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Pause {},

    Unpause {},

    AcceptAdminRole {},

    UpdateConfig {
        admin: Option<String>,
        nft_minter: Option<String>,
        hub_address: Option<String>,
        token_limit: Option<u8>,
        min_ntrn_ibc_fee: Option<Uint128>,
    },

    AddCollection {
        hub_collection: String,
        home_collection: String,
    },

    RemoveCollection {
        hub_collection: String,
    },

    SetChannel {
        prefix: String,
        from_hub: String,
        to_hub: String,
    },

    Send {
        hub_collection: String,
        token_list: Vec<String>,
        /// if specified will send to the contract on the same chain
        target: Option<String>,
    },

    Accept {
        msg: String,
        timestamp: Timestamp,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(super::types::Config)]
    Config {},

    #[returns(bool)]
    PauseState {},

    /// works well only for TransceiverType::Hub
    #[returns(Vec<String>)]
    Outposts {},

    #[returns(super::types::Collection)]
    Collection {
        hub_collection: Option<String>,
        home_collection: Option<String>,
    },

    #[returns(Vec<super::types::Collection>)]
    CollectionList {},

    #[returns(Vec<super::types::Channel>)]
    ChannelList {},

    #[returns(Vec<super::types::CollectionInfo>)]
    User { address: String },

    #[returns(Vec<(cosmwasm_std::Addr, Vec<super::types::CollectionInfo>)>)]
    UserList {
        amount: u32,
        start_after: Option<String>,
    },
}
