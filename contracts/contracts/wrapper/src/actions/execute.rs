use cosmwasm_std::{
    to_json_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, Storage, WasmMsg,
};

use snb_base::{
    error::ContractError,
    transceiver::{state::TRANSFER_ADMIN_TIMEOUT, types::TransferAdminState},
    utils::{
        check_authorization, check_funds, check_tokens_holder, get_collection_operator_approvals,
        AuthType, FundsType,
    },
    wrapper::{
        state::{COLLECTIONS, CONFIG, IS_PAUSED, TRANSFER_ADMIN_STATE},
        types::{Collection, Config},
    },
};

pub fn try_accept_admin_role(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let Config { admin, worker, .. } = CONFIG.load(deps.storage)?;
    let block_time = env.block.time.seconds();
    let TransferAdminState {
        new_admin,
        deadline,
    } = TRANSFER_ADMIN_STATE.load(deps.storage)?;

    check_authorization(
        &sender_address,
        &admin,
        &worker,
        AuthType::Specified {
            allowlist: vec![Some(new_admin)],
        },
    )?;

    if block_time >= deadline {
        Err(ContractError::TransferAdminDeadline)?;
    }

    CONFIG.update(deps.storage, |mut x| -> StdResult<Config> {
        x.admin = sender_address;
        Ok(x)
    })?;

    TRANSFER_ADMIN_STATE.update(deps.storage, |mut x| -> StdResult<TransferAdminState> {
        x.deadline = block_time;
        Ok(x)
    })?;

    Ok(Response::new().add_attribute("action", "try_accept_admin_role"))
}

pub fn try_update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    admin: Option<String>,
    worker: Option<String>,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let mut config = CONFIG.load(deps.storage)?;
    let mut is_config_updated = false;
    let current_admin = &config.admin;
    let current_worker = &config.worker.clone();

    if let Some(x) = admin {
        check_authorization(
            &sender_address,
            current_admin,
            current_worker,
            AuthType::Admin,
        )?;
        let block_time = env.block.time.seconds();
        let new_admin = &deps.api.addr_validate(&x)?;

        TRANSFER_ADMIN_STATE.save(
            deps.storage,
            &TransferAdminState {
                new_admin: new_admin.to_owned(),
                deadline: block_time + TRANSFER_ADMIN_TIMEOUT,
            },
        )?;

        is_config_updated = true;
    }

    if let Some(x) = worker {
        check_authorization(
            &sender_address,
            current_admin,
            current_worker,
            AuthType::Admin,
        )?;
        config.worker = Some(deps.api.addr_validate(&x)?);
        is_config_updated = true;
    }

    // don't allow empty messages
    if !is_config_updated {
        Err(ContractError::NoParameters)?;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "try_update_config"))
}

pub fn try_pause(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let address_config = CONFIG.load(deps.storage)?;
    check_authorization(
        &sender_address,
        &address_config.admin,
        &address_config.worker,
        AuthType::AdminOrWorker,
    )?;

    IS_PAUSED.update(deps.storage, |_| -> StdResult<_> { Ok(true) })?;

    Ok(Response::new().add_attribute("action", "try_pause"))
}

pub fn try_unpause(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let address_config = CONFIG.load(deps.storage)?;
    check_authorization(
        &sender_address,
        &address_config.admin,
        &address_config.worker,
        AuthType::AdminOrWorker,
    )?;

    IS_PAUSED.update(deps.storage, |_| -> StdResult<_> { Ok(false) })?;

    Ok(Response::new().add_attribute("action", "try_unpause"))
}

pub fn try_wrap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection_in: String,
    token_list: Vec<String>,
) -> Result<Response, ContractError> {
    let mut response = Response::new().add_attribute("action", "try_wrap");
    check_pause_state(deps.storage)?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let contract_address = &env.contract.address;
    let config = CONFIG.load(deps.storage)?;
    let collection_list = COLLECTIONS.load(deps.storage)?;
    let Collection {
        collection_in,
        collection_out,
    } = collection_list
        .iter()
        .find(|x| x.collection_in == collection_in)
        .ok_or(ContractError::CollectionIsNotFound)?;

    check_token_list(&token_list)?;
    check_tokens_holder(deps.as_ref(), &sender_address, collection_in, &token_list)?;

    // move tokens to contract
    for token_id in &token_list {
        response = response.add_message(get_transfer_nft_msg(
            collection_in,
            contract_address,
            token_id,
        )?);
    }

    // mint tokens instead
    Ok(response.add_message(get_mint_nft_msg(
        &config.nft_minter,
        collection_out,
        &token_list,
        sender_address,
    )?))
}

pub fn try_unwrap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection_out: String,
    token_list: Vec<String>,
) -> Result<Response, ContractError> {
    let mut response = Response::new().add_attribute("action", "try_unwrap");
    check_pause_state(deps.storage)?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let contract_address = &env.contract.address;
    let config = CONFIG.load(deps.storage)?;
    let collection_list = COLLECTIONS.load(deps.storage)?;
    let Collection {
        collection_in,
        collection_out,
    } = collection_list
        .iter()
        .find(|x| x.collection_out == collection_out)
        .ok_or(ContractError::CollectionIsNotFound)?;

    check_token_list(&token_list)?;
    check_tokens_holder(deps.as_ref(), &sender_address, collection_out, &token_list)?;
    check_tokens_holder(deps.as_ref(), &contract_address, collection_in, &token_list)?;

    // move tokens to contract
    for token_id in &token_list {
        response = response.add_message(get_transfer_nft_msg(
            collection_out,
            contract_address,
            token_id,
        )?);
    }

    // add approvals for burning and burn
    response = response
        .add_messages(get_collection_operator_approvals(
            deps.querier,
            &[collection_out],
            contract_address,
            config.nft_minter.clone(),
        )?)
        .add_message(get_burn_nft_msg(
            &config.nft_minter,
            collection_out,
            &token_list,
        )?);

    // release original tokens
    for token_id in &token_list {
        response = response.add_message(get_transfer_nft_msg(
            collection_in,
            &sender_address,
            token_id,
        )?);
    }

    Ok(response)
}

pub fn try_add_collection(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collection_in: String,
    collection_out: String,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let config = CONFIG.load(deps.storage)?;
    let collection_in = deps.api.addr_validate(&collection_in)?;
    let collection_out = deps.api.addr_validate(&collection_out)?;

    if sender_address != config.admin {
        Err(ContractError::Unauthorized)?;
    }

    COLLECTIONS.update(deps.storage, |mut collection_list| -> StdResult<_> {
        if collection_list
            .iter()
            .any(|x| x.collection_in == collection_in || x.collection_out == collection_out)
        {
            Err(ContractError::CollectionDuplication)?;
        }

        collection_list.push(Collection {
            collection_in,
            collection_out,
        });

        Ok(collection_list)
    })?;

    Ok(Response::new().add_attribute("action", "try_add_collection"))
}

pub fn try_remove_collection(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collection_in: String,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let config = CONFIG.load(deps.storage)?;

    if sender_address != config.admin {
        Err(ContractError::Unauthorized)?;
    }

    COLLECTIONS.update(deps.storage, |mut collection_list| -> StdResult<_> {
        collection_list.retain(|x| x.collection_in != collection_in);

        Ok(collection_list)
    })?;

    Ok(Response::new().add_attribute("action", "try_remove_collection"))
}

/// user actions are disabled when the contract is paused
fn check_pause_state(storage: &dyn Storage) -> StdResult<()> {
    if IS_PAUSED.load(storage)? {
        Err(ContractError::ContractIsPaused)?;
    }

    Ok(())
}

fn get_transfer_nft_msg(
    collection_address: impl ToString,
    recipient: impl ToString,
    token_id: &str,
) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: collection_address.to_string(),
        msg: to_json_binary(&cw721::Cw721ExecuteMsg::TransferNft {
            recipient: recipient.to_string(),
            token_id: token_id.to_string(),
        })?,
        funds: vec![],
    }))
}

fn get_mint_nft_msg(
    nft_minter: impl ToString,
    collection: impl ToString,
    token_list: &[String],
    recipient: impl ToString,
) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: nft_minter.to_string(),
        msg: to_json_binary(&snb_base::nft_minter::msg::ExecuteMsg::Mint {
            collection: collection.to_string(),
            token_list: token_list.to_owned(),
            recipient: recipient.to_string(),
        })?,
        funds: vec![],
    }))
}

fn get_burn_nft_msg(
    nft_minter: impl ToString,
    collection: impl ToString,
    token_list: &[String],
) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: nft_minter.to_string(),
        msg: to_json_binary(&snb_base::nft_minter::msg::ExecuteMsg::Burn {
            collection: collection.to_string(),
            token_list: token_list.to_owned(),
        })?,
        funds: vec![],
    }))
}

fn check_token_list(token_list: &[String]) -> StdResult<()> {
    let mut tokens = token_list.to_vec();
    tokens.sort_unstable();
    tokens.dedup();

    if tokens.len() != token_list.len() {
        Err(ContractError::NftDuplication)?;
    }

    if token_list.is_empty() {
        Err(ContractError::EmptyTokenList)?;
    }

    Ok(())
}
