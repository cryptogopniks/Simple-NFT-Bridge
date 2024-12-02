use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use super::types::{Config, TransferAdminState};

pub const CONTRACT_NAME: &str = "snb-nft-minter";

pub const SAVE_CW721_ADDRESS_REPLY: u64 = 0;

pub const CONFIG: Item<Config> = Item::new("config");

/// Stores the state of changing admin process
pub const TRANSFER_ADMIN_STATE: Item<TransferAdminState> = Item::new("transfer_admin_state");
/// colletion name by address
pub const COLLECTIONS: Map<&Addr, String> = Map::new("collections");
