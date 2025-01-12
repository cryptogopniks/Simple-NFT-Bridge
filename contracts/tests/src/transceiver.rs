use cw_multi_test::Executor;
use speculoos::assert_that;

use cosmwasm_std::{Addr, StdResult};

use snb_base::transceiver::{msg::MigrateMsg, types::TransceiverType};

use crate::helpers::{
    nft_minter::NftMinterExtension,
    suite::{
        codes::WithCodes,
        core::{to_string_vec, Project},
        types::{ProjectAccount, ProjectCoin, ProjectNft},
    },
    transceiver::TransceiverExtension,
};

fn add_retranslation_outpost(p: &mut Project) -> StdResult<Addr> {
    // add retranslation outpost, update configs
    let retranslation_outpost_address = p.instantiate_transceiver(
        p.get_transceiver_code_id(),
        None,
        Some(&p.get_transceiver_hub_address()),
        None,
        TransceiverType::Outpost,
        None,
        None,
    );

    p.transceiver_try_update_config(
        ProjectAccount::Admin,
        &retranslation_outpost_address,
        None,
        None,
        Some(&p.get_transceiver_hub_address()),
        None,
        None,
    )?;

    for transceiver in [
        &p.get_transceiver_hub_address(),
        &p.get_transceiver_outpost_address(),
        &retranslation_outpost_address,
    ] {
        p.transceiver_try_set_retranslation_outpost(
            ProjectAccount::Admin,
            transceiver,
            &retranslation_outpost_address,
        )?;
    }

    Ok(retranslation_outpost_address)
}

#[test]
fn migrate_default() {
    let mut p = Project::new();

    p.app
        .migrate_contract(
            ProjectAccount::Admin.into(),
            p.get_transceiver_hub_address(),
            &MigrateMsg {
                version: "1.1.0".to_string(),
            },
            p.get_transceiver_code_id(),
        )
        .unwrap();
}

#[test]
fn short_local_transfer() -> StdResult<()> {
    let mut p = Project::new();

    p.nft_minter_try_create_collection(ProjectAccount::Admin, "gopniks")?;

    let collection_list = p.nft_minter_query_collection_list(9, None)?;
    let (collection_gopniks, _) = collection_list.first().unwrap();

    for transceiver in &[
        p.get_transceiver_hub_address(),
        p.get_transceiver_outpost_address(),
    ] {
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
        &p.get_transceiver_outpost_address(),
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
        &p.get_transceiver_hub_address(),
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

    for transceiver in &[
        p.get_transceiver_hub_address(),
        p.get_transceiver_outpost_address(),
    ] {
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
        &p.get_transceiver_outpost_address(),
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
        &p.get_transceiver_hub_address(),
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

#[test]
fn long_local_transfer() -> StdResult<()> {
    let mut p = Project::new();
    let retranslation_outpost_address = add_retranslation_outpost(&mut p)?;

    p.nft_minter_try_create_collection(ProjectAccount::Admin, "gopniks")?;

    let collection_list = p.nft_minter_query_collection_list(9, None)?;
    let (collection_gopniks, _) = collection_list.first().unwrap();

    for transceiver in [
        &p.get_transceiver_hub_address(),
        &p.get_transceiver_outpost_address(),
        &retranslation_outpost_address,
    ] {
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
        &p.get_transceiver_outpost_address(),
        collection_gopniks,
        &["1", "2"],
        Some(retranslation_outpost_address),
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
        &p.get_transceiver_hub_address(),
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

// TODO: check if RetranslationOutpost can't be used as HomeOutpost, there is no way to send:
// RetranslationOutpost (chain A) -> Hub (chain B)
// RetranslationOutpost (chain A) -> HomeOutpost (chain B)
// RetranslationOutpost (chain A) -> Hub (chain A)
// RetranslationOutpost (chain A) -> HomeOutpost (chain A)
//

// TODO: check wrong target
// TODO: check there is no intersection between Hub, HomeOutpost, RetranslationOutpost
// TODO: check other guards
