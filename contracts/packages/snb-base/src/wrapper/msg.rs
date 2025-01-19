use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub worker: Option<String>,
    pub nft_minter: String,
    pub lending_platform: String,
}

#[cw_serde]
pub struct MigrateMsg {
    pub version: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // any
    AcceptAdminRole {},

    // admin
    UpdateConfig {
        admin: Option<String>,
        worker: Option<String>,
    },

    // admin, worker
    Pause {},

    Unpause {},

    // user
    Wrap {
        collection_in: String,
        token_list: Vec<String>,
    },

    Unwrap {
        collection_out: String,
        token_list: Vec<String>,
    },

    AddCollection {
        collection_in: String,
        collection_out: String,
    },

    RemoveCollection {
        collection_in: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(super::types::Config)]
    Config {},

    #[returns(Vec<super::types::Collection>)]
    CollectionList {},

    #[returns(super::types::Collection)]
    Collection { collection_in: String },
}
