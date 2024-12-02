use cosmwasm_schema::{cw_serde, QueryResponses};

use super::types::TransceiverType;

#[cw_serde]
pub struct MigrateMsg {
    pub version: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub transceiver_type: TransceiverType,
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
    },

    AddCollection {
        home_chain_address: String,
        hub_chain_address: String,
    },

    RemoveCollection {},

    Send {
        collection: String,
        token_id_list: Vec<String>,
        target: Option<String>,
    },

    Accept {
        msg: String,
        time: u64,
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

    #[returns(String)]
    Collection { hub_chain_address: String },

    #[returns(Vec<super::types::Collection>)]
    CollectionList {
        amount: u32,
        /// hub chain address
        start_after: Option<String>,
    },
}
