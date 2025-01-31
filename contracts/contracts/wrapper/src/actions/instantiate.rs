use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use snb_base::{
    error::ContractError,
    wrapper::{
        msg::InstantiateMsg,
        state::{COLLECTIONS, CONFIG, CONTRACT_NAME, IS_PAUSED},
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
            worker: msg
                .worker
                .map(|x| deps.api.addr_validate(&x))
                .transpose()
                .unwrap_or(Some(sender.to_owned())),
            nft_minter: deps.api.addr_validate(&msg.nft_minter)?,
            lending_platform: deps.api.addr_validate(&msg.lending_platform)?,
        },
    )?;

    COLLECTIONS.save(deps.storage, &vec![])?;

    Ok(Response::new().add_attribute("action", "try_instantiate"))
}
