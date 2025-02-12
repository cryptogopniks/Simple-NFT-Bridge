use cw_multi_test::Executor;
use speculoos::assert_that;

use cosmwasm_std::StdResult;

use snb_base::transceiver::{msg::MigrateMsg, types::TransceiverType};

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
            p.get_transceiver_hub_address(),
            &MigrateMsg {
                version: "1.0.0".to_string(),
            },
            p.get_transceiver_code_id(),
        )
        .unwrap();
}

#[test]
fn local_transfer() -> StdResult<()> {
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

    Ok(())
}

#[test]
fn short_local_transfer_between_users() -> StdResult<()> {
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

    // send from alice to bob
    for token_id in ["1", "2"] {
        p.transfer_nft(
            ProjectAccount::Alice,
            ProjectAccount::Bob,
            collection_gopniks.to_owned(),
            token_id,
        );
    }

    // send hub -> outpost
    p.increase_allowances_nft(
        ProjectAccount::Bob,
        p.get_transceiver_hub_address(),
        collection_gopniks,
    );

    let bob_nft_home_before = p.query_nft(ProjectAccount::Bob, ProjectNft::Gopniks);
    let bob_nft_hub_before = p.query_nft(ProjectAccount::Bob, collection_gopniks);
    assert_that(&bob_nft_home_before).is_equal_to(to_string_vec(&["4", "5", "6"]));
    assert_that(&bob_nft_hub_before).is_equal_to(to_string_vec(&["1", "2"]));

    p.transceiver_try_send(
        ProjectAccount::Bob,
        TransceiverType::Hub,
        collection_gopniks,
        &["1", "2"],
        Some(p.get_transceiver_outpost_address()),
        1,
        ProjectCoin::Stars,
    )?;

    let bob_nft_home_after = p.query_nft(ProjectAccount::Bob, ProjectNft::Gopniks);
    let bob_nft_hub_after = p.query_nft(ProjectAccount::Bob, collection_gopniks);
    assert_that(&bob_nft_home_after).is_equal_to(to_string_vec(&["1", "2", "4", "5", "6"]));
    assert_that(&bob_nft_hub_after).is_equal_to(to_string_vec(&[]));

    Ok(())
}

// TODO: check wrong target
// TODO: check other guards
