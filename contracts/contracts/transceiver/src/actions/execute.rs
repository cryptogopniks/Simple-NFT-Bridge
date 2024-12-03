use cosmwasm_std::{
    to_json_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult, Timestamp, Uint128,
    WasmMsg,
};

use encryption_helper::serde::{decrypt_deserialize, serialize_encrypt};

use snb_base::{
    converters::get_addr_by_prefix,
    error::ContractError,
    private_communication::types::{EncryptedResponse, Hash},
    transceiver::{
        msg::ExecuteMsg,
        state::{
            CHANNELS, COLLECTIONS, CONFIG, DENOM_NTRN, ENC_KEY, IBC_TIMEOUT, IS_PAUSED, OUTPOSTS,
            PREFIX_NEUTRON, TRANSFER_ADMIN_STATE, TRANSFER_ADMIN_TIMEOUT,
        },
        types::{Channel, Collection, Config, Packet, TransceiverType, TransferAdminState},
    },
    utils::{check_funds, get_collection_operator_approvals, FundsType},
};

use crate::helpers::{
    check_pause_state, check_tokens_holder, get_channel_and_transceiver, get_ibc_transfer_memo,
    get_ibc_transfer_msg, get_neutron_ibc_transfer_msg,
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

    CONFIG.update(deps.storage, |mut x| -> StdResult<_> {
        x.admin = sender_address;
        Ok(x)
    })?;

    TRANSFER_ADMIN_STATE.update(deps.storage, |mut x| -> StdResult<_> {
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
    min_ntrn_ibc_fee: Option<Uint128>,
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

    if let Some(x) = min_ntrn_ibc_fee {
        config.min_ntrn_ibc_fee = x;
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

    COLLECTIONS.update(deps.storage, |mut collection_list| -> StdResult<_> {
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

    COLLECTIONS.update(deps.storage, |mut collection_list| -> StdResult<_> {
        collection_list.retain(|x| x.hub_collection != hub_collection);

        Ok(collection_list)
    })?;

    Ok(Response::new().add_attribute("action", "try_remove_collection"))
}

pub fn try_set_channel(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    prefix: String,
    from_hub: String,
    to_hub: String,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let config = CONFIG.load(deps.storage)?;

    if sender_address != config.admin {
        Err(ContractError::Unauthorized)?;
    }

    CHANNELS.update(deps.storage, |mut channel_list| -> StdResult<_> {
        channel_list.retain(|x| x.prefix == prefix);
        channel_list.push(Channel::new(&prefix, &from_hub, &to_hub));

        Ok(channel_list)
    })?;

    Ok(Response::new().add_attribute("action", "try_set_channel"))
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
    let (sender_address, asset_amount, asset_info) = check_funds(
        deps.as_ref(),
        &info,
        FundsType::Single {
            sender: None,
            amount: None,
        },
    )?;
    let contract_address = &env.contract.address;
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

    // we need 1 token for regular ibc transfer or 2 * fee + 1 for ibc transfer from hub
    let required_asset_amount =
        if target.is_none() && config.transceiver_type == TransceiverType::Hub {
            if asset_info.try_get_native()? != DENOM_NTRN {
                Err(ContractError::WrongAssetType)?;
            }

            Uint128::new(2 * config.min_ntrn_ibc_fee.u128() + 1)
        } else {
            Uint128::one()
        };

    if asset_amount != required_asset_amount {
        Err(ContractError::WrongFundsCombination)?;
    }

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

    // check if nfts are on user balance
    let collection_address = match config.transceiver_type {
        TransceiverType::Outpost => home_collection,
        TransceiverType::Hub => hub_collection,
    };

    check_tokens_holder(
        deps.as_ref(),
        &sender_address,
        collection_address,
        &token_list,
    )?;

    // add transfer msgs
    for token_id in &token_list {
        response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: collection_address.clone(),
            msg: to_json_binary(&cw721::Cw721ExecuteMsg::TransferNft {
                recipient: contract_address.to_string(),
                token_id: token_id.to_string(),
            })?,
            funds: vec![],
        }));
    }

    if let TransceiverType::Outpost = config.transceiver_type {
        // add approvals for burning and burn
        response = response.add_messages(get_collection_operator_approvals(
            deps.querier,
            &[collection_address],
            contract_address,
            config.nft_minter.clone(),
        )?);

        response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.nft_minter,
            msg: to_json_binary(&snb_base::nft_minter::msg::ExecuteMsg::Burn {
                collection: collection_address.to_owned(),
                token_list: token_list.clone(),
            })?,
            funds: vec![],
        }));
    }

    // prepare and encrypt packet for accept msg
    let recipient = if target.is_none() {
        get_addr_by_prefix(&sender_address, PREFIX_NEUTRON)?
    } else {
        sender_address.to_string()
    };

    let packet = Packet {
        sender: contract_address.to_string(),
        recipient,
        hub_collection: hub_collection.to_owned(),
        home_collection: home_collection.to_owned(),
        token_list,
    };

    let enc_key = Hash::parse(ENC_KEY)?;
    let EncryptedResponse { value, timestamp } = serialize_encrypt(&enc_key, &timestamp, &packet)?;

    match target {
        // same network
        Some(hub_contract) => {
            response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: hub_contract,
                msg: to_json_binary(&ExecuteMsg::Accept {
                    msg: value,
                    timestamp,
                })?,
                funds: vec![],
            }));
        }
        // ibc transfer
        None => {
            let outpost_list = OUTPOSTS.load(deps.storage)?;
            let channel_list = CHANNELS.load(deps.storage)?;
            let timeout_timestamp_ns = env.block.time.plus_seconds(IBC_TIMEOUT).nanos();
            let ibc_transfer_memo = get_ibc_transfer_memo(&config.hub_address, &value, timestamp)?;
            let (ibc_channel, target_transceiver) = get_channel_and_transceiver(
                contract_address,
                &config.hub_address,
                home_collection,
                &outpost_list,
                &channel_list,
            )?;

            let msg = if config.transceiver_type == TransceiverType::Hub {
                get_neutron_ibc_transfer_msg(
                    &ibc_channel,
                    &asset_info.try_get_native()?,
                    asset_amount,
                    contract_address,
                    &target_transceiver,
                    timeout_timestamp_ns,
                    &ibc_transfer_memo,
                    config.min_ntrn_ibc_fee,
                )
            } else {
                get_ibc_transfer_msg(
                    &ibc_channel,
                    &asset_info.try_get_native()?,
                    asset_amount,
                    contract_address,
                    &target_transceiver,
                    timeout_timestamp_ns,
                    &ibc_transfer_memo,
                )
            };

            response = response.add_message(msg);
        }
    }

    Ok(response)
}

pub fn try_accept(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: String,
    timestamp: Timestamp,
) -> Result<Response, ContractError> {
    let mut response = Response::new().add_attribute("action", "try_accept");
    let config = CONFIG.load(deps.storage)?;

    let enc_key = Hash::parse(ENC_KEY)?;
    let Packet {
        sender,
        recipient,
        hub_collection,
        home_collection,
        token_list,
    } = decrypt_deserialize(&enc_key, &timestamp, &msg)?;

    match config.transceiver_type {
        TransceiverType::Hub => {
            OUTPOSTS.update(deps.storage, |mut x| -> StdResult<_> {
                if !x.contains(&sender) {
                    x.push(sender);
                }

                Ok(x)
            })?;

            // mint nfts
            response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: config.nft_minter,
                msg: to_json_binary(&snb_base::nft_minter::msg::ExecuteMsg::Mint {
                    collection: hub_collection.to_owned(),
                    token_list,
                    recipient,
                })?,
                funds: vec![],
            }));
        }
        TransceiverType::Outpost => {
            // unlock nfts
            for token_id in token_list {
                response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: home_collection.clone(),
                    msg: to_json_binary(&cw721::Cw721ExecuteMsg::TransferNft {
                        recipient: recipient.clone(),
                        token_id: token_id.to_string(),
                    })?,
                    funds: vec![],
                }));
            }
        }
    };

    Ok(response)
}
