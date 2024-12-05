use cosmwasm_std::{Addr, StdResult, Timestamp, Uint128};
use cw_multi_test::{AppResponse, Executor};

use snb_base::{
    error::parse_err,
    transceiver::types::{Channel, Collection, TransceiverType},
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
        transceiver: TransceiverType,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_unpause(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_accept_admin_role(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
    ) -> StdResult<AppResponse>;

    #[allow(clippy::too_many_arguments)]
    fn transceiver_try_update_config(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
        admin: Option<ProjectAccount>,
        nft_minter: Option<&Addr>,
        hub_address: Option<&Addr>,
        token_limit: Option<u8>,
        min_ntrn_ibc_fee: Option<u128>,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_add_collection(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
        hub_collection: impl ToString,
        home_collection: impl ToString,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_remove_collection(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
        hub_collection: impl ToString,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_set_channel(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
        prefix: &str,
        from_hub: &str,
        to_hub: &str,
    ) -> StdResult<AppResponse>;

    #[allow(clippy::too_many_arguments)]
    fn transceiver_try_send(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
        hub_collection: impl ToString,
        token_list: &[&str],
        target: Option<Addr>,
        amount: u128,
        asset: impl Into<ProjectAsset>,
    ) -> StdResult<AppResponse>;

    fn transceiver_try_accept(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
        msg: &str,
        timestamp: Timestamp,
    ) -> StdResult<AppResponse>;

    fn transceiver_query_config(&self, transceiver: TransceiverType) -> StdResult<Config>;

    fn transceiver_query_pause_state(&self, transceiver: TransceiverType) -> StdResult<bool>;

    fn transceiver_query_outposts(&self, transceiver: TransceiverType) -> StdResult<Vec<String>>;

    fn transceiver_query_collection(
        &self,
        transceiver: TransceiverType,
        hub_collection: Option<&Addr>,
        home_collection: Option<&Addr>,
    ) -> StdResult<Collection>;

    fn transceiver_query_collection_list(
        &self,
        transceiver: TransceiverType,
    ) -> StdResult<Vec<Collection>>;

    fn transceiver_query_channel_list(
        &self,
        transceiver: TransceiverType,
    ) -> StdResult<Vec<Channel>>;
}

impl TransceiverExtension for Project {
    #[track_caller]
    fn transceiver_try_pause(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
    ) -> StdResult<AppResponse> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .execute_contract(
                sender.into(),
                transceiver_address,
                &ExecuteMsg::Pause {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_try_unpause(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
    ) -> StdResult<AppResponse> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .execute_contract(
                sender.into(),
                transceiver_address,
                &ExecuteMsg::Unpause {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_try_accept_admin_role(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
    ) -> StdResult<AppResponse> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .execute_contract(
                sender.into(),
                transceiver_address,
                &ExecuteMsg::AcceptAdminRole {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_try_update_config(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
        admin: Option<ProjectAccount>,
        nft_minter: Option<&Addr>,
        hub_address: Option<&Addr>,
        token_limit: Option<u8>,
        min_ntrn_ibc_fee: Option<u128>,
    ) -> StdResult<AppResponse> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .execute_contract(
                sender.into(),
                transceiver_address,
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
        transceiver: TransceiverType,
        hub_collection: impl ToString,
        home_collection: impl ToString,
    ) -> StdResult<AppResponse> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .execute_contract(
                sender.into(),
                transceiver_address,
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
        transceiver: TransceiverType,
        hub_collection: impl ToString,
    ) -> StdResult<AppResponse> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .execute_contract(
                sender.into(),
                transceiver_address,
                &ExecuteMsg::RemoveCollection {
                    hub_collection: hub_collection.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_try_set_channel(
        &mut self,
        sender: ProjectAccount,
        transceiver: TransceiverType,
        prefix: &str,
        from_hub: &str,
        to_hub: &str,
    ) -> StdResult<AppResponse> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .execute_contract(
                sender.into(),
                transceiver_address,
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
        transceiver: TransceiverType,
        hub_collection: impl ToString,
        token_list: &[&str],
        target: Option<Addr>,
        amount: u128,
        asset: impl Into<ProjectAsset>,
    ) -> StdResult<AppResponse> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        add_funds_to_exec_msg(
            self,
            sender,
            &transceiver_address,
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
        transceiver: TransceiverType,
        msg: &str,
        timestamp: Timestamp,
    ) -> StdResult<AppResponse> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .execute_contract(
                sender.into(),
                transceiver_address,
                &ExecuteMsg::Accept {
                    msg: msg.to_string(),
                    timestamp,
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn transceiver_query_config(&self, transceiver: TransceiverType) -> StdResult<Config> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .wrap()
            .query_wasm_smart(transceiver_address, &QueryMsg::Config {})
    }

    #[track_caller]
    fn transceiver_query_pause_state(&self, transceiver: TransceiverType) -> StdResult<bool> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .wrap()
            .query_wasm_smart(transceiver_address, &QueryMsg::PauseState {})
    }

    #[track_caller]
    fn transceiver_query_outposts(&self, transceiver: TransceiverType) -> StdResult<Vec<String>> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .wrap()
            .query_wasm_smart(transceiver_address, &QueryMsg::Outposts {})
    }

    #[track_caller]
    fn transceiver_query_collection(
        &self,
        transceiver: TransceiverType,
        hub_collection: Option<&Addr>,
        home_collection: Option<&Addr>,
    ) -> StdResult<Collection> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

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
        transceiver: TransceiverType,
    ) -> StdResult<Vec<Collection>> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .wrap()
            .query_wasm_smart(transceiver_address, &QueryMsg::CollectionList {})
    }

    #[track_caller]
    fn transceiver_query_channel_list(
        &self,
        transceiver: TransceiverType,
    ) -> StdResult<Vec<Channel>> {
        let transceiver_address = match transceiver {
            TransceiverType::Hub => self.get_transceiver_hub_address(),
            TransceiverType::Outpost => self.get_transceiver_outpost_address(),
        };

        self.app
            .wrap()
            .query_wasm_smart(transceiver_address, &QueryMsg::ChannelList {})
    }
}
