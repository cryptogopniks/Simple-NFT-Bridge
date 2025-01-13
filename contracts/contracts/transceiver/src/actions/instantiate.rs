use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw2::set_contract_version;

use snb_base::{
    error::ContractError,
    transceiver::{
        msg::InstantiateMsg,
        state::{
            CHANNELS, CHANNEL_NEUTRON_STARGAZE, CHANNEL_STARGAZE_NEUTRON, COLLECTIONS, CONFIG,
            CONTRACT_NAME, IS_PAUSED, MIN_NTRN_IBC_FEE, OUTPOSTS, PREFIX_STARGAZE,
            RETRANSLATION_OUTPOST, TOKEN_LIMIT,
        },
        types::{Channel, Config},
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
    let transceiver = &env.contract.address;
    let hub_address = if msg.transceiver_type.is_hub() {
        transceiver.to_string()
    } else {
        msg.hub_address.unwrap_or_default()
    };

    if msg.transceiver_type.is_hub() && msg.is_retranslation_outpost {
        Err(ContractError::HubIsNotRetranslator)?;
    }

    let retranslation_outpost = if msg.is_retranslation_outpost {
        Some(transceiver.to_string())
    } else {
        None
    };
    RETRANSLATION_OUTPOST.save(deps.storage, &retranslation_outpost)?;

    IS_PAUSED.save(deps.storage, &false)?;
    CONFIG.save(
        deps.storage,
        &Config {
            admin: sender.to_owned(),
            nft_minter: msg.nft_minter.unwrap_or_default(),
            hub_address,
            transceiver_type: msg.transceiver_type,
            token_limit: msg.token_limit.unwrap_or(TOKEN_LIMIT),
            min_ntrn_ibc_fee: msg
                .min_ntrn_ibc_fee
                .unwrap_or(Uint128::new(MIN_NTRN_IBC_FEE)),
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
