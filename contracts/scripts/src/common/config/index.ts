import { ChainConfig } from "../../common/interfaces";
import { $, toJson } from "./config-utils";
import * as NftMinterTypes from "../codegen/NftMinter.types";
import * as TransceiverTypes from "../codegen/Transceiver.types";

export type NetworkName =
  | "STARGAZE"
  | "NEUTRON"
  | "SECRET_NETWORK"
  | "ORAICHAIN";

export type Wasm = "nft_minter.wasm" | "transceiver.wasm";

export type Label = "nft_minter" | "transceiver_hub" | "transceiver_outpost";

export const ADDRESS = {
  MAINNET: {
    NEUTRON: {
      ADMIN: "neutron1f37v0rdvrred27tlqqcpkrqpzfv6ddr2dxqan2",
      WORKER: "neutron1hvp3q00ypzrurd46h7c7c3hu86tx9uf8qt2q28",
    },
    SECRET: {
      ADMIN: "secret1f37v0rdvrred27tlqqcpkrqpzfv6ddr2tuak53",
      WORKER: "secret1gjqnuhv52pd2a7ets2vhw9w9qa9knyhy56zjrg",
    },
    ORAI: {
      ADMIN: "orai1f37v0rdvrred27tlqqcpkrqpzfv6ddr262lug7",
      WORKER: "orai1gjqnuhv52pd2a7ets2vhw9w9qa9knyhy9vqcl8",
    },
    STARGAZE: {
      ADMIN: "stars1f37v0rdvrred27tlqqcpkrqpzfv6ddr2a97zzu",
      WORKER: "stars1hvp3q00ypzrurd46h7c7c3hu86tx9uf8sg5lm3",
    },
  },
  TESTNET: {
    NEUTRON: {
      ADMIN: "neutron1f37v0rdvrred27tlqqcpkrqpzfv6ddr2dxqan2",
      WORKER: "neutron1hvp3q00ypzrurd46h7c7c3hu86tx9uf8qt2q28",
    },
    STARGAZE: {
      ADMIN: "stars1f37v0rdvrred27tlqqcpkrqpzfv6ddr2a97zzu",
      WORKER: "stars1hvp3q00ypzrurd46h7c7c3hu86tx9uf8sg5lm3",
    },
  },
};

export const TOKEN = {
  NEUTRON: {
    MAINNET: {
      NTRN: "untrn",
    },
    TESTNET: {
      NTRN: "untrn",
    },
  },
  STARGAZE: {
    MAINNET: {
      STARS: "ustars",
    },
    TESTNET: {
      STARS: "ustars",
    },
  },
};

/**
 * This config is used to generate `config.json` used by any script (ts, js, bash).
 * It must be filled manually. If any contract must be added it's required to include
 * it with default parameters - code is 0, address is "".
 * This config uses logs.json generated by local-interchaintest to update endpoints
 * in cofig.json.
 */
export const CHAIN_CONFIG: ChainConfig = {
  CHAINS: [
    {
      NAME: "neutron",
      PREFIX: "neutron",
      OPTIONS: [
        // TODO: NEUTRON main
        {
          TYPE: "main",
          DENOM: "untrn",
          CHAIN_ID: "neutron-1",
          RPC_LIST: [
            "https://rpc.neutron.quokkastake.io:443",
            "https://rpc-neutron.cosmos-spaces.cloud:443",
          ],
          GAS_PRICE_AMOUNT: 0.99,
          STORE_CODE_GAS_MULTIPLIER: 21.5,
          CONTRACTS: [
            {
              WASM: "nft_minter.wasm",
              LABEL: "nft_minter",
              PERMISSION: [ADDRESS.MAINNET.NEUTRON.ADMIN],
              INIT_MSG: toJson<NftMinterTypes.InstantiateMsg>({
                cw721_code_id: 2554,
                transceiver_hub: $(
                  "OPTIONS[CHAIN_ID=neutron-1]|CONTRACTS[LABEL=transceiver_hub]|ADDRESS"
                ),
              }),
              MIGRATE_MSG: toJson<NftMinterTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson({}),
              CODE: 2731,
              ADDRESS: "",
            },

            {
              WASM: "transceiver.wasm",
              LABEL: "transceiver_hub",
              PERMISSION: [ADDRESS.MAINNET.NEUTRON.ADMIN],
              INIT_MSG: toJson<TransceiverTypes.InstantiateMsg>({
                transceiver_type: "hub",
                is_retranslation_outpost: false,
              }),
              MIGRATE_MSG: toJson<TransceiverTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson<TransceiverTypes.ExecuteMsg>({
                update_config: {
                  nft_minter: $(
                    "OPTIONS[CHAIN_ID=neutron-1]|CONTRACTS[LABEL=nft_minter]|ADDRESS"
                  ),
                },
              }),
              CODE: 0,
              ADDRESS: "",
            },
          ],
          IBC: [],
        },
      ],
    },

    {
      NAME: "secretnetwork",
      PREFIX: "secret",
      OPTIONS: [
        // TODO: SECRET_NETWORK main
        {
          TYPE: "main",
          DENOM: "uscrt",
          CHAIN_ID: "secret-4",
          RPC_LIST: ["https://scrt.public-rpc.com:443"],
          GAS_PRICE_AMOUNT: 0.1,
          STORE_CODE_GAS_MULTIPLIER: 20,
          CONTRACTS: [
            {
              WASM: "transceiver.wasm",
              LABEL: "transceiver_outpost",
              PERMISSION: [ADDRESS.MAINNET.SECRET.ADMIN],
              INIT_MSG: toJson<TransceiverTypes.InstantiateMsg>({
                transceiver_type: "outpost",
                is_retranslation_outpost: true,
              }),
              MIGRATE_MSG: toJson<TransceiverTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson<TransceiverTypes.ExecuteMsg>({
                update_config: {
                  hub_address: $(
                    "OPTIONS[CHAIN_ID=neutron-1]|CONTRACTS[LABEL=transceiver_hub]|ADDRESS"
                  ),
                },
              }),
              CODE: 0,
              ADDRESS: "",
            },
          ],
          IBC: [],
        },
      ],
    },

    {
      NAME: "oraichain",
      PREFIX: "orai",
      OPTIONS: [
        // TODO: ORAICHAIN main
        {
          TYPE: "main",
          DENOM: "orai",
          CHAIN_ID: "Oraichain",
          RPC_LIST: ["https://rpc.orai.io:443"],
          GAS_PRICE_AMOUNT: 0.005,
          STORE_CODE_GAS_MULTIPLIER: 20,
          CONTRACTS: [
            {
              WASM: "transceiver.wasm",
              LABEL: "transceiver_outpost",
              PERMISSION: [ADDRESS.MAINNET.ORAI.ADMIN],
              INIT_MSG: toJson<TransceiverTypes.InstantiateMsg>({
                transceiver_type: "outpost",
                is_retranslation_outpost: false,
              }),
              MIGRATE_MSG: toJson<TransceiverTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson<TransceiverTypes.ExecuteMsg>({
                update_config: {
                  hub_address: $(
                    "OPTIONS[CHAIN_ID=neutron-1]|CONTRACTS[LABEL=transceiver_hub]|ADDRESS"
                  ),
                },
              }),
              CODE: 0,
              ADDRESS: "",
            },
          ],
          IBC: [],
        },
      ],
    },
  ],
};
