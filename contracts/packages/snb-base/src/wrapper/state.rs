use cw_storage_plus::Item;

use crate::transceiver::types::TransferAdminState;

use super::types::{Collection, Config};

pub const CONTRACT_NAME: &str = "goplend-wrapper";

/// Stores user functions pause flag
pub const IS_PAUSED: Item<bool> = Item::new("is_paused");
pub const CONFIG: Item<Config> = Item::new("config");
pub const TRANSFER_ADMIN_STATE: Item<TransferAdminState> = Item::new("transfer_admin_state");
pub const COLLECTIONS: Item<Vec<Collection>> = Item::new("collections");
