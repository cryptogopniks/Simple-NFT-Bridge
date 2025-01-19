#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use snb_base::{
    error::ContractError,
    wrapper::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
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
        ExecuteMsg::AcceptAdminRole {} => e::try_accept_admin_role(deps, env, info),

        ExecuteMsg::UpdateConfig { admin, worker } => {
            e::try_update_config(deps, env, info, admin, worker)
        }

        ExecuteMsg::Pause {} => e::try_pause(deps, env, info),

        ExecuteMsg::Unpause {} => e::try_unpause(deps, env, info),

        ExecuteMsg::Wrap {
            collection_in,
            token_list,
        } => e::try_wrap(deps, env, info, collection_in, token_list),

        ExecuteMsg::Unwrap {
            collection_out,
            token_list,
        } => e::try_unwrap(deps, env, info, collection_out, token_list),

        ExecuteMsg::AddCollection {
            collection_in,
            collection_out,
        } => e::try_add_collection(deps, env, info, collection_in, collection_out),

        ExecuteMsg::RemoveCollection { collection_in } => {
            e::try_remove_collection(deps, env, info, collection_in)
        }
    }
}

/// Exposes all the queries available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfig {} => to_json_binary(&q::query_config(deps, env)?),

        QueryMsg::QueryCollectionList {} => to_json_binary(&q::query_collection_list(deps, env)?),

        QueryMsg::QueryCollection { collection_in } => {
            to_json_binary(&q::query_collection(deps, env, collection_in)?)
        }
    }
}

/// Used for contract migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_contract(deps, env, msg)
}
