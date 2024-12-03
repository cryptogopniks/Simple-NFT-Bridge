use cw_multi_test::Executor;
use speculoos::assert_that;

use cosmwasm_std::{Addr, StdResult};

use snb_base::{
    assets::{Currency, Token},
    error::ContractError,
    nft_minter::msg::MigrateMsg,
};

use crate::helpers::{
    nft_minter::NftMinterExtension,
    suite::{
        core::{assert_error, Project},
        types::ProjectAccount,
    },
};

#[test]
fn migrate_default() {
    let mut p = Project::new();

    p.app
        .migrate_contract(
            ProjectAccount::Admin.into(),
            p.get_nft_minter_address(),
            &MigrateMsg {
                version: "1.0.0".to_string(),
            },
            p.get_nft_minter_code_id(),
        )
        .unwrap();
}

#[test]
fn default() -> StdResult<()> {
    let mut p = Project::new();

    Ok(())
}
