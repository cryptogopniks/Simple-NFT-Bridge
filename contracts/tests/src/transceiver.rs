use cw_multi_test::Executor;
use speculoos::assert_that;

use cosmwasm_std::StdResult;

use snb_base::transceiver::{
    msg::MigrateMsg,
    types::{CollectionInfo, TransceiverType},
};

use crate::helpers::{
    nft_minter::NftMinterExtension,
    suite::{
        core::{to_string_vec, Project},
        types::{ProjectAccount, ProjectCoin, ProjectNft},
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

    p.nft_minter_try_create_collection(ProjectAccount::Admin, "gopniks")?;

    let collection_list = p.nft_minter_query_collection_list(9, None)?;
    let (collection_gopniks, _) = collection_list.first().unwrap();

    for transceiver in [TransceiverType::Hub, TransceiverType::Outpost] {
        p.transceiver_try_add_collection(
            ProjectAccount::Admin,
            transceiver,
            collection_gopniks,
            ProjectNft::Gopniks,
        )?;
    }

    // send outpost -> hub
    p.increase_allowances_nft(
        ProjectAccount::Alice,
        p.get_transceiver_outpost_address(),
        &ProjectNft::Gopniks.into(),
    );

    let alice_nft_home_before = p.query_nft(ProjectAccount::Alice, ProjectNft::Gopniks);
    let alice_nft_hub_before = p.query_nft(ProjectAccount::Alice, collection_gopniks);
    assert_that(&alice_nft_home_before).is_equal_to(to_string_vec(&["1", "2", "3"]));
    assert_that(&alice_nft_hub_before).is_equal_to(to_string_vec(&[]));

    p.transceiver_try_send(
        ProjectAccount::Alice,
        TransceiverType::Outpost,
        collection_gopniks,
        &["1", "2"],
        Some(p.get_transceiver_hub_address()),
        1,
        ProjectCoin::Stars,
    )?;

    let alice_nft_home_after = p.query_nft(ProjectAccount::Alice, ProjectNft::Gopniks);
    let alice_nft_hub_after = p.query_nft(ProjectAccount::Alice, collection_gopniks);
    assert_that(&alice_nft_home_after).is_equal_to(to_string_vec(&["3"]));
    assert_that(&alice_nft_hub_after).is_equal_to(to_string_vec(&["1", "2"]));

    let alice_locked_nft =
        p.transceiver_query_user(TransceiverType::Outpost, ProjectAccount::Alice)?;
    assert_that(&alice_locked_nft).is_equal_to(vec![CollectionInfo {
        home_collection: ProjectNft::Gopniks.to_string(),
        token_list: to_string_vec(&["1", "2"]),
    }]);

    // send hub -> outpost
    p.increase_allowances_nft(
        ProjectAccount::Alice,
        p.get_transceiver_hub_address(),
        collection_gopniks,
    );

    let alice_nft_home_before = p.query_nft(ProjectAccount::Alice, ProjectNft::Gopniks);
    let alice_nft_hub_before = p.query_nft(ProjectAccount::Alice, collection_gopniks);
    assert_that(&alice_nft_home_before).is_equal_to(to_string_vec(&["3"]));
    assert_that(&alice_nft_hub_before).is_equal_to(to_string_vec(&["1", "2"]));

    p.transceiver_try_send(
        ProjectAccount::Alice,
        TransceiverType::Hub,
        collection_gopniks,
        &["1", "2"],
        Some(p.get_transceiver_outpost_address()),
        1,
        ProjectCoin::Stars,
    )?;

    let alice_nft_home_after = p.query_nft(ProjectAccount::Alice, ProjectNft::Gopniks);
    let alice_nft_hub_after = p.query_nft(ProjectAccount::Alice, collection_gopniks);
    assert_that(&alice_nft_home_after).is_equal_to(to_string_vec(&["1", "2", "3"]));
    assert_that(&alice_nft_hub_after).is_equal_to(to_string_vec(&[]));

    let alice_locked_nft =
        p.transceiver_query_user(TransceiverType::Outpost, ProjectAccount::Alice)?;
    assert_that(&alice_locked_nft).is_equal_to(vec![]);

    Ok(())
}
