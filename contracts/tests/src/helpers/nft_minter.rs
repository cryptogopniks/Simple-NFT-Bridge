use cosmwasm_std::{Addr, StdResult};
use cw_multi_test::{AppResponse, Executor};

use snb_base::{
    error::parse_err,
    nft_minter::{
        msg::{ExecuteMsg, QueryMsg},
        types::Config,
    },
};

use crate::helpers::suite::{core::Project, types::ProjectAccount};

pub trait NftMinterExtension {
    fn nft_minter_try_accept_admin_role(
        &mut self,
        sender: ProjectAccount,
    ) -> StdResult<AppResponse>;

    fn nft_minter_try_update_config(
        &mut self,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        wrapper: Option<&Addr>,
    ) -> StdResult<AppResponse>;

    fn nft_minter_try_create_collection(
        &mut self,
        sender: ProjectAccount,
        name: &str,
    ) -> StdResult<AppResponse>;

    fn nft_minter_try_mint(
        &mut self,
        sender: ProjectAccount,
        collection: impl ToString,
        token_list: &[&str],
        recipient: impl ToString,
    ) -> StdResult<AppResponse>;

    fn nft_minter_try_burn(
        &mut self,
        sender: ProjectAccount,
        collection: impl ToString,
        token_list: &[&str],
    ) -> StdResult<AppResponse>;

    fn nft_minter_query_config(&self) -> StdResult<Config>;

    fn nft_minter_query_collection(&self, address: impl ToString) -> StdResult<String>;

    fn nft_minter_query_collection_list(
        &self,
        amount: u32,
        start_after: Option<&str>,
    ) -> StdResult<Vec<(Addr, String)>>;
}

impl NftMinterExtension for Project {
    #[track_caller]
    fn nft_minter_try_accept_admin_role(
        &mut self,
        sender: ProjectAccount,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_nft_minter_address(),
                &ExecuteMsg::AcceptAdminRole {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn nft_minter_try_update_config(
        &mut self,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        wrapper: Option<&Addr>,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_nft_minter_address(),
                &ExecuteMsg::UpdateConfig {
                    admin: admin.map(|x| x.to_string()),
                    wrapper: wrapper.map(|x| x.to_string()),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn nft_minter_try_create_collection(
        &mut self,
        sender: ProjectAccount,
        name: &str,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_nft_minter_address(),
                &ExecuteMsg::CreateCollection {
                    name: name.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn nft_minter_try_mint(
        &mut self,
        sender: ProjectAccount,
        collection: impl ToString,
        token_list: &[&str],
        recipient: impl ToString,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_nft_minter_address(),
                &ExecuteMsg::Mint {
                    collection: collection.to_string(),
                    token_list: token_list.iter().map(|x| x.to_string()).collect(),
                    recipient: recipient.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn nft_minter_try_burn(
        &mut self,
        sender: ProjectAccount,
        collection: impl ToString,
        token_list: &[&str],
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_nft_minter_address(),
                &ExecuteMsg::Burn {
                    collection: collection.to_string(),
                    token_list: token_list.iter().map(|x| x.to_string()).collect(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn nft_minter_query_config(&self) -> StdResult<Config> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_nft_minter_address(), &QueryMsg::Config {})
    }

    #[track_caller]
    fn nft_minter_query_collection(&self, address: impl ToString) -> StdResult<String> {
        self.app.wrap().query_wasm_smart(
            self.get_nft_minter_address(),
            &QueryMsg::Collection {
                address: address.to_string(),
            },
        )
    }

    #[track_caller]
    fn nft_minter_query_collection_list(
        &self,
        amount: u32,
        start_after: Option<&str>,
    ) -> StdResult<Vec<(Addr, String)>> {
        self.app.wrap().query_wasm_smart(
            self.get_nft_minter_address(),
            &QueryMsg::CollectionList {
                amount,
                start_after: start_after.as_ref().map(|x| x.to_string()),
            },
        )
    }
}
