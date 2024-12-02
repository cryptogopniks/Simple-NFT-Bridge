use cosmwasm_std::{
    to_json_binary, Addr, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Storage,
    WasmMsg,
};

use encryption_helper::serde::serialize_encrypt;
use snb_base::{
    converters::get_addr_by_prefix,
    error::ContractError,
    private_communication::types::Hash,
    transceiver::{
        msg::ExecuteMsg,
        state::{
            COLLECTIONS, CONFIG, ENC_KEY, HUB_PREFIX, IS_PAUSED, TRANSFER_ADMIN_STATE,
            TRANSFER_ADMIN_TIMEOUT,
        },
        types::{Collection, Config, Packet, TransceiverType, TransferAdminState},
    },
    utils::{check_funds, FundsType},
};

pub fn try_accept_admin_role(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let block_time = env.block.time.seconds();
    let TransferAdminState {
        new_admin,
        deadline,
    } = TRANSFER_ADMIN_STATE.load(deps.storage)?;

    if sender_address != new_admin {
        Err(ContractError::Unauthorized)?;
    }

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

pub fn try_pause(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let Config { admin, .. } = CONFIG.load(deps.storage)?;

    if sender_address != admin {
        Err(ContractError::Unauthorized)?;
    }

    IS_PAUSED.save(deps.storage, &true)?;

    Ok(Response::new().add_attribute("action", "try_pause"))
}

pub fn try_unpause(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let Config { admin, .. } = CONFIG.load(deps.storage)?;

    if sender_address != admin {
        Err(ContractError::Unauthorized)?;
    }

    IS_PAUSED.save(deps.storage, &false)?;

    Ok(Response::new().add_attribute("action", "try_unpause"))
}

pub fn try_update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    admin: Option<String>,
    nft_minter: Option<String>,
    hub_address: Option<String>,
    token_limit: Option<u8>,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let mut config = CONFIG.load(deps.storage)?;
    let mut is_config_updated = false;

    if sender_address != config.admin {
        Err(ContractError::Unauthorized)?;
    }

    if let Some(x) = admin {
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

    if let Some(x) = nft_minter {
        if config.transceiver_type == TransceiverType::Hub {
            deps.api.addr_validate(&x)?;
        }

        config.nft_minter = x;
        is_config_updated = true;
    }

    if let Some(x) = hub_address {
        if config.transceiver_type == TransceiverType::Hub {
            Err(ContractError::WrongActionType)?;
        }

        config.hub_address = x;
        is_config_updated = true;
    }

    if let Some(x) = token_limit {
        config.token_limit = x;
        is_config_updated = true;
    }

    // don't allow empty messages
    if !is_config_updated {
        Err(ContractError::NoParameters)?;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "try_update_config"))
}

pub fn try_add_collection(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    hub_collection: String,
    home_collection: String,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let config = CONFIG.load(deps.storage)?;

    if sender_address != config.admin {
        Err(ContractError::Unauthorized)?;
    }

    let collection_list = COLLECTIONS.load(deps.storage)?;

    if collection_list
        .iter()
        .any(|x| x.hub_collection == hub_collection || x.home_collection == home_collection)
    {
        Err(ContractError::CollectionDuplication)?;
    }

    if config.transceiver_type == TransceiverType::Hub {
        deps.api.addr_validate(&hub_collection)?;
    } else {
        deps.api.addr_validate(&home_collection)?;
    }

    COLLECTIONS.update(deps.storage, |mut collection_list| -> StdResult<Vec<_>> {
        collection_list.push(Collection {
            home_collection,
            hub_collection,
        });

        Ok(collection_list)
    })?;

    Ok(Response::new().add_attribute("action", "try_add_collection"))
}

pub fn try_remove_collection(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    hub_collection: String,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let config = CONFIG.load(deps.storage)?;

    if sender_address != config.admin {
        Err(ContractError::Unauthorized)?;
    }

    COLLECTIONS.update(deps.storage, |mut collection_list| -> StdResult<Vec<_>> {
        collection_list.retain(|x| x.hub_collection != hub_collection);

        Ok(collection_list)
    })?;

    Ok(Response::new().add_attribute("action", "try_remove_collection"))
}

pub fn try_send(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    hub_collection: String,
    token_list: Vec<String>,
    target: Option<String>,
) -> Result<Response, ContractError> {
    let mut response = Response::new().add_attribute("action", "try_send");
    check_pause_state(deps.storage)?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let timestamp = env.block.time;
    let config = CONFIG.load(deps.storage)?;
    let collection_list = COLLECTIONS.load(deps.storage)?;
    let Collection {
        home_collection,
        hub_collection,
    } = collection_list
        .iter()
        .find(|x| x.hub_collection == hub_collection)
        .ok_or(ContractError::CollectionIsNotFound)?;

    let mut tokens = token_list.clone();
    tokens.sort_unstable();
    tokens.dedup();

    if tokens.len() != token_list.len() {
        Err(ContractError::NftDuplication)?;
    }

    if token_list.is_empty() {
        Err(ContractError::EmptyTokenList)?;
    }

    if token_list.len() > config.token_limit as usize {
        Err(ContractError::ExceededTokenLimit)?;
    }

    // TODO: add storage for locked tokens
    match config.transceiver_type {
        TransceiverType::Outpost => {
            // check if nfts are on user balance
            check_tokens_holder(
                deps.as_ref(),
                &sender_address,
                &home_collection,
                &token_list,
            )?;

            // add transfer msgs
            for token_id in &token_list {
                let cw721_msg = cw721::Cw721ExecuteMsg::TransferNft {
                    recipient: env.contract.address.to_string(),
                    token_id: token_id.to_string(),
                };

                let msg = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: home_collection.clone(),
                    msg: to_json_binary(&cw721_msg)?,
                    funds: vec![],
                });

                response = response.add_message(msg);
            }

            // prepare and encrypt packet for accept msg
            let recipient = if target.is_none() {
                get_addr_by_prefix(&sender_address.to_string(), HUB_PREFIX)?
            } else {
                sender_address.to_string()
            };

            let packet = Packet {
                hub_collection: hub_collection.to_owned(),
                token_list,
                recipient,
            };

            let enc_key = Hash::parse(ENC_KEY)?;
            let encrypted_response = serialize_encrypt(&enc_key, &timestamp, &packet)?;

            match target {
                // same network
                Some(hub_contract) => {
                    response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: hub_contract,
                        msg: to_json_binary(&ExecuteMsg::Accept {
                            msg: encrypted_response.value,
                            timestamp: encrypted_response.timestamp,
                        })?,
                        funds: vec![],
                    }));
                }
                // ibc transfer
                None => {
                    unimplemented!();
                }
            }
        }
        TransceiverType::Hub => {
            unimplemented!();
        }
    }

    Ok(response)
}

/// user actions are disabled when the contract is paused
fn check_pause_state(storage: &dyn Storage) -> StdResult<()> {
    if IS_PAUSED.load(storage)? {
        Err(ContractError::ContractIsPaused)?;
    }

    Ok(())
}

pub fn check_tokens_holder(
    deps: Deps,
    holder: &Addr,
    collection_address: &str,
    token_id_list: &[String],
) -> StdResult<()> {
    const MAX_LIMIT: u32 = 100;
    const ITER_LIMIT: u32 = 50;

    let mut token_list: Vec<String> = vec![];
    let mut token_amount_sum: u32 = 0;
    let mut i: u32 = 0;
    let mut last_token: Option<String> = None;

    while (i == 0 || token_amount_sum == MAX_LIMIT) && i < ITER_LIMIT {
        i += 1;

        let query_tokens_msg = cw721::Cw721QueryMsg::Tokens {
            owner: holder.to_string(),
            start_after: last_token,
            limit: Some(MAX_LIMIT),
        };

        let cw721::TokensResponse { tokens } = deps
            .querier
            .query_wasm_smart(collection_address, &query_tokens_msg)?;

        for token in tokens.clone() {
            token_list.push(token);
        }

        token_amount_sum = tokens.len() as u32;
        last_token = tokens.last().cloned();
    }

    let are_tokens_owned = token_id_list.iter().all(|x| token_list.contains(x));

    if !are_tokens_owned {
        Err(ContractError::NftIsNotFound)?;
    }

    Ok(())
}
