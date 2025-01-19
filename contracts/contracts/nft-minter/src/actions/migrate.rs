use cosmwasm_std::{DepsMut, Env, Response, Storage};
use cw2::{get_contract_version, set_contract_version};

use cw_storage_plus::Item;
use semver::Version;

use snb_base::{
    error::ContractError,
    nft_minter::{
        msg::MigrateMsg,
        state::{CONFIG, CONTRACT_NAME},
        types::{Config, ConfigPre},
    },
};

pub fn migrate_contract(
    deps: DepsMut,
    _env: Env,
    msg: MigrateMsg,
) -> Result<Response, ContractError> {
    let (version_previous, version_new) = get_versions(deps.storage, msg)?;

    if version_new >= version_previous {
        set_contract_version(deps.storage, CONTRACT_NAME, version_new.to_string())?;

        let ConfigPre {
            admin,
            transceiver_hub,
            cw721_code_id,
        } = Item::new("config").load(deps.storage)?;
        CONFIG.save(
            deps.storage,
            &Config {
                admin,
                transceiver_hub,
                cw721_code_id,
                wrapper: None,
            },
        )?;
    }

    Ok(Response::new())
}

fn get_versions(
    storage: &dyn Storage,
    msg: MigrateMsg,
) -> Result<(Version, Version), ContractError> {
    let version_previous: Version = get_contract_version(storage)?
        .version
        .parse()
        .map_err(|_| ContractError::ParsingPrevVersion)?;

    let version_new: Version = env!("CARGO_PKG_VERSION")
        .parse()
        .map_err(|_| ContractError::ParsingNewVersion)?;

    if version_new.to_string() != msg.version {
        Err(ContractError::ImproperMsgVersion)?;
    }

    Ok((version_previous, version_new))
}
