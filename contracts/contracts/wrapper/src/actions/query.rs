use cosmwasm_std::{Deps, Env, StdResult};

use snb_base::{
    error::ContractError,
    wrapper::{
        state::{COLLECTIONS, CONFIG},
        types::{Collection, Config},
    },
};

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_collection_list(deps: Deps, _env: Env) -> StdResult<Vec<Collection>> {
    COLLECTIONS.load(deps.storage)
}

pub fn query_collection(deps: Deps, _env: Env, collection_in: String) -> StdResult<Collection> {
    let collection_in = deps.api.addr_validate(&collection_in)?;

    Ok(COLLECTIONS
        .load(deps.storage)?
        .iter()
        .find(|x| x.collection_in == collection_in)
        .cloned()
        .ok_or(ContractError::CollectionIsNotFound)?)
}
