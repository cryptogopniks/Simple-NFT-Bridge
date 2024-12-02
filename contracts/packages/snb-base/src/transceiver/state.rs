use cw_storage_plus::{Item, Map};

use super::types::{Config, TransferAdminState};

pub const CONTRACT_NAME: &str = "snb-transceiver";

pub const CONFIG: Item<Config> = Item::new("config");

/// Stores the state of changing admin process
pub const TRANSFER_ADMIN_STATE: Item<TransferAdminState> = Item::new("transfer_admin_state");
pub const OUTPOSTS: Item<Vec<String>> = Item::new("outposts");
/// home_chain_address by hub_chain_address
pub const COLLECTIONS: Map<&str, String> = Map::new("collections");
