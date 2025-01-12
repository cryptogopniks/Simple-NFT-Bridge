use cosmwasm_std::{
    coins, to_json_string, Addr, Coin, CosmosMsg, Deps, Env, StdResult, Storage, Timestamp, Uint128,
};

use anybuf::Anybuf;

use snb_base::{
    assets::Token,
    converters::get_addr_by_prefix,
    error::ContractError,
    transceiver::{
        state::{DENOM_NTRN, IS_PAUSED, PORT},
        types::{
            Channel, Config, IbcMemo, TransmissionDescription, TransmissionDirection,
            TransmissionInfo, TransmissionMode, TransmissionRoute, TransmissionStage,
        },
    },
};

/// user actions are disabled when the contract is paused
pub fn check_pause_state(storage: &dyn Storage) -> StdResult<()> {
    if IS_PAUSED.load(storage)? {
        Err(ContractError::ContractIsPaused)?;
    }

    Ok(())
}

pub fn check_tokens_holder(
    deps: Deps,
    holder: &Addr,
    collection_address: &str,
    token_id_list: &[String],
) -> StdResult<()> {
    const MAX_LIMIT: u32 = 100;
    const ITER_LIMIT: u32 = 50;

    let mut token_list: Vec<String> = vec![];
    let mut token_amount_sum: u32 = 0;
    let mut i: u32 = 0;
    let mut last_token: Option<String> = None;

    while (i == 0 || token_amount_sum == MAX_LIMIT) && i < ITER_LIMIT {
        i += 1;

        let query_tokens_msg = cw721::Cw721QueryMsg::Tokens {
            owner: holder.to_string(),
            start_after: last_token,
            limit: Some(MAX_LIMIT),
        };

        let cw721::TokensResponse { tokens } = deps
            .querier
            .query_wasm_smart(collection_address, &query_tokens_msg)?;

        for token in tokens.clone() {
            token_list.push(token);
        }

        token_amount_sum = tokens.len() as u32;
        last_token = tokens.last().cloned();
    }

    let are_tokens_owned = token_id_list.iter().all(|x| token_list.contains(x));

    if !are_tokens_owned {
        Err(ContractError::NftIsNotFound)?;
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn get_ibc_transfer_msg(
    channel: &str,
    denom_in: &str,
    amount_in: Uint128,
    sender: &str,
    contract_address: &str,
    timeout_timestamp_ns: u64,
    ibc_transfer_memo: &str,
) -> CosmosMsg {
    CosmosMsg::Stargate {
        type_url: "/ibc.applications.transfer.v1.MsgTransfer".to_string(),
        value: Anybuf::new()
            // source port
            .append_string(1, PORT)
            // source channel (IBC Channel on your network side)
            .append_string(2, channel)
            // token
            .append_message(
                3,
                get_coin_msgs(&coins(amount_in.u128(), denom_in))
                    .first()
                    .unwrap(),
            )
            // sender
            .append_string(4, sender)
            // recipient
            .append_string(5, contract_address)
            // TimeoutHeight
            .append_message(6, &Anybuf::new().append_uint64(1, 0).append_uint64(2, 0))
            // TimeoutTimestamp
            .append_uint64(7, timeout_timestamp_ns)
            // IBC Hook memo
            .append_string(8, ibc_transfer_memo)
            .into_vec()
            .into(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn get_neutron_ibc_transfer_msg(
    channel: &str,
    denom_in: &str,
    amount_in: Uint128,
    sender: &str,
    contract_address: &str,
    timeout_timestamp_ns: u64,
    ibc_transfer_memo: &str,
    min_ntrn_ibc_fee: Uint128,
) -> CosmosMsg {
    let recv_fee: &Vec<Coin> = &vec![];
    let ack_fee = &coins(min_ntrn_ibc_fee.u128(), DENOM_NTRN);
    let timeout_fee = &coins(min_ntrn_ibc_fee.u128(), DENOM_NTRN);

    // https://github.com/neutron-org/neutron-std/blob/main/packages/neutron-std/src/types/neutron/transfer.rs
    // https://github.com/neutron-org/neutron/blob/main/proto/neutron/transfer/v1/tx.proto#L25
    CosmosMsg::Stargate {
        type_url: "/neutron.transfer.MsgTransfer".to_string(),
        value: Anybuf::new()
            // source port
            .append_string(1, "transfer")
            // source channel (IBC Channel on your network side)
            .append_string(2, channel)
            // token
            .append_message(
                3,
                get_coin_msgs(&coins(amount_in.u128(), denom_in))
                    .first()
                    .unwrap(),
            )
            // sender
            .append_string(4, sender)
            // recipient
            .append_string(5, contract_address)
            // TimeoutHeight
            .append_message(6, &Anybuf::new().append_uint64(1, 0).append_uint64(2, 0))
            // TimeoutTimestamp
            .append_uint64(7, timeout_timestamp_ns)
            // IBC Hook memo
            .append_string(8, ibc_transfer_memo)
            // fee refunder
            .append_message(
                9,
                &Anybuf::new()
                    .append_repeated_message(1, &get_coin_msgs(recv_fee))
                    .append_repeated_message(2, &get_coin_msgs(ack_fee))
                    .append_repeated_message(3, &get_coin_msgs(timeout_fee)),
            )
            .into_vec()
            .into(),
    }
}

fn get_coin_msgs(coin_list: &[Coin]) -> Vec<Anybuf> {
    coin_list
        .iter()
        .map(|coin| {
            Anybuf::new()
                .append_string(1, coin.denom.clone())
                .append_string(2, coin.amount.to_string())
        })
        .collect()
}

pub fn get_ibc_transfer_memo(
    contract_address: &str,
    msg: &str,
    timestamp: Timestamp,
) -> StdResult<String> {
    to_json_string(&IbcMemo::Wasm {
        contract: contract_address.to_string(),
        msg: snb_base::transceiver::msg::ExecuteMsg::Accept {
            msg: msg.to_string(),
            timestamp,
        },
    })
}

/// returns (ibc_channel, target_transceiver)
pub fn get_channel_and_transceiver(
    contract_address: &str,
    hub_address: &str,
    home_collection: &str,
    outpost_list: &[String],
    channel_list: &[Channel],
) -> StdResult<(String, String)> {
    let is_hub_sender = contract_address == hub_address;
    let (home_prefix, _) = split_address(home_collection);
    let channel = channel_list
        .iter()
        .find(|x| x.prefix == home_prefix)
        .ok_or(ContractError::ChannelIsNotFound)?;

    if is_hub_sender {
        let outpost = outpost_list
            .iter()
            .find(|x| {
                let (prefix, _) = split_address(x);
                prefix == home_prefix
            })
            .ok_or(ContractError::OutpostIsNotFound)?;

        Ok((channel.from_hub.clone(), outpost.to_owned()))
    } else {
        Ok((channel.to_hub.clone(), hub_address.to_owned()))
    }
}

pub fn split_address(address: impl ToString) -> (String, String) {
    let address = address.to_string();
    let (prefix, postfix) = address.split_once('1').unwrap();
    (prefix.to_string(), postfix.to_string())
}

// Possible options
//
// 1a) HomeOutpost (chain A) -> Hub (chain B)
// 1b) Hub (chain A) -> HomeOutpost (chain B)
//
// 2a) HomeOutpost (chain A) -> Hub (chain A)
// 2b) Hub (chain A) -> HomeOutpost (chain A)
//
// 3a) HomeOutpost (chain A) -> RetranslationOutpost (chain B) -> Hub (chain C)
// 3b) Hub (chain A) -> RetranslationOutpost (chain B) -> HomeOutpost (chain C)
//
// 4a) HomeOutpost (chain A) -> RetranslationOutpost (chain A) -> Hub (chain A)
// 4b) Hub (chain A) -> RetranslationOutpost (chain A) -> HomeOutpost (chain A)
pub fn get_transmission_info(
    config: &Config,
    retranslation_outpost: &Option<String>,
    target: &Option<String>,
    home_collection: &str,
    outpost_list: &[String],
    transceiver: &str,
    sender_transceiver: &str,
) -> StdResult<TransmissionInfo> {
    let is_hub_sender = config.transceiver_type.is_hub();

    let (home_prefix, _) = &split_address(home_collection);

    let hub = config.hub_address.clone();
    // if outpost list is empty then transceiver is home_outpost
    let home_outpost = outpost_list
        .iter()
        .cloned()
        .find(|x| {
            let (prefix, _) = &split_address(x);
            prefix == home_prefix
        })
        .unwrap_or(sender_transceiver.to_string());

    let target = target.clone().unwrap_or(if is_hub_sender {
        home_outpost.to_string()
    } else {
        hub.to_string()
    });

    let whitelist = match retranslation_outpost {
        Some(retranslation_outpost) => vec![&hub, &home_outpost, retranslation_outpost],
        None => vec![&hub, &home_outpost],
    };
    if !whitelist.contains(&&target) {
        Err(ContractError::WrongTargetAddress)?;
    }

    let (transceiver_prefix, _) = &split_address(&transceiver);
    let (target_prefix, _) = &split_address(&target);

    let mode = if transceiver_prefix == target_prefix {
        TransmissionMode::Local
    } else {
        TransmissionMode::Interchain
    };

    let direction = if is_hub_sender || target != hub {
        TransmissionDirection::FromHub
    } else {
        TransmissionDirection::ToHub
    };

    let stage = match retranslation_outpost {
        Some(retranslation_outpost) => {
            if &transceiver == retranslation_outpost {
                TransmissionStage::Second
            } else {
                TransmissionStage::First
            }
        }
        None => TransmissionStage::First,
    };

    let route = if (is_hub_sender && target == home_outpost) || (!is_hub_sender && target == hub) {
        TransmissionRoute::Short
    } else {
        TransmissionRoute::Long
    };

    Ok(TransmissionInfo {
        description: TransmissionDescription {
            mode,
            direction,
            stage,
            route,
        },
        home_outpost,
        hub,
        transceiver: transceiver.to_string(),
        target,
    })
}

pub fn check_token_list(config: &Config, token_list: &[String]) -> StdResult<()> {
    let mut tokens = token_list.to_vec();
    tokens.sort_unstable();
    tokens.dedup();

    if tokens.len() != token_list.len() {
        Err(ContractError::NftDuplication)?;
    }

    if token_list.is_empty() {
        Err(ContractError::EmptyTokenList)?;
    }

    if token_list.len() > config.token_limit as usize {
        Err(ContractError::ExceededTokenLimit)?;
    }

    Ok(())
}

/// we need 1 token for regular ibc transfer or fee + 1 for ibc transfer from hub
pub fn get_checked_amount_in(
    config: &Config,
    asset_amount: Uint128,
    asset_info: &Token,
    transmission_description: &TransmissionDescription,
) -> StdResult<Uint128> {
    let amount_in = Uint128::one();
    let required_asset_amount =
        if transmission_description.mode.is_interchain() && config.transceiver_type.is_hub() {
            if asset_info.try_get_native()? != DENOM_NTRN {
                Err(ContractError::WrongAssetType)?;
            }

            amount_in + config.min_ntrn_ibc_fee
        } else {
            amount_in
        };

    if asset_amount != required_asset_amount {
        Err(ContractError::WrongFundsCombination)?;
    }

    Ok(amount_in)
}

pub fn validate_any_address(deps: Deps, env: &Env, address: &str) -> StdResult<Addr> {
    let (prefix, _) = split_address(env.contract.address.to_string());
    deps.api
        .addr_validate(&get_addr_by_prefix(&address, &prefix)?)
}
