#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use snb_base::{
    error::ContractError,
    transceiver::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
};

use crate::actions::{
    execute as e, instantiate::try_instantiate, migrate::migrate_contract, query as q,
};

/// Creates a new contract with the specified parameters packed in the "msg" variable
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    try_instantiate(deps, env, info, msg)
}

/// Exposes all the execute functions available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Pause {} => e::try_pause(deps, env, info),

        ExecuteMsg::Unpause {} => e::try_unpause(deps, env, info),

        ExecuteMsg::AcceptAdminRole {} => e::try_accept_admin_role(deps, env, info),

        ExecuteMsg::UpdateConfig {
            admin,
            nft_minter,
            hub_address,
            token_limit,
        } => e::try_update_config(deps, env, info, admin, nft_minter, hub_address, token_limit),

        ExecuteMsg::AddCollection {
            hub_collection,
            home_collection,
        } => e::try_add_collection(deps, env, info, hub_collection, home_collection),

        ExecuteMsg::RemoveCollection { hub_collection } => {
            e::try_remove_collection(deps, env, info, hub_collection)
        }

        ExecuteMsg::Send {
            hub_collection,
            token_list,
            target,
        } => e::try_send(deps, env, info, hub_collection, token_list, target),

        ExecuteMsg::Accept { msg, timestamp } => e::try_accept(deps, env, info, msg, timestamp),
    }
}

/// Exposes all the queries available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&q::query_config(deps, env)?),

        QueryMsg::PauseState {} => to_json_binary(&q::query_pause_state(deps, env)?),

        QueryMsg::Outposts {} => to_json_binary(&q::query_outposts(deps, env)?),

        QueryMsg::Collection {
            hub_collection,
            home_collection,
        } => to_json_binary(&q::query_collection(
            deps,
            env,
            hub_collection,
            home_collection,
        )?),

        QueryMsg::CollectionList {} => to_json_binary(&q::query_collection_list(deps, env)?),
    }
}

/// Used for contract migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_contract(deps, env, msg)
}
