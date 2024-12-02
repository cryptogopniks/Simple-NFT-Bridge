use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use snb_base::{
    error::ContractError,
    nft_minter::{
        msg::InstantiateMsg,
        state::{CONFIG, CONTRACT_NAME, IS_PAUSED},
        types::Config,
    },
};

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn try_instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let sender = &info.sender;

    IS_PAUSED.save(deps.storage, &false)?;
    CONFIG.save(
        deps.storage,
        &Config {
            admin: sender.to_owned(),
            transceiver: deps.api.addr_validate(&msg.transceiver)?,
            cw721_code_id: msg.cw721_code_id,
        },
    )?;

    Ok(Response::new().add_attribute("action", "try_instantiate"))
}
