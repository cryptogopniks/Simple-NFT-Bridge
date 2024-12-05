use cosmwasm_std::{
    to_json_binary, CosmosMsg, DepsMut, Empty, Env, MessageInfo, Order, Response, StdResult,
    SubMsg, SubMsgResult, WasmMsg,
};

use snb_base::{
    error::ContractError,
    nft_minter::{
        state::{
            COLLECTIONS, CONFIG, SAVE_CW721_ADDRESS_REPLY, TRANSFER_ADMIN_STATE,
            TRANSFER_ADMIN_TIMEOUT,
        },
        types::{Config, TransferAdminState},
    },
    utils::{check_funds, unwrap_field, FundsType},
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

pub fn try_update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    admin: Option<String>,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let config = CONFIG.load(deps.storage)?;
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

    // don't allow empty messages
    if !is_config_updated {
        Err(ContractError::NoParameters)?;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "try_update_config"))
}

pub fn try_create_collection(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let config = CONFIG.load(deps.storage)?;
    let nft_minter = &env.contract.address;

    if sender_address != config.admin {
        Err(ContractError::Unauthorized)?;
    }

    let collection_list = COLLECTIONS
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?;

    if collection_list
        .iter()
        .any(|(_, current_name)| current_name == &name)
    {
        Err(ContractError::CollectionDuplication)?;
    }

    // will be updated on reply
    COLLECTIONS.save(deps.storage, nft_minter, &name)?;

    let cw721_msg = cw721_base::msg::InstantiateMsg {
        name: name.clone(),
        symbol: String::default(),
        minter: nft_minter.to_string(),
    };

    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Instantiate {
        admin: Some(config.admin.to_string()),
        code_id: config.cw721_code_id,
        label: format!("Simple NFT Bridge collection: {}", name),
        msg: to_json_binary(&cw721_msg)?,
        funds: vec![],
    });

    let submsg = SubMsg::reply_on_success(msg, SAVE_CW721_ADDRESS_REPLY);

    Ok(Response::new()
        .add_submessage(submsg)
        .add_attribute("action", "try_create_collection"))
}

pub fn save_cw721_address(
    deps: DepsMut,
    env: Env,
    result: &SubMsgResult,
) -> Result<Response, ContractError> {
    let nft_minter = &env.contract.address;
    let res = result
        .to_owned()
        .into_result()
        .map_err(|e| ContractError::CustomError { val: e })?;

    let instantiate_event = unwrap_field(
        res.events.iter().find(|x| x.ty == "instantiate"),
        "instantiate_event",
    )?;

    let cw721_address = &unwrap_field(
        instantiate_event
            .attributes
            .iter()
            .find(|x| x.key == "_contract_address"),
        "cw721_address",
    )?
    .value;

    let name = COLLECTIONS.load(deps.storage, nft_minter)?;
    COLLECTIONS.remove(deps.storage, nft_minter);
    COLLECTIONS.save(deps.storage, &deps.api.addr_validate(cw721_address)?, &name)?;

    Ok(Response::new().add_attribute("cw721_address", cw721_address))
}

pub fn try_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collection: String,
    token_list: Vec<String>,
    recipient: String,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let config = CONFIG.load(deps.storage)?;

    if sender_address != config.transceiver_hub {
        Err(ContractError::Unauthorized)?;
    }

    // TODO: move in tr
    // let collection = deps.api.addr_validate(&collection)?;
    // deps.api.addr_validate(&recipient)?;

    // if !COLLECTIONS.has(deps.storage, &collection) {
    //     Err(ContractError::CollectionIsNotFound)?;
    // }

    let msg_list = token_list
        .into_iter()
        .map(|token_id| {
            Ok(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: collection.to_string(),
                msg: to_json_binary(
                    &cw721_base::ExecuteMsg::Mint::<Option<Empty>, Option<Empty>> {
                        token_id,
                        owner: recipient.clone(),
                        token_uri: None,
                        extension: None,
                    },
                )?,
                funds: vec![],
            }))
        })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(Response::new()
        .add_messages(msg_list)
        .add_attribute("action", "try_mint"))
}

pub fn try_burn(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collection: String,
    token_list: Vec<String>,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let config = CONFIG.load(deps.storage)?;

    if sender_address != config.transceiver_hub {
        Err(ContractError::Unauthorized)?;
    }

    let msg_list = token_list
        .into_iter()
        .map(|token_id| {
            Ok(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: collection.to_string(),
                msg: to_json_binary(
                    &cw721_base::ExecuteMsg::Burn::<Option<Empty>, Option<Empty>> { token_id },
                )?,
                funds: vec![],
            }))
        })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(Response::new()
        .add_messages(msg_list)
        .add_attribute("action", "try_burn"))
}
