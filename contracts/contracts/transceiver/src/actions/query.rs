use cosmwasm_std::{Addr, Deps, Env, Order, StdResult};

use cw_storage_plus::Bound;
use snb_base::{
    error::ContractError,
    transceiver::{
        state::{CHANNELS, COLLECTIONS, CONFIG, IS_PAUSED, OUTPOSTS, USERS},
        types::{Channel, Collection, CollectionInfo, Config},
    },
};

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_pause_state(deps: Deps, _env: Env) -> StdResult<bool> {
    IS_PAUSED.load(deps.storage)
}

pub fn query_outposts(deps: Deps, _env: Env) -> StdResult<Vec<String>> {
    OUTPOSTS.load(deps.storage)
}

pub fn query_collection(
    deps: Deps,
    _env: Env,
    hub_collection: Option<String>,
    home_collection: Option<String>,
) -> StdResult<Collection> {
    let collections = COLLECTIONS.load(deps.storage)?;

    if let Some(x) = hub_collection {
        return collections
            .into_iter()
            .find(|y| y.hub_collection == x)
            .ok_or(ContractError::CollectionIsNotFound.into());
    }

    if let Some(x) = home_collection {
        return collections
            .into_iter()
            .find(|y| y.home_collection == x)
            .ok_or(ContractError::CollectionIsNotFound.into());
    }

    Err(ContractError::NoParameters)?
}

pub fn query_collection_list(deps: Deps, _env: Env) -> StdResult<Vec<Collection>> {
    COLLECTIONS.load(deps.storage)
}

pub fn query_channel_list(deps: Deps, _env: Env) -> StdResult<Vec<Channel>> {
    CHANNELS.load(deps.storage)
}

pub fn query_user(deps: Deps, _env: Env, address: String) -> StdResult<Vec<CollectionInfo>> {
    USERS.load(deps.storage, &deps.api.addr_validate(&address)?)
}

pub fn query_user_list(
    deps: Deps,
    _env: Env,
    amount: u32,
    start_after: Option<String>,
) -> StdResult<Vec<(Addr, Vec<CollectionInfo>)>> {
    let binding;
    let start_bound = match start_after {
        Some(addr) => {
            binding = deps.api.addr_validate(&addr)?;
            Some(Bound::exclusive(&binding))
        }
        None => None,
    };

    USERS
        .range(deps.storage, start_bound, None, Order::Ascending)
        .take(amount as usize)
        .collect::<StdResult<_>>()
}

// pub fn query_fee(deps: Deps, _env: Env) -> StdResult<Vec<Channel>> {
//     let request = QueryRequest::Stargate {
//         path: "/neutron.interchaintxs.v1.Query/Params".to_string(),
//         data: to_json_binary("")?,
//     };
//     let a: String = deps.querier.query(&request)?;

//     unimplemented!()
// }
