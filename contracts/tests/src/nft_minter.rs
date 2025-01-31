use cw_multi_test::Executor;

use snb_base::nft_minter::msg::MigrateMsg;

use crate::helpers::suite::{core::Project, types::ProjectAccount};

#[test]
fn migrate_default() {
    let mut p = Project::new();

    p.app
        .migrate_contract(
            ProjectAccount::Admin.into(),
            p.get_nft_minter_address(),
            &MigrateMsg {
                version: "1.1.0".to_string(),
            },
            p.get_nft_minter_code_id(),
        )
        .unwrap();
}
