use cw_storage_plus::Item;

use super::types::{Channel, Collection, Config, TransferAdminState};

pub const CONTRACT_NAME: &str = "snb-transceiver";

pub const TRANSFER_ADMIN_TIMEOUT: u64 = 7 * 24 * 3600;
pub const TOKEN_LIMIT: u8 = 10;
// https://rest-kralum.neutron-1.neutron.org/neutron-org/neutron/feerefunder/params
pub const MIN_NTRN_IBC_FEE: u128 = 100_000;

// TODO: replace before storing the contract
pub const ENC_KEY: &str = "qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq";

pub const DENOM_NTRN: &str = "untrn";

pub const PREFIX_NEUTRON: &str = "neutron";
pub const PREFIX_STARGAZE: &str = "stars";

pub const CHANNEL_STARGAZE_NEUTRON: &str = "channel-191";
pub const CHANNEL_NEUTRON_STARGAZE: &str = "channel-18";

pub const PORT: &str = "transfer";
pub const IBC_TIMEOUT: u64 = 10 * 60;

pub const IS_PAUSED: Item<bool> = Item::new("is_paused");
pub const CONFIG: Item<Config> = Item::new("config");

/// Stores the state of changing admin process
pub const TRANSFER_ADMIN_STATE: Item<TransferAdminState> = Item::new("transfer_admin_state");
pub const OUTPOSTS: Item<Vec<String>> = Item::new("outposts");
pub const COLLECTIONS: Item<Vec<Collection>> = Item::new("collections");
pub const CHANNELS: Item<Vec<Channel>> = Item::new("channels");
