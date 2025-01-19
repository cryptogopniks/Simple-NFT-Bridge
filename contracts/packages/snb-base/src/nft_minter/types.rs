use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct ConfigPre {
    pub admin: Addr,
    pub transceiver_hub: Addr,
    pub cw721_code_id: u64,
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub transceiver_hub: Addr,
    pub wrapper: Option<Addr>,
    pub cw721_code_id: u64,
}

#[cw_serde]
pub struct TransferAdminState {
    pub new_admin: Addr,
    pub deadline: u64,
}
