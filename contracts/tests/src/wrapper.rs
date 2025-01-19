use cosmwasm_std::StdResult;
use cw_multi_test::Executor;
use speculoos::assert_that;

use snb_base::wrapper::msg::MigrateMsg;

use crate::helpers::{
    nft_minter::NftMinterExtension,
    suite::{
        core::{to_string_vec, Project},
        types::{ProjectAccount, ProjectNft},
    },
    wrapper::WrapperExtension,
};

#[test]
fn migrate_default() {
    let mut p = Project::new();

    p.app
        .migrate_contract(
            ProjectAccount::Admin.into(),
            p.get_wrapper_address(),
            &MigrateMsg {
                version: "1.0.0".to_string(),
            },
            p.get_wrapper_code_id(),
        )
        .unwrap();
}

#[test]
fn default() -> StdResult<()> {
    let mut p = Project::new();

    p.nft_minter_try_create_collection(ProjectAccount::Admin, "gopniks")?;

    let collection_list = p.nft_minter_query_collection_list(9, None)?;
    let (collection_gopniks, _) = collection_list.first().unwrap();

    // register wrapper in nft-minter
    p.nft_minter_try_update_config(ProjectAccount::Admin, &None, Some(&p.get_wrapper_address()))?;

    // register collection in wrapper
    p.wrapper_try_add_collection(
        ProjectAccount::Admin,
        ProjectNft::Gopniks,
        collection_gopniks,
    )?;

    // wrap tokens
    p.increase_allowances_nft(
        ProjectAccount::Alice,
        p.get_wrapper_address(),
        &ProjectNft::Gopniks.into(),
    );

    let alice_nft_in = p.query_nft(ProjectAccount::Alice, ProjectNft::Gopniks);
    let alice_nft_out = p.query_nft(ProjectAccount::Alice, collection_gopniks);
    assert_that(&alice_nft_in).is_equal_to(to_string_vec(&["1", "2", "3"]));
    assert_that(&alice_nft_out).is_equal_to(to_string_vec(&[]));

    p.wrapper_try_wrap(ProjectAccount::Alice, ProjectNft::Gopniks, &["1", "2"])?;

    let alice_nft_in = p.query_nft(ProjectAccount::Alice, ProjectNft::Gopniks);
    let alice_nft_out = p.query_nft(ProjectAccount::Alice, collection_gopniks);
    assert_that(&alice_nft_in).is_equal_to(to_string_vec(&["3"]));
    assert_that(&alice_nft_out).is_equal_to(to_string_vec(&["1", "2"]));

    // unwrap tokens
    p.increase_allowances_nft(
        ProjectAccount::Alice,
        p.get_wrapper_address(),
        collection_gopniks,
    );

    p.wrapper_try_unwrap(ProjectAccount::Alice, collection_gopniks, &["1", "2"])?;

    let alice_nft_in = p.query_nft(ProjectAccount::Alice, ProjectNft::Gopniks);
    let alice_nft_out = p.query_nft(ProjectAccount::Alice, collection_gopniks);
    assert_that(&alice_nft_in).is_equal_to(to_string_vec(&["1", "2", "3"]));
    assert_that(&alice_nft_out).is_equal_to(to_string_vec(&[]));

    Ok(())
}

// TODO: check guards
