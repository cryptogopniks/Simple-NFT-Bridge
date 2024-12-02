use cw_storage_plus::Item;

use super::types::{Collection, Config, TransferAdminState};

pub const CONTRACT_NAME: &str = "snb-transceiver";

pub const TRANSFER_ADMIN_TIMEOUT: u64 = 7 * 24 * 3600;
pub const TOKEN_LIMIT: u8 = 10;
// TODO: replace before storing the contract
pub const ENC_KEY: &str = "qqqqqqqqqqqq";

pub const IS_PAUSED: Item<bool> = Item::new("is_paused");
pub const CONFIG: Item<Config> = Item::new("config");

/// Stores the state of changing admin process
pub const TRANSFER_ADMIN_STATE: Item<TransferAdminState> = Item::new("transfer_admin_state");
pub const OUTPOSTS: Item<Vec<String>> = Item::new("outposts");
pub const COLLECTIONS: Item<Vec<Collection>> = Item::new("collections");
