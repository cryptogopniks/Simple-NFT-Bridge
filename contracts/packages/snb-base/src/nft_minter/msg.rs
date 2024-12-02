use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct MigrateMsg {
    pub version: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub cw721_code_id: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    AcceptAdminRole {},

    UpdateConfig {
        admin: Option<String>,
        whitelist: Option<Vec<String>>,
    },

    CreateCollection {
        name: String,
    },

    Mint {
        collection: String,
        token_id_list: Vec<String>,
    },

    Burn {
        collection: String,
        token_id_list: Vec<String>,
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
