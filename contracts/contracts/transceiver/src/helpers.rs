use cosmwasm_std::{
    coins, to_json_string, Addr, Coin, CosmosMsg, Deps, StdResult, Storage, Timestamp, Uint128,
};

use anybuf::Anybuf;

use snb_base::{
    error::ContractError,
    transceiver::{
        state::{DENOM_NTRN, IS_PAUSED, PORT},
        types::{Channel, IbcMemo},
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

pub fn get_ibc_transfer_msg(
    channel: &str,
    denom_in: &str,
    amount_in: Uint128,
    sender: &Addr,
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
                &Anybuf::new()
                    .append_string(1, denom_in)
                    .append_string(2, amount_in.to_string()),
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

pub fn get_neutron_ibc_transfer_msg(
    channel: &str,
    denom_in: &str,
    amount_in: Uint128,
    sender: &Addr,
    contract_address: &str,
    timeout_timestamp_ns: u64,
    ibc_transfer_memo: &str,
    min_ntrn_ibc_fee: Uint128,
) -> CosmosMsg {
    let recv_fee = &coins(0, DENOM_NTRN);
    let ack_fee = &coins(min_ntrn_ibc_fee.u128(), DENOM_NTRN);
    let timeout_fee = &coins(min_ntrn_ibc_fee.u128(), DENOM_NTRN);
    let fee_message = &create_fee_message(recv_fee, ack_fee, timeout_fee);

    // https://github.com/neutron-org/neutron/blob/main/proto/neutron/transfer/v1/tx.proto#L25
    CosmosMsg::Stargate {
        type_url: "/ibc.applications.transfer.v1.MsgTransfer".to_string(),
        value: Anybuf::new()
            // source port
            .append_string(1, "transfer")
            // source channel (IBC Channel on your network side)
            .append_string(2, channel)
            // token
            .append_message(
                3,
                &Anybuf::new()
                    .append_string(1, denom_in)
                    .append_string(2, amount_in.to_string()),
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
            // fee funder
            .append_message(9, fee_message)
            .into_vec()
            .into(),
    }
}

fn create_fee_message(
    recv_fee: &Vec<Coin>,
    ack_fee: &Vec<Coin>,
    timeout_fee: &Vec<Coin>,
) -> Anybuf {
    Anybuf::new()
        .append_message(1, &create_coin_list(recv_fee))
        .append_message(2, &create_coin_list(ack_fee))
        .append_message(3, &create_coin_list(timeout_fee))
}

fn create_coin_list(coins: &Vec<Coin>) -> Anybuf {
    let mut coin_list = Anybuf::new();
    for coin in coins {
        coin_list = coin_list.append_message(
            1,
            &Anybuf::new()
                .append_string(1, coin.denom.to_string())
                .append_string(2, coin.amount.to_string()),
        );
    }
    coin_list
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
    contract_address: &Addr,
    hub_address: &str,
    home_collection: &str,
    outpost_list: &[String],
    channel_list: &[Channel],
) -> StdResult<(String, String)> {
    let is_hub_sender = contract_address.as_str() == hub_address;
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
