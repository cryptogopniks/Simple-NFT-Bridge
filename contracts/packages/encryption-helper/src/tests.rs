use crate::base::{decrypt, encrypt};

use speculoos::assert_that;

#[test]
fn default_ecryption() {
    const MESSAGE: &str = "The secret message #1. Don't share it!⚠️";
    const ENC_KEY: &[u8; 32] = &[1; 32];
    const NONCE: &str = "unique nonce";

    let encrypted = encrypt(MESSAGE, ENC_KEY, NONCE).unwrap();
    let decrypted = decrypt(&encrypted, ENC_KEY, NONCE).unwrap();

    assert_that(&encrypted).is_not_equal_to(&decrypted);
    assert_that(&MESSAGE).is_equal_to(&*decrypted);
}
