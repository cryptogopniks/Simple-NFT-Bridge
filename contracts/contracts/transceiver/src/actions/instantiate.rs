use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use snb_base::{
    error::ContractError,
    transceiver::{
        msg::InstantiateMsg,
        state::{
            CHANNELS, CHANNEL_NEUTRON_STARGAZE, CHANNEL_STARGAZE_NEUTRON, COLLECTIONS, CONFIG,
            CONTRACT_NAME, IS_PAUSED, OUTPOSTS, PREFIX_STARGAZE, TOKEN_LIMIT,
        },
        types::{Channel, Config, TransceiverType},
    },
};

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn try_instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let sender = &info.sender;
    let hub_address = if msg.transceiver_type == TransceiverType::Hub {
        env.contract.address.to_string()
    } else {
        msg.hub_address.unwrap_or_default()
    };

    IS_PAUSED.save(deps.storage, &false)?;
    CONFIG.save(
        deps.storage,
        &Config {
            admin: sender.to_owned(),
            nft_minter: msg.nft_minter.unwrap_or_default(),
            hub_address,
            transceiver_type: msg.transceiver_type,
            token_limit: msg.token_limit.unwrap_or(TOKEN_LIMIT),
        },
    )?;

    OUTPOSTS.save(deps.storage, &vec![])?;
    COLLECTIONS.save(deps.storage, &vec![])?;
    CHANNELS.save(
        deps.storage,
        &vec![Channel::new(
            PREFIX_STARGAZE,
            CHANNEL_NEUTRON_STARGAZE,
            CHANNEL_STARGAZE_NEUTRON,
        )],
    )?;

    Ok(Response::new().add_attribute("action", "try_instantiate"))
}
