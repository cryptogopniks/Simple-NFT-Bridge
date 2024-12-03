use cw_multi_test::Executor;
use speculoos::assert_that;

use cosmwasm_std::StdResult;

use snb_base::transceiver::msg::MigrateMsg;

use crate::helpers::{
    suite::{
        core::Project,
        types::{ProjectAccount, ProjectNft},
    },
    transceiver::TransceiverExtension,
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
