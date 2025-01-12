use cosmwasm_std::{Addr, StdResult, Timestamp, Uint128};
use cw_multi_test::{AppResponse, Executor};

use snb_base::{
    error::parse_err,
    transceiver::types::{Channel, Collection},
    transceiver::{
        msg::{ExecuteMsg, QueryMsg},
        types::Config,
    },
};

use crate::helpers::suite::{
    core::{add_funds_to_exec_msg, Project},
    types::{ProjectAccount, ProjectAsset},
};

use super::suite::core::to_string_vec;

pub trait TransceiverExtension {
    fn transceiver_try_pause(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_unpause(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_accept_admin_role(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
    ) -> StdResult<AppResponse>;

    #[allow(clippy::too_many_arguments)]
    fn transceiver_try_update_config(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        admin: Option<ProjectAccount>,
        nft_minter: Option<&Addr>,
        hub_address: Option<&Addr>,
        token_limit: Option<u8>,
        min_ntrn_ibc_fee: Option<u128>,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_add_collection(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        hub_collection: impl ToString,
        home_collection: impl ToString,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_remove_collection(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        hub_collection: impl ToString,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_set_retranslation_outpost(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        retranslation_outpost: impl ToString,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_set_channel(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        prefix: &str,
        from_hub: &str,
        to_hub: &str,
    ) -> StdResult<AppResponse>;

    #[allow(clippy::too_many_arguments)]
    fn transceiver_try_send(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        hub_collection: impl ToString,
        token_list: &[&str],
        target: Option<Addr>,
        amount: u128,
        asset: impl Into<ProjectAsset>,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_accept(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        msg: &str,
        timestamp: Timestamp,
    ) -> StdResult<AppResponse>;

    fn transceiver_query_config(&self, transceiver_address: &Addr) -> StdResult<Config>;

    fn transceiver_query_pause_state(&self, transceiver_address: &Addr) -> StdResult<bool>;

    fn transceiver_query_outposts(&self, transceiver_address: &Addr) -> StdResult<Vec<String>>;

    fn transceiver_query_collection(
        &self,
        transceiver_address: &Addr,
        hub_collection: Option<&Addr>,
        home_collection: Option<&Addr>,
    ) -> StdResult<Collection>;

    fn transceiver_query_collection_list(
        &self,
        transceiver_address: &Addr,
    ) -> StdResult<Vec<Collection>>;

    fn transceiver_query_channel_list(&self, transceiver_address: &Addr)
        -> StdResult<Vec<Channel>>;
}

impl TransceiverExtension for Project {
    #[track_caller]
    fn transceiver_try_pause(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                transceiver_address.to_owned(),
                &ExecuteMsg::Pause {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_try_unpause(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                transceiver_address.to_owned(),
                &ExecuteMsg::Unpause {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_try_accept_admin_role(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                transceiver_address.to_owned(),
                &ExecuteMsg::AcceptAdminRole {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_try_update_config(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        admin: Option<ProjectAccount>,
        nft_minter: Option<&Addr>,
        hub_address: Option<&Addr>,
        token_limit: Option<u8>,
        min_ntrn_ibc_fee: Option<u128>,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                transceiver_address.to_owned(),
                &ExecuteMsg::UpdateConfig {
                    admin: admin.map(|x| x.to_string()),
                    nft_minter: nft_minter.map(|x| x.to_string()),
                    hub_address: hub_address.map(|x| x.to_string()),
                    token_limit,
                    min_ntrn_ibc_fee: min_ntrn_ibc_fee.map(Uint128::new),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_try_add_collection(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        hub_collection: impl ToString,
        home_collection: impl ToString,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                transceiver_address.to_owned(),
                &ExecuteMsg::AddCollection {
                    hub_collection: hub_collection.to_string(),
                    home_collection: home_collection.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_try_remove_collection(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        hub_collection: impl ToString,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                transceiver_address.to_owned(),
                &ExecuteMsg::RemoveCollection {
                    hub_collection: hub_collection.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_try_set_retranslation_outpost(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        retranslation_outpost: impl ToString,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                transceiver_address.to_owned(),
                &ExecuteMsg::SetRetranslationOutpost(retranslation_outpost.to_string()),
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_try_set_channel(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        prefix: &str,
        from_hub: &str,
        to_hub: &str,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                transceiver_address.to_owned(),
                &ExecuteMsg::SetChannel {
                    prefix: prefix.to_string(),
                    from_hub: from_hub.to_string(),
                    to_hub: to_hub.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_try_send(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        hub_collection: impl ToString,
        token_list: &[&str],
        target: Option<Addr>,
        amount: u128,
        asset: impl Into<ProjectAsset>,
    ) -> StdResult<AppResponse> {
        add_funds_to_exec_msg(
            self,
            sender,
            transceiver_address,
            &ExecuteMsg::Send {
                hub_collection: hub_collection.to_string(),
                token_list: to_string_vec(token_list),
                target: target.map(|x| x.to_string()),
            },
            amount,
            asset,
        )
    }

    #[track_caller]
    fn transceiver_try_accept(
        &mut self,
        sender: ProjectAccount,
        transceiver_address: &Addr,
        msg: &str,
        timestamp: Timestamp,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                transceiver_address.to_owned(),
                &ExecuteMsg::Accept {
                    msg: msg.to_string(),
                    timestamp,
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_query_config(&self, transceiver_address: &Addr) -> StdResult<Config> {
        self.app
            .wrap()
            .query_wasm_smart(transceiver_address, &QueryMsg::Config {})
    }

    #[track_caller]
    fn transceiver_query_pause_state(&self, transceiver_address: &Addr) -> StdResult<bool> {
        self.app
            .wrap()
            .query_wasm_smart(transceiver_address, &QueryMsg::PauseState {})
    }

    #[track_caller]
    fn transceiver_query_outposts(&self, transceiver_address: &Addr) -> StdResult<Vec<String>> {
        self.app
            .wrap()
            .query_wasm_smart(transceiver_address, &QueryMsg::Outposts {})
    }

    #[track_caller]
    fn transceiver_query_collection(
        &self,
        transceiver_address: &Addr,
        hub_collection: Option<&Addr>,
        home_collection: Option<&Addr>,
    ) -> StdResult<Collection> {
        self.app.wrap().query_wasm_smart(
            transceiver_address,
            &QueryMsg::Collection {
                hub_collection: hub_collection.map(|x| x.to_string()),
                home_collection: home_collection.map(|x| x.to_string()),
            },
        )
    }

    #[track_caller]
    fn transceiver_query_collection_list(
        &self,
        transceiver_address: &Addr,
    ) -> StdResult<Vec<Collection>> {
        self.app
            .wrap()
            .query_wasm_smart(transceiver_address, &QueryMsg::CollectionList {})
    }

    #[track_caller]
    fn transceiver_query_channel_list(
        &self,
        transceiver_address: &Addr,
    ) -> StdResult<Vec<Channel>> {
        self.app
            .wrap()
            .query_wasm_smart(transceiver_address, &QueryMsg::ChannelList {})
    }
}
