use cosmwasm_std::{from_json, to_json_vec, StdResult, Timestamp};

use serde::{de::DeserializeOwned, Serialize};

use snb_base::{
    constants::ENC_KEY_LEN,
    converters::{timestamp_to_nonce, utf8_vec_to_str},
    private_communication::types::EncryptedResponse,
};

use crate::base::{decrypt, encrypt};

pub fn serialize<T: ?Sized + Serialize>(data: &T) -> StdResult<String> {
    utf8_vec_to_str(&to_json_vec(data)?)
}

pub fn deserialize<T: DeserializeOwned>(data: &str) -> StdResult<T> {
    from_json::<T>(data.as_bytes())
}

pub fn decrypt_deserialize<T, F>(enc_key: &F, timestamp: &Timestamp, value: &str) -> StdResult<T>
where
    T: DeserializeOwned,
    F: Into<[u8; ENC_KEY_LEN]> + Clone,
{
    let encryption_key = &enc_key.to_owned().into();
    let nonce = &timestamp_to_nonce(timestamp);
    let decrypted_data = &decrypt(value, encryption_key, nonce)?;

    deserialize(decrypted_data)
}

pub fn serialize_encrypt<T, F>(
    enc_key: &F,
    timestamp: &Timestamp,
    value: &T,
) -> StdResult<EncryptedResponse>
where
    T: ?Sized + Serialize,
    F: Into<[u8; ENC_KEY_LEN]> + Clone,
{
    let encryption_key = &enc_key.to_owned().into();
    let nonce = &timestamp_to_nonce(timestamp);
    let serialized_value = &serialize(value)?;
    let encrypted_value = encrypt(serialized_value, encryption_key, nonce)?;

    Ok(EncryptedResponse {
        value: encrypted_value,
        timestamp: timestamp.to_owned(),
    })
}
