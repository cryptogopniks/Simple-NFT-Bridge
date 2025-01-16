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
            CHANNELS, COLLECTIONS, CONFIG, ENC_KEY, IBC_TIMEOUT, IS_PAUSED, OUTPOSTS,
            RETRANSLATION_OUTPOST, TRANSFER_ADMIN_STATE, TRANSFER_ADMIN_TIMEOUT,
        },
        types::{Channel, Collection, Config, Packet, TransceiverType, TransferAdminState},
    },
    utils::{check_funds, get_collection_operator_approvals, unwrap_field, FundsType},
};

use crate::helpers::{
    check_pause_state, check_token_list, check_tokens_holder, get_channel_and_transceiver,
    get_checked_amount_in, get_ibc_transfer_memo, get_ibc_transfer_msg,
    get_neutron_ibc_transfer_msg, get_transmission_info, split_address, validate_any_address,
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

#[allow(clippy::too_many_arguments)]
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
        validate_any_address(deps.as_ref(), &env, &x)?;

        config.nft_minter = x;
        is_config_updated = true;
    }

    if let Some(x) = hub_address {
        let outposts = OUTPOSTS.load(deps.storage)?;
        let retranslation_outpost = RETRANSLATION_OUTPOST.load(deps.storage)?;

        if config.transceiver_type.is_hub() {
            Err(ContractError::WrongActionType)?;
        }

        if outposts.contains(&x) {
            Err(ContractError::HubIsNotOutpost)?;
        }

        if let Some(retranslation_outpost) = retranslation_outpost {
            if x == retranslation_outpost {
                Err(ContractError::HubIsNotRetranslator)?;
            }
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
    env: Env,
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

    for address in [&hub_collection, &home_collection] {
        validate_any_address(deps.as_ref(), &env, &address)?;
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

pub fn try_set_retranslation_outpost(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    validate_any_address(deps.as_ref(), &env, &address)?;
    let config = CONFIG.load(deps.storage)?;
    let outposts = OUTPOSTS.load(deps.storage)?;

    if sender_address != config.admin {
        Err(ContractError::Unauthorized)?;
    }

    if address == config.hub_address {
        Err(ContractError::HubIsNotRetranslator)?;
    }

    if outposts.contains(&address) {
        Err(ContractError::HomeOutpostIsNotRetranslator)?;
    }

    RETRANSLATION_OUTPOST.save(deps.storage, &Some(address))?;

    Ok(Response::new().add_attribute("action", "try_set_retranslation_outpost"))
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
    let timestamp = env.block.time;
    let config = CONFIG.load(deps.storage)?;
    let retranslation_outpost = RETRANSLATION_OUTPOST.load(deps.storage)?;
    let outpost_list = OUTPOSTS.load(deps.storage)?;
    let collection_list = COLLECTIONS.load(deps.storage)?;
    let Collection {
        home_collection,
        hub_collection,
    } = collection_list
        .iter()
        .find(|x| x.hub_collection == hub_collection)
        .ok_or(ContractError::CollectionIsNotFound)?;
    let transceiver = env.contract.address.to_string();
    let transmission_info = get_transmission_info(
        &config,
        &retranslation_outpost,
        &target,
        &home_collection,
        &outpost_list,
        &transceiver,
        &transceiver,
    )?;

    // the msg is disabled for retranslation outpost
    if transmission_info.description.stage.is_second() {
        Err(ContractError::WrongMessageType)?;
    }

    let amount_in = get_checked_amount_in(
        &config,
        asset_amount,
        &asset_info,
        &transmission_info.description,
    )?;

    // check if nfts are on user balance
    let collection_address = match config.transceiver_type {
        TransceiverType::Outpost => home_collection,
        TransceiverType::Hub => hub_collection,
    };

    check_token_list(&config, &token_list)?;
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
                recipient: transmission_info.transceiver.clone(),
                token_id: token_id.to_string(),
            })?,
            funds: vec![],
        }));
    }

    if config.transceiver_type.is_hub() {
        // add approvals for burning and burn
        response = response.add_messages(get_collection_operator_approvals(
            deps.querier,
            &[collection_address],
            transmission_info.transceiver.clone(),
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
        let (hub_prefix, _) = split_address(hub_collection);
        let (home_prefix, _) = split_address(home_collection);
        let recipient_prefix = match config.transceiver_type {
            TransceiverType::Outpost => hub_prefix,
            TransceiverType::Hub => home_prefix,
        };

        get_addr_by_prefix(&sender_address, &recipient_prefix)?
    } else {
        sender_address.to_string()
    };

    let packet = Packet {
        sender: transmission_info.transceiver.clone(),
        recipient,
        hub_collection: hub_collection.to_owned(),
        home_collection: home_collection.to_owned(),
        token_list,
    };

    let enc_key = Hash::parse(ENC_KEY)?;
    let EncryptedResponse { value, timestamp } = serialize_encrypt(&enc_key, &timestamp, &packet)?;

    if transmission_info.description.mode.is_local() {
        // same network transfer
        let contract_address = if transmission_info.description.route.is_short() {
            if transmission_info.prefix.hub != transmission_info.prefix.home_outpost {
                Err(ContractError::TransceiversAreNotLocal)?;
            }

            transmission_info.target
        } else {
            let retranslation_outpost_prefix = unwrap_field(
                transmission_info.prefix.retranslation_outpost,
                "retranslation_outpost_prefix",
            )?;

            if transmission_info.prefix.hub != retranslation_outpost_prefix
                || retranslation_outpost_prefix != transmission_info.prefix.home_outpost
            {
                Err(ContractError::TransceiversAreNotLocal)?;
            }

            unwrap_field(retranslation_outpost, "retranslation_outpost")?
        };

        response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_address,
            msg: to_json_binary(&ExecuteMsg::Accept {
                msg: value.clone(),
                timestamp,
            })?,
            funds: vec![],
        }));
    } else {
        // ibc transfer
        let outpost_list = OUTPOSTS.load(deps.storage)?;
        let channel_list = CHANNELS.load(deps.storage)?;
        let timeout_timestamp_ns = env.block.time.plus_seconds(IBC_TIMEOUT).nanos();
        let denom_in = &asset_info.try_get_native()?;

        if transmission_info.description.route.is_short() {
            if transmission_info.prefix.hub == transmission_info.prefix.home_outpost {
                Err(ContractError::TransceiversAreNotInterchain)?;
            }

            let (ibc_channel, target_transceiver) = get_channel_and_transceiver(
                &transmission_info.transceiver,
                &config.hub_address,
                home_collection,
                &outpost_list,
                &channel_list,
            )?;
            let ibc_transfer_memo = get_ibc_transfer_memo(&target_transceiver, &value, timestamp)?;

            let msg = if config.transceiver_type.is_hub() {
                get_neutron_ibc_transfer_msg(
                    &ibc_channel,
                    denom_in,
                    amount_in,
                    &transmission_info.transceiver,
                    &target_transceiver,
                    timeout_timestamp_ns,
                    &ibc_transfer_memo,
                    config.min_ntrn_ibc_fee,
                )
            } else {
                get_ibc_transfer_msg(
                    &ibc_channel,
                    denom_in,
                    amount_in,
                    &transmission_info.transceiver,
                    &target_transceiver,
                    timeout_timestamp_ns,
                    &ibc_transfer_memo,
                )
            };

            response = response.add_message(msg);
        } else {
            let retranslation_outpost_prefix = unwrap_field(
                transmission_info.prefix.retranslation_outpost,
                "retranslation_outpost_prefix",
            )?;

            if transmission_info.prefix.hub == retranslation_outpost_prefix
                || retranslation_outpost_prefix == transmission_info.prefix.home_outpost
                || transmission_info.prefix.home_outpost == transmission_info.prefix.hub
            {
                Err(ContractError::TransceiversAreNotInterchain)?;
            }

            // TODO
        }
    }

    Ok(response)
}

pub fn try_accept(
    deps: DepsMut,
    env: Env,
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

    let retranslation_outpost = RETRANSLATION_OUTPOST.load(deps.storage)?;
    let outpost_list = OUTPOSTS.load(deps.storage)?;
    let transceiver = env.contract.address.to_string();
    let transmission_info = get_transmission_info(
        &config,
        &retranslation_outpost,
        &None,
        &home_collection,
        &outpost_list,
        &transceiver,
        &sender,
    )?;

    if transmission_info.description.stage.is_second() {
        if transmission_info.description.mode.is_local() {
            response = response.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: transmission_info.target,
                msg: to_json_binary(&ExecuteMsg::Accept { msg, timestamp })?,
                funds: vec![],
            }));
        } else {
            // TODO: IBC
            unimplemented!()
        }

        return Ok(response);
    }

    if config.transceiver_type.is_hub() {
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
    } else {
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

    Ok(response)
}
