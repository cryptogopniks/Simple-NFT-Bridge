import { ChainConfig } from "../../common/interfaces";
import { $, toJson } from "./config-utils";
import * as NftMinterTypes from "../codegen/NftMinter.types";
import * as TransceiverTypes from "../codegen/Transceiver.types";
import * as WrapperTypes from "../codegen/Wrapper.types";

export type NetworkName = "STARGAZE" | "NEUTRON";

export type Wasm = "nft_minter.wasm" | "transceiver.wasm" | "wrapper.wasm";

export type Label =
  | "nft_minter"
  | "transceiver_hub"
  | "transceiver_outpost"
  | "wrapper";

export const ADDRESS = {
  MAINNET: {
    NEUTRON: {
      GOPLEND_SUB_DAO:
        "neutron1dgh7svqfpdckn20280qeuuvx7fyf25g87gsv34hwmec80v0x77rsezd6m5",
      ADMIN: "neutron1f37v0rdvrred27tlqqcpkrqpzfv6ddr2dxqan2",
      WORKER: "neutron1hvp3q00ypzrurd46h7c7c3hu86tx9uf8qt2q28",
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
        // TODO: NEUTRON test
        {
          TYPE: "test",
          DENOM: "untrn",
          CHAIN_ID: "pion-1",
          RPC_LIST: ["https://rpc-falcron.pion-1.ntrn.tech:443"],
          GAS_PRICE_AMOUNT: 0.0053,
          STORE_CODE_GAS_MULTIPLIER: 21.5,
          CONTRACTS: [
            {
              WASM: "nft_minter.wasm",
              LABEL: "nft_minter",
              PERMISSION: [ADDRESS.TESTNET.NEUTRON.ADMIN],
              INIT_MSG: toJson<NftMinterTypes.InstantiateMsg>({
                cw721_code_id: 8345,
                transceiver_hub: $(
                  "OPTIONS[CHAIN_ID=pion-1]|CONTRACTS[LABEL=transceiver_hub]|ADDRESS"
                ),
              }),
              MIGRATE_MSG: toJson<NftMinterTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson({}),
              CODE: 0,
              ADDRESS: "",
            },

            {
              WASM: "transceiver.wasm",
              LABEL: "transceiver_hub",
              PERMISSION: [ADDRESS.TESTNET.NEUTRON.ADMIN],
              INIT_MSG: toJson<TransceiverTypes.InstantiateMsg>({
                transceiver_type: "hub",
              }),
              MIGRATE_MSG: toJson<TransceiverTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson<TransceiverTypes.ExecuteMsg>({
                update_config: {
                  nft_minter: $(
                    "OPTIONS[CHAIN_ID=pion-1]|CONTRACTS[LABEL=nft_minter]|ADDRESS"
                  ),
                },
              }),
              CODE: 8431,
              ADDRESS: "",
            },
          ],
          IBC: [],
        },

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
              PERMISSION: [
                ADDRESS.MAINNET.NEUTRON.GOPLEND_SUB_DAO,
                ADDRESS.MAINNET.NEUTRON.ADMIN,
              ],
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
              CODE: 3080, // 2601,
              // neutron1shmhnuwz2fuq00njdnr5hc6wt5j2gq2sap3nwcx2jt0gqnk8fk9q82l23j
              ADDRESS:
                "neutron1004c3ay7vr3pqzgxgmwfa8rl0pyx8ka5gfgxcdmqnyqyt9dgh2js9tpdpn",
            },

            {
              WASM: "transceiver.wasm",
              LABEL: "transceiver_hub",
              PERMISSION: [
                ADDRESS.MAINNET.NEUTRON.GOPLEND_SUB_DAO,
                ADDRESS.MAINNET.NEUTRON.ADMIN,
              ],
              INIT_MSG: toJson<TransceiverTypes.InstantiateMsg>({
                transceiver_type: "hub",
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
              CODE: 2732, // 2640,
              // neutron1a2qvkpwmrkyh6klfqvhmj0l5m9ematw9smyhk524z8hkunr7d9ns2ulznk
              ADDRESS:
                "neutron1qe7mlmud48ucz4zkcg62uzrw32qrrv72r9ky3mt664lka6d0qdvsrk7zn2",
            },

            {
              WASM: "wrapper.wasm",
              LABEL: "wrapper",
              PERMISSION: [
                ADDRESS.MAINNET.NEUTRON.GOPLEND_SUB_DAO,
                ADDRESS.MAINNET.NEUTRON.ADMIN,
              ],
              INIT_MSG: toJson<WrapperTypes.InstantiateMsg>({
                nft_minter: $(
                  "OPTIONS[CHAIN_ID=neutron-1]|CONTRACTS[LABEL=nft_minter]|ADDRESS"
                ),
                lending_platform:
                  "neutron1dta5fnv70ukvu7g95xqr3eeewc00ztcacw5rpew5hl380crzm9gqmx442u",
                worker:
                  "neutron16nmp4vgaj0tp4fv2eqts3aa8cy67zrp90lmqrcenxla2wmsc2uuqpqd4ht",
              }),
              MIGRATE_MSG: toJson<WrapperTypes.MigrateMsg>({
                version: "1.0.0",
              }),
              UPDATE_MSG: toJson({}),
              CODE: 3078,
              ADDRESS:
                "neutron14kk7zxt043vgm9gczaam6srppx6a52pz4p733jhc3ny7jcmp2s3sc7yh3y",
            },
          ],
          IBC: [],
        },
      ],
    },

    {
      NAME: "stargaze",
      PREFIX: "stars",
      OPTIONS: [
        // TODO: STARGAZE main
        {
          TYPE: "main",
          DENOM: "ustars",
          CHAIN_ID: "stargaze-1",
          RPC_LIST: ["https://rpc.stargaze-apis.com:443"],
          GAS_PRICE_AMOUNT: 1.1,
          STORE_CODE_GAS_MULTIPLIER: 21.5,
          CONTRACTS: [
            {
              WASM: "transceiver.wasm",
              LABEL: "transceiver_outpost",
              PERMISSION: [ADDRESS.MAINNET.STARGAZE.ADMIN],
              INIT_MSG: toJson<TransceiverTypes.InstantiateMsg>({
                transceiver_type: "outpost",
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
              CODE: 475, // 472,
              // stars1adqv99crwz7vswysw4yhnf5apw5svmq2femc2j85dys9uxfw8czs5ld2hs
              ADDRESS:
                "stars19al6vqmae0lljvqe46r8wtd3fqzj7gmhz6dkas9emrapgukyyzaq5q9rq4",
            },
          ],
          IBC: [],
        },
      ],
    },

    // {
    //   NAME: "stargaze",
    //   PREFIX: "stars",
    //   OPTIONS: [
    //     {
    //       TYPE: "local",
    //       DENOM: "ustars",
    //       CHAIN_ID: "stargaze-0",
    //       RPC_LIST: [],
    //       GAS_PRICE_AMOUNT: 0.04,
    //       STORE_CODE_GAS_MULTIPLIER: 25,
    //       CONTRACTS: [
    //         {
    //           WASM: "adapter_dex_stargaze.wasm",
    //           LABEL: "adapter_dex",
    //           INIT_MSG: toJson<AdapterDexStargazeTypes.InstantiateMsg>({
    //             worker: "stars19y7a38cnf9d8cr264wz5d6dmrsgsmplxkf4lyw",
    //             adapter_marketplace:
    //               "stars19y7a38cnf9d8cr264wz5d6dmrsgsmplxkf4lyw",
    //             recover_address: "stars19y7a38cnf9d8cr264wz5d6dmrsgsmplxkf4lyw",
    //           }),
    //           MIGRATE_MSG: toJson<AdapterDexStargazeTypes.MigrateMsg>({
    //             version: "1.0.0",
    //           }),
    //           UPDATE_MSG: toJson<AdapterDexStargazeTypes.ExecuteMsg>({
    //             update_config: {
    //               lending_platform_stable:
    //                 "stars19y7a38cnf9d8cr264wz5d6dmrsgsmplxkf4lyw",
    //             },
    //           }),
    //           CODE: 0,
    //           ADDRESS: "",
    //         },
    //       ],
    //       IBC: [],
    //     },
    //   ],
    // },

    // {
    //   NAME: "terra",
    //   PREFIX: "terra",
    //   OPTIONS: [
    //     {
    //       TYPE: "local",
    //       DENOM: "uluna",
    //       CHAIN_ID: "terra-0",
    //       RPC_LIST: [],
    //       GAS_PRICE_AMOUNT: 0.04,
    //       STORE_CODE_GAS_MULTIPLIER: 25,
    //       CONTRACTS: [
    //         {
    //           WASM: "adapter_dex_stargaze.wasm",
    //           LABEL: "adapter_dex",
    //           INIT_MSG: toJson<AdapterDexStargazeTypes.InstantiateMsg>({
    //             worker: "terra19y7a38cnf9d8cr264wz5d6dmrsgsmplxy3czdl",
    //             adapter_marketplace:
    //               "terra19y7a38cnf9d8cr264wz5d6dmrsgsmplxy3czdl",
    //             recover_address: "terra19y7a38cnf9d8cr264wz5d6dmrsgsmplxy3czdl",
    //           }),
    //           MIGRATE_MSG: toJson<AdapterDexStargazeTypes.MigrateMsg>({
    //             version: "1.0.0",
    //           }),
    //           UPDATE_MSG: toJson<AdapterDexStargazeTypes.ExecuteMsg>({
    //             update_config: {
    //               lending_platform_stable:
    //                 "terra19y7a38cnf9d8cr264wz5d6dmrsgsmplxy3czdl",
    //             },
    //           }),
    //           CODE: 0,
    //           ADDRESS: "",
    //         },
    //       ],
    //       IBC: [],
    //     },
    //   ],
    // },

    // {
    //   NAME: "neutron",
    //   PREFIX: "neutron",
    //   OPTIONS: [
    //     {
    //       TYPE: "main",
    //       DENOM: "untrn",
    //       CHAIN_ID: "neutron-1",
    //       RPC_LIST: ["https://rpc-neutron.cosmos-spaces.cloud:443"],
    //       GAS_PRICE_AMOUNT: 0.99,
    //       STORE_CODE_GAS_MULTIPLIER: 20,
    //       CONTRACTS: [
    //         {
    //           WASM: "adapter_dex_stargaze.wasm",
    //           LABEL: "adapter_dex",
    //           INIT_MSG: toJson<AdapterDexStargazeTypes.InstantiateMsg>({
    //             worker: ADDRESS.NEUTRON.WORKER,
    //             adapter_marketplace: ADDRESS.NEUTRON.ADMIN,
    //             recover_address: ADDRESS.NEUTRON.WORKER,
    //           }),
    //           MIGRATE_MSG: toJson<AdapterDexStargazeTypes.MigrateMsg>({
    //             version: "1.0.0",
    //           }),
    //           UPDATE_MSG: toJson<AdapterDexStargazeTypes.ExecuteMsg>({
    //             update_config: {
    //               lending_platform_stable: $(
    //                 "OPTIONS[CHAIN_ID=neutron-1]|CONTRACTS[LABEL=oracle]|ADDRESS"
    //               ),
    //             },
    //           }),
    //           CODE: 1474,
    //           ADDRESS:
    //             "neutron1r7t9k558xrgvzjrl05m22fvqwk74rgs6y4dkm5ls73z55amxrknq9jh3xt",
    //         },

    //         {
    //           WASM: "oracle.wasm",
    //           LABEL: "oracle",
    //           INIT_MSG: toJson<OracleTypes.InstantiateMsg>({
    //             worker: ADDRESS.NEUTRON.WORKER,
    //           }),
    //           MIGRATE_MSG: toJson<OracleTypes.MigrateMsg>({
    //             version: "1.0.0",
    //           }),
    //           UPDATE_MSG: toJson<OracleTypes.ExecuteMsg>({
    //             update_config: {
    //               controller: [ADDRESS.NEUTRON.ADMIN],
    //             },
    //           }),
    //           CODE: 1472,
    //           ADDRESS:
    //             "neutron1wtf3j50a32hvwdvjdxu97fz8zgxsp3ay324u75r5yecwj5jzhsass9aqkv",
    //         },
    //       ],
    //       IBC: [],
    //     },
    //   ],
    // },
  ],
};
