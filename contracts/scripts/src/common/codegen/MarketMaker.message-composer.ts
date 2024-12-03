/**
* This file was automatically generated by @cosmwasm/ts-codegen@1.9.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { Coin } from "@cosmjs/amino";
import { MsgExecuteContractEncodeObject } from "@cosmjs/cosmwasm-stargate";
import { MsgExecuteContract } from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { toUtf8 } from "@cosmjs/encoding";
import { TokenUnverified, InstantiateMsg, ExecuteMsg, Uint128, Decimal, BidType, Addr, BiddedCollateralItem, TokenItem, QueryMsg, MigrateMsg, ArrayOfCollectionInfoForAddr, CollectionInfoForAddr, ArrayOfCollateralListResponseItem, CollateralListResponseItem, CollectionOwnerForAddr, ArrayOfCollectionOwnerForAddr, Token, Config, LiquidityInfo, ArrayOfLiquidityInfo, OffersResponse, ArrayOfOffersResponse } from "./MarketMaker.types";
export interface MarketMakerMsg {
  contractAddress: string;
  sender: string;
  acceptAdminRole: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
  updateConfig: ({
    admin,
    controller,
    lendingPlatform,
    oracle,
    token,
    worker
  }: {
    admin?: string;
    controller?: string[];
    lendingPlatform?: string;
    oracle?: string;
    token?: TokenUnverified;
    worker?: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  removeOffers: ({
    collection,
    recipient
  }: {
    collection: string;
    recipient?: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  pause: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
  unpause: (_funds?: Coin[]) => MsgExecuteContractEncodeObject;
  setCollection: ({
    collection,
    owner
  }: {
    collection: string;
    owner: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  removeCollections: ({
    collectionList
  }: {
    collectionList: string[];
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  depositLiquidity: ({
    collection
  }: {
    collection: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  withdrawUndistributedLiquidity: ({
    collection
  }: {
    collection: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  withdrawCollateral: ({
    collection
  }: {
    collection: string;
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  updateOffers: ({
    collection,
    fromToPriceList
  }: {
    collection: string;
    fromToPriceList: Uint128[][];
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
  acceptOffers: ({
    biddedCollateralItemList
  }: {
    biddedCollateralItemList: BiddedCollateralItem[];
  }, _funds?: Coin[]) => MsgExecuteContractEncodeObject;
}
export class MarketMakerMsgComposer implements MarketMakerMsg {
  sender: string;
  contractAddress: string;
  constructor(sender: string, contractAddress: string) {
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.acceptAdminRole = this.acceptAdminRole.bind(this);
    this.updateConfig = this.updateConfig.bind(this);
    this.removeOffers = this.removeOffers.bind(this);
    this.pause = this.pause.bind(this);
    this.unpause = this.unpause.bind(this);
    this.setCollection = this.setCollection.bind(this);
    this.removeCollections = this.removeCollections.bind(this);
    this.depositLiquidity = this.depositLiquidity.bind(this);
    this.withdrawUndistributedLiquidity = this.withdrawUndistributedLiquidity.bind(this);
    this.withdrawCollateral = this.withdrawCollateral.bind(this);
    this.updateOffers = this.updateOffers.bind(this);
    this.acceptOffers = this.acceptOffers.bind(this);
  }
  acceptAdminRole = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          accept_admin_role: {}
        })),
        funds: _funds
      })
    };
  };
  updateConfig = ({
    admin,
    controller,
    lendingPlatform,
    oracle,
    token,
    worker
  }: {
    admin?: string;
    controller?: string[];
    lendingPlatform?: string;
    oracle?: string;
    token?: TokenUnverified;
    worker?: string;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          update_config: {
            admin,
            controller,
            lending_platform: lendingPlatform,
            oracle,
            token,
            worker
          }
        })),
        funds: _funds
      })
    };
  };
  removeOffers = ({
    collection,
    recipient
  }: {
    collection: string;
    recipient?: string;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          remove_offers: {
            collection,
            recipient
          }
        })),
        funds: _funds
      })
    };
  };
  pause = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          pause: {}
        })),
        funds: _funds
      })
    };
  };
  unpause = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          unpause: {}
        })),
        funds: _funds
      })
    };
  };
  setCollection = ({
    collection,
    owner
  }: {
    collection: string;
    owner: string;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          set_collection: {
            collection,
            owner
          }
        })),
        funds: _funds
      })
    };
  };
  removeCollections = ({
    collectionList
  }: {
    collectionList: string[];
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          remove_collections: {
            collection_list: collectionList
          }
        })),
        funds: _funds
      })
    };
  };
  depositLiquidity = ({
    collection
  }: {
    collection: string;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          deposit_liquidity: {
            collection
          }
        })),
        funds: _funds
      })
    };
  };
  withdrawUndistributedLiquidity = ({
    collection
  }: {
    collection: string;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          withdraw_undistributed_liquidity: {
            collection
          }
        })),
        funds: _funds
      })
    };
  };
  withdrawCollateral = ({
    collection
  }: {
    collection: string;
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          withdraw_collateral: {
            collection
          }
        })),
        funds: _funds
      })
    };
  };
  updateOffers = ({
    collection,
    fromToPriceList
  }: {
    collection: string;
    fromToPriceList: Uint128[][];
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          update_offers: {
            collection,
            from_to_price_list: fromToPriceList
          }
        })),
        funds: _funds
      })
    };
  };
  acceptOffers = ({
    biddedCollateralItemList
  }: {
    biddedCollateralItemList: BiddedCollateralItem[];
  }, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(JSON.stringify({
          accept_offers: {
            bidded_collateral_item_list: biddedCollateralItemList
          }
        })),
        funds: _funds
      })
    };
  };
}