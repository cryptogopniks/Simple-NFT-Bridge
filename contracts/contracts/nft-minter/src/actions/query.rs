use cosmwasm_std::{Addr, Deps, Env, Order, StdResult};

use cw_storage_plus::Bound;
use snb_base::nft_minter::{
    state::{COLLECTIONS, CONFIG},
    types::Config,
};

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_collection(deps: Deps, _env: Env, address: String) -> StdResult<String> {
    COLLECTIONS.load(deps.storage, &deps.api.addr_validate(&address)?)
}

pub fn query_collection_list(
    deps: Deps,
    _env: Env,
    amount: u32,
    start_after: Option<String>,
) -> StdResult<Vec<(Addr, String)>> {
    let binding;
    let start_bound = match start_after {
        Some(addr) => {
            binding = deps.api.addr_validate(&addr)?;
            Some(Bound::exclusive(&binding))
        }
        None => None,
    };

    COLLECTIONS
        .range(deps.storage, start_bound, None, Order::Ascending)
        .take(amount as usize)
        .collect::<StdResult<_>>()
}
