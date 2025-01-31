use cosmwasm_std::StdResult;
use cw_multi_test::{AppResponse, Executor};

use snb_base::{
    error::parse_err,
    wrapper::{
        msg::{ExecuteMsg, QueryMsg},
        types::{Collection, Config},
    },
};

use crate::helpers::suite::{core::Project, types::ProjectAccount};

use super::suite::types::ProjectNft;

pub trait WrapperExtension {
    fn wrapper_try_accept_admin_role(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn wrapper_try_update_config(
        &mut self,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        worker: &Option<ProjectAccount>,
    ) -> StdResult<AppResponse>;

    fn wrapper_try_pause(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn wrapper_try_unpause(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn wrapper_try_wrap(
        &mut self,
        sender: ProjectAccount,
        collection_in: ProjectNft,
        token_list: &[&str],
    ) -> StdResult<AppResponse>;

    fn wrapper_try_unwrap(
        &mut self,
        sender: ProjectAccount,
        collection_out: impl ToString,
        token_list: &[&str],
    ) -> StdResult<AppResponse>;

    fn wrapper_try_add_collection(
        &mut self,
        sender: ProjectAccount,
        collection_in: impl ToString,
        collection_out: impl ToString,
    ) -> StdResult<AppResponse>;

    fn wrapper_try_remove_collection(
        &mut self,
        sender: ProjectAccount,
        collection_in: impl ToString,
    ) -> StdResult<AppResponse>;

    fn wrapper_query_config(&self) -> StdResult<Config>;

    fn wrapper_query_collection_list(&self) -> StdResult<Vec<Collection>>;

    fn wrapper_query_collection(&self, collection_in: ProjectNft) -> StdResult<Collection>;
}

impl WrapperExtension for Project {
    #[track_caller]
    fn wrapper_try_accept_admin_role(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_wrapper_address(),
                &ExecuteMsg::AcceptAdminRole {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn wrapper_try_update_config(
        &mut self,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        worker: &Option<ProjectAccount>,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_wrapper_address(),
                &ExecuteMsg::UpdateConfig {
                    admin: admin.map(|x| x.to_string()),
                    worker: worker.map(|x| x.to_string()),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn wrapper_try_pause(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_wrapper_address(),
                &ExecuteMsg::Pause {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn wrapper_try_unpause(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_wrapper_address(),
                &ExecuteMsg::Unpause {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn wrapper_try_wrap(
        &mut self,
        sender: ProjectAccount,
        collection_in: ProjectNft,
        token_list: &[&str],
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_wrapper_address(),
                &ExecuteMsg::Wrap {
                    collection_in: collection_in.to_string(),
                    token_list: token_list.iter().map(|x| x.to_string()).collect(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn wrapper_try_unwrap(
        &mut self,
        sender: ProjectAccount,
        collection_out: impl ToString,
        token_list: &[&str],
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_wrapper_address(),
                &ExecuteMsg::Unwrap {
                    collection_out: collection_out.to_string(),
                    token_list: token_list.iter().map(|x| x.to_string()).collect(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn wrapper_try_add_collection(
        &mut self,
        sender: ProjectAccount,
        collection_in: impl ToString,
        collection_out: impl ToString,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_wrapper_address(),
                &ExecuteMsg::AddCollection {
                    collection_in: collection_in.to_string(),
                    collection_out: collection_out.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn wrapper_try_remove_collection(
        &mut self,
        sender: ProjectAccount,
        collection_in: impl ToString,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_wrapper_address(),
                &ExecuteMsg::RemoveCollection {
                    collection_in: collection_in.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn wrapper_query_config(&self) -> StdResult<Config> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_wrapper_address(), &QueryMsg::Config {})
    }

    #[track_caller]
    fn wrapper_query_collection_list(&self) -> StdResult<Vec<Collection>> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_wrapper_address(), &QueryMsg::CollectionList {})
    }

    #[track_caller]
    fn wrapper_query_collection(&self, collection_in: ProjectNft) -> StdResult<Collection> {
        self.app.wrap().query_wasm_smart(
            self.get_wrapper_address(),
            &QueryMsg::Collection {
                collection_in: collection_in.to_string(),
            },
        )
    }
}
