use cosmwasm_std::{Addr, StdResult, Uint128};
use cw_multi_test::{AppResponse, ContractWrapper, Executor};

use serde::Serialize;
use strum::IntoEnumIterator;

use snb_base::{error::parse_err, transceiver::types::TransceiverType};

use crate::helpers::suite::{
    core::Project,
    types::{GetDecimals, ProjectAccount, ProjectToken},
};

pub trait WithCodes {
    // store packages
    fn store_cw20_base_code(&mut self) -> u64;
    fn store_cw721_base_code(&mut self) -> u64;

    // store contracts
    fn store_nft_minter_code(&mut self) -> u64;
    fn store_transceiver_code(&mut self) -> u64;

    // instantiate packages
    fn instantiate_cw20_base_token(&mut self, code_id: u64, project_token: ProjectToken) -> Addr;
    fn instantiate_cw721_base_token(&mut self, code_id: u64) -> Addr;

    // instantiate contracts
    fn instantiate_nft_minter(
        &mut self,
        nft_minter_code_id: u64,
        transceiver_hub: &Addr,
        cw721_code_id: u64,
    ) -> Addr;

    #[allow(clippy::too_many_arguments)]
    fn instantiate_transceiver(
        &mut self,
        transceiver_code_id: u64,
        nft_minter: Option<&Addr>,
        hub_address: Option<&Addr>,
        is_retranslation_outpost: bool,
        transceiver_type: TransceiverType,
        token_limit: Option<u8>,
        min_ntrn_ibc_fee: Option<u128>,
    ) -> Addr;

    fn migrate_contract(
        &mut self,
        sender: ProjectAccount,
        contract_address: Addr,
        contract_new_code_id: u64,
        migrate_msg: impl Serialize,
    ) -> StdResult<AppResponse>;
}

impl WithCodes for Project {
    // store packages
    fn store_cw20_base_code(&mut self) -> u64 {
        self.app.store_code(Box::new(ContractWrapper::new(
            cw20_base::contract::execute,
            cw20_base::contract::instantiate,
            cw20_base::contract::query,
        )))
    }

    fn store_cw721_base_code(&mut self) -> u64 {
        self.app.store_code(Box::new(ContractWrapper::new(
            cw721_base::entry::execute,
            cw721_base::entry::instantiate,
            cw721_base::entry::query,
        )))
    }

    // store contracts
    fn store_nft_minter_code(&mut self) -> u64 {
        self.app.store_code(Box::new(
            ContractWrapper::new(
                nft_minter::contract::execute,
                nft_minter::contract::instantiate,
                nft_minter::contract::query,
            )
            .with_reply(nft_minter::contract::reply)
            .with_migrate(nft_minter::contract::migrate),
        ))
    }

    fn store_transceiver_code(&mut self) -> u64 {
        self.app.store_code(Box::new(
            ContractWrapper::new(
                transceiver::contract::execute,
                transceiver::contract::instantiate,
                transceiver::contract::query,
            )
            .with_migrate(transceiver::contract::migrate),
        ))
    }

    // instantiate packages
    fn instantiate_cw20_base_token(&mut self, code_id: u64, project_token: ProjectToken) -> Addr {
        let symbol = "TOKEN".to_string();

        let initial_balances: Vec<cw20::Cw20Coin> = ProjectAccount::iter()
            .map(|project_account| {
                let amount = project_account.get_initial_funds_amount()
                    * 10u128.pow(project_token.get_decimals() as u32);

                cw20::Cw20Coin {
                    address: project_account.to_string(),
                    amount: Uint128::from(amount),
                }
            })
            .collect();

        self.instantiate_contract(
            code_id,
            "token",
            &cw20_base::msg::InstantiateMsg {
                name: format!("cw20-base token {}", symbol),
                symbol,
                decimals: project_token.get_decimals(),
                initial_balances,
                mint: None,
                marketing: None,
            },
        )
    }

    fn instantiate_cw721_base_token(&mut self, code_id: u64) -> Addr {
        let symbol = "NFT XYZ".to_string(); // max 10 tokens

        self.instantiate_contract(
            code_id,
            "nft xyz",
            &cw721_base::msg::InstantiateMsg {
                name: format!("cw721-base token {}", symbol),
                symbol,
                minter: ProjectAccount::Owner.to_string(),
            },
        )
    }

    // instantiate contracts
    fn instantiate_nft_minter(
        &mut self,
        nft_minter_code_id: u64,
        transceiver_hub: &Addr,
        cw721_code_id: u64,
    ) -> Addr {
        self.instantiate_contract(
            nft_minter_code_id,
            "nft_minter",
            &snb_base::nft_minter::msg::InstantiateMsg {
                transceiver_hub: transceiver_hub.to_string(),
                cw721_code_id,
            },
        )
    }

    fn instantiate_transceiver(
        &mut self,
        transceiver_code_id: u64,
        nft_minter: Option<&Addr>,
        hub_address: Option<&Addr>,
        is_retranslation_outpost: bool,
        transceiver_type: TransceiverType,
        token_limit: Option<u8>,
        min_ntrn_ibc_fee: Option<u128>,
    ) -> Addr {
        self.instantiate_contract(
            transceiver_code_id,
            "transceiver",
            &snb_base::transceiver::msg::InstantiateMsg {
                nft_minter: nft_minter.map(|x| x.to_string()),
                hub_address: hub_address.map(|x| x.to_string()),
                is_retranslation_outpost,
                transceiver_type,
                token_limit,
                min_ntrn_ibc_fee: min_ntrn_ibc_fee.map(Uint128::new),
            },
        )
    }

    fn migrate_contract(
        &mut self,
        sender: ProjectAccount,
        contract_address: Addr,
        contract_new_code_id: u64,
        migrate_msg: impl Serialize,
    ) -> StdResult<AppResponse> {
        self.app
            .migrate_contract(
                sender.into(),
                contract_address,
                &migrate_msg,
                contract_new_code_id,
            )
            .map_err(parse_err)
    }
}
