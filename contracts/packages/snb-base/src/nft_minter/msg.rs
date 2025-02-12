use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct MigrateMsg {
    pub version: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub transceiver_hub: String,
    pub cw721_code_id: u64,
    pub wrapper: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    AcceptAdminRole {},

    UpdateConfig {
        admin: Option<String>,
        wrapper: Option<String>,
    },

    CreateCollection {
        name: String,
    },

    Mint {
        collection: String,
        token_list: Vec<String>,
        recipient: String,
    },

    Burn {
        collection: String,
        token_list: Vec<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(super::types::Config)]
    Config {},

    #[returns(String)]
    Collection { address: String },

    #[returns(Vec<(Addr, String)>)]
    CollectionList {
        amount: u32,
        start_after: Option<String>,
    },
}
