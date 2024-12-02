use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Env, Timestamp};

#[cw_serde]
pub struct EncryptedResponse {
    pub value: String,
    pub timestamp: Timestamp,
}

#[cw_serde]
pub struct ExecuteMsgWithTimestamp<T: Clone> {
    pub msg: T,
    pub timestamp: Timestamp,
}

impl<T: Clone> ExecuteMsgWithTimestamp<T> {
    pub fn new(env: &Env, msg: &T) -> Self {
        Self {
            msg: msg.to_owned(),
            timestamp: env.block.time,
        }
    }
}
