use cosmwasm_std::{StdError, StdResult};

use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};

use snb_base::{
    constants::ENC_KEY_LEN,
    converters::{str_to_u8_vec, u8_vec_to_str, utf8_vec_to_str},
};

fn get_cipher(enc_key: &[u8; ENC_KEY_LEN]) -> StdResult<Aes256GcmSiv> {
    let generic_array: &GenericArray<u8, _> = &GenericArray::from(*enc_key);

    Ok(Aes256GcmSiv::new(generic_array))
}

pub fn encrypt(msg: &str, enc_key: &[u8; ENC_KEY_LEN], nonce: &str) -> StdResult<String> {
    let nonce: &GenericArray<u8, _> = Nonce::from_slice(nonce.as_bytes());
    let cipher = get_cipher(enc_key)?;

    cipher
        .encrypt(nonce, msg.as_bytes())
        .map_err(|e| StdError::GenericErr { msg: e.to_string() })
        .map(|bytes| u8_vec_to_str(&bytes))
}

pub fn decrypt(enc_msg: &str, enc_key: &[u8; ENC_KEY_LEN], nonce: &str) -> StdResult<String> {
    let nonce: &GenericArray<u8, _> = Nonce::from_slice(nonce.as_bytes());
    let cipher = get_cipher(enc_key)?;

    cipher
        .decrypt(nonce, str_to_u8_vec(enc_msg).as_ref())
        .map_err(|e| StdError::GenericErr { msg: e.to_string() })
        .and_then(|decrypted| utf8_vec_to_str(&decrypted))
}
