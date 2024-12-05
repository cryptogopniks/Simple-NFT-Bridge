#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdResult,
};

use snb_base::{
    error::ContractError,
    nft_minter::{
        msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
        state::SAVE_CW721_ADDRESS_REPLY,
    },
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

        ExecuteMsg::UpdateConfig { admin } => e::try_update_config(deps, env, info, admin),

        ExecuteMsg::CreateCollection { name } => e::try_create_collection(deps, env, info, name),

        ExecuteMsg::Mint {
            collection,
            token_list,
            recipient,
        } => e::try_mint(deps, env, info, collection, token_list, recipient),

        ExecuteMsg::Burn {
            collection,
            token_list,
        } => e::try_burn(deps, env, info, collection, token_list),
    }
}

/// Exposes all the queries available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&q::query_config(deps, env)?),

        QueryMsg::Collection { address } => {
            to_json_binary(&q::query_collection(deps, env, address)?)
        }

        QueryMsg::CollectionList {
            amount,
            start_after,
        } => to_json_binary(&q::query_collection_list(deps, env, amount, start_after)?),
    }
}

/// Exposes all reply functions available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    let Reply { id, result } = reply;

    match id {
        SAVE_CW721_ADDRESS_REPLY => e::save_cw721_address(deps, env, &result),
        _ => Err(ContractError::UndefinedReplyId),
    }
}

/// Used for contract migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_contract(deps, env, msg)
}
