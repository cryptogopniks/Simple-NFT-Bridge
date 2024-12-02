use cosmwasm_std::{Deps, Env, StdResult};

use snb_base::{
    error::ContractError,
    transceiver::{
        state::{COLLECTIONS, CONFIG, IS_PAUSED, OUTPOSTS},
        types::{Collection, Config},
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
