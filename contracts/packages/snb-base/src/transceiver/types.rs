use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Binary, Uint128};

#[cw_serde]
pub struct TransmissionInfo {
    pub description: TransmissionDescription,
    pub prefix: Prefix,
    pub home_outpost: String,
    pub hub: String,
    pub transceiver: String,
    pub target: String,
}

#[cw_serde]
pub struct TransmissionDescription {
    pub mode: TransmissionMode,
    pub direction: TransmissionDirection,
    pub stage: TransmissionStage,
    pub route: TransmissionRoute,
}

#[cw_serde]
pub struct Prefix {
    pub hub: String,
    pub home_outpost: String,
    pub retranslation_outpost: Option<String>,
}

#[cw_serde]
pub enum TransmissionMode {
    Local,
    Interchain,
}

impl TransmissionMode {
    pub fn is_local(&self) -> bool {
        self == &Self::Local
    }

    pub fn is_interchain(&self) -> bool {
        self == &Self::Interchain
    }
}

#[cw_serde]
pub enum TransmissionDirection {
    FromHub,
    ToHub,
}

impl TransmissionDirection {
    pub fn is_from_hub(&self) -> bool {
        self == &Self::FromHub
    }

    pub fn is_to_hub(&self) -> bool {
        self == &Self::ToHub
    }
}

#[cw_serde]
pub enum TransmissionStage {
    First,
    Second,
}

impl TransmissionStage {
    pub fn is_first(&self) -> bool {
        self == &Self::First
    }

    pub fn is_second(&self) -> bool {
        self == &Self::Second
    }
}

#[cw_serde]
pub enum TransmissionRoute {
    Short,
    Long,
}

impl TransmissionRoute {
    pub fn is_short(&self) -> bool {
        self == &Self::Short
    }

    pub fn is_long(&self) -> bool {
        self == &Self::Long
    }
}

#[cw_serde]
pub enum TransceiverType {
    /// it's contract on Neutron by design
    Hub,
    Outpost,
}

impl TransceiverType {
    pub fn is_hub(&self) -> bool {
        self == &Self::Hub
    }

    pub fn is_outpost(&self) -> bool {
        self == &Self::Outpost
    }
}

#[cw_serde]
pub struct Collection {
    pub home_collection: String,
    pub hub_collection: String,
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub nft_minter: String,
    pub hub_address: String,
    pub transceiver_type: TransceiverType,
    pub token_limit: u8,
    pub min_ntrn_ibc_fee: Uint128,
}

#[cw_serde]
pub struct TransferAdminState {
    pub new_admin: Addr,
    pub deadline: u64,
}

#[cw_serde]
pub struct Packet {
    /// hub or home outpost
    pub sender: String,
    /// NFT owner
    pub recipient: String,
    pub hub_collection: String,
    pub home_collection: String,
    pub token_list: Vec<String>,
}

#[cw_serde]
pub enum IbcMemo<M> {
    Forward {
        channel: String,
        port: String,
        receiver: String,
        retries: u8,
        timeout: u64,
    },
    Wasm {
        contract: String,
        msg: M,
    },
}

/// short transmission example:                                                         \
/// stargaze(channel-191) - (channel-18)neutron                                         \
/// prefix: "stars", from_hub: "channel-18", to_hub: "channel-191"                      \
///                                                                                     \
/// long transmission example:                                                          \
/// oraichain(channel-15) - (channel-301)cosmos_hub(channel-569) - (channel-1)neutron   \
/// prefix: "orai", from_hub: "channel-301", to_hub: "channel-15"                       \
/// prefix: "cosmos", from_hub: "channel-1", to_hub: "channel-569"
#[cw_serde]
pub struct Channel {
    pub prefix: String,
    pub from_hub: String,
    pub to_hub: String,
}

impl Channel {
    pub fn new(prefix: &str, from_hub: &str, to_hub: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            from_hub: from_hub.to_string(),
            to_hub: to_hub.to_string(),
        }
    }
}

// https://github.com/neutron-org/neutron-sdk/blob/main/packages/neutron-sdk/src/sudo/msg.rs
#[cw_serde]
pub struct RequestPacket {
    pub sequence: Option<u64>,
    pub source_port: Option<String>,
    pub source_channel: Option<String>,
    pub destination_port: Option<String>,
    pub destination_channel: Option<String>,
    pub data: Option<Binary>,
    pub timeout_height: Option<RequestPacketTimeoutHeight>,
    pub timeout_timestamp: Option<u64>,
}

#[cw_serde]
pub struct RequestPacketTimeoutHeight {
    pub revision_number: Option<u64>,
    pub revision_height: Option<u64>,
}

/// Height is used for sudo call for `TxQueryResult` enum variant type
#[cw_serde]
pub struct Height {
    /// the revision that the client is currently on
    #[serde(default)]
    pub revision_number: u64,
    /// **height** is a height of remote chain
    #[serde(default)]
    pub revision_height: u64,
}
