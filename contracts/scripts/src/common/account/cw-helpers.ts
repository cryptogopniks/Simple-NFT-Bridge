import { NftMinterMsgComposer } from "../codegen/NftMinter.message-composer";
import { NftMinterQueryClient } from "../codegen/NftMinter.client";

import { TransceiverMsgComposer } from "../codegen/Transceiver.message-composer";
import { TransceiverQueryClient } from "../codegen/Transceiver.client";

import { WrapperMsgComposer } from "../codegen/Wrapper.message-composer";
import { WrapperQueryClient } from "../codegen/Wrapper.client";

import CONFIG_JSON from "../config/config.json";
import { getLast, l, logAndReturn } from "../utils";
import { toBase64, fromUtf8, toUtf8 } from "@cosmjs/encoding";
import {
  MsgMigrateContract,
  MsgUpdateAdmin,
} from "cosmjs-types/cosmwasm/wasm/v1/tx";
import { getChainOptionById, getContractByLabel } from "../config/config-utils";
import { Timestamp } from "../codegen/Transceiver.types";
import {
  getCwClient,
  signAndBroadcastWrapper,
  getExecuteContractMsg,
} from "./clients";
import {
  SigningCosmWasmClient,
  CosmWasmClient,
  MsgExecuteContractEncodeObject,
  MsgUpdateAdminEncodeObject,
  MsgMigrateContractEncodeObject,
} from "@cosmjs/cosmwasm-stargate";
import {
  DirectSecp256k1HdWallet,
  OfflineSigner,
  OfflineDirectSigner,
  coin,
} from "@cosmjs/proto-signing";
import {
  Cw20SendMsg,
  TokenUnverified,
  ChainConfig,
  ContractInfo,
  QueryAllOperatorsResponse,
  QueryAllOperatorsMsg,
  ApproveAllMsg,
  RevokeAllMsg,
  QueryApprovalsMsg,
  ApprovalsResponse,
  QueryTokens,
  TokensResponse,
  QueryOwnerOf,
  OwnerOfResponse,
} from "../interfaces";

function addSingleTokenToComposerObj(
  obj: MsgExecuteContractEncodeObject,
  amount: number,
  token: TokenUnverified
): MsgExecuteContractEncodeObject {
  const {
    value: { contract, sender, msg },
  } = obj;

  if (!(contract && sender && msg)) {
    throw new Error(`${msg} parameters error!`);
  }

  return getSingleTokenExecMsg(
    contract,
    sender,
    JSON.parse(fromUtf8(msg)),
    amount,
    token
  );
}

function getSingleTokenExecMsg(
  contractAddress: string,
  senderAddress: string,
  msg: any,
  amount?: number,
  token?: TokenUnverified
) {
  // get msg without funds
  if (!(token && amount)) {
    return getExecuteContractMsg(contractAddress, senderAddress, msg, []);
  }

  // get msg with native token
  if ("native" in token) {
    return getExecuteContractMsg(contractAddress, senderAddress, msg, [
      coin(amount, token.native.denom),
    ]);
  }

  // get msg with CW20 token
  const cw20SendMsg: Cw20SendMsg = {
    send: {
      contract: contractAddress,
      amount: `${amount}`,
      msg: toBase64(msg),
    },
  };

  return getExecuteContractMsg(
    token.cw20.address,
    senderAddress,
    cw20SendMsg,
    []
  );
}

async function isCollectionApproved(
  signingClient: SigningCosmWasmClient,
  owner: string,
  target: string,
  collection: string
) {
  const queryAllOperatorsMsg: QueryAllOperatorsMsg = {
    all_operators: {
      owner,
    },
  };

  const { operators }: QueryAllOperatorsResponse =
    await signingClient.queryContractSmart(collection, queryAllOperatorsMsg);

  return operators.find((x) => x.spender === target);
}

function getApproveCollectionMsg(
  collectionAddress: string,
  senderAddress: string,
  operator: string
): MsgExecuteContractEncodeObject {
  const approveAllMsg: ApproveAllMsg = {
    approve_all: {
      operator,
    },
  };

  return getSingleTokenExecMsg(collectionAddress, senderAddress, approveAllMsg);
}

function getRevokeCollectionMsg(
  collectionAddress: string,
  senderAddress: string,
  operator: string
): MsgExecuteContractEncodeObject {
  const revokeAllMsg: RevokeAllMsg = {
    revoke_all: {
      operator,
    },
  };

  return getSingleTokenExecMsg(collectionAddress, senderAddress, revokeAllMsg);
}

function getContracts(contracts: ContractInfo[]) {
  let NFT_MINTER_CONTRACT: ContractInfo | undefined;
  let TRANSCEIVER_CONTRACT: ContractInfo | undefined;
  let WRAPPER_CONTRACT: ContractInfo | undefined;

  try {
    NFT_MINTER_CONTRACT = getContractByLabel(contracts, "nft_minter");
  } catch (error) {
    l(error);
  }

  try {
    TRANSCEIVER_CONTRACT = NFT_MINTER_CONTRACT
      ? getContractByLabel(contracts, "transceiver_hub")
      : getContractByLabel(contracts, "transceiver_outpost");
  } catch (error) {
    l(error);
  }

  try {
    WRAPPER_CONTRACT = getContractByLabel(contracts, "wrapper");
  } catch (error) {
    l(error);
  }

  return {
    NFT_MINTER_CONTRACT,
    TRANSCEIVER_CONTRACT,
    WRAPPER_CONTRACT,
  };
}

async function getCwExecHelpers(
  chainId: string,
  rpc: string,
  owner: string,
  signer: (OfflineSigner & OfflineDirectSigner) | DirectSecp256k1HdWallet
) {
  const CHAIN_CONFIG = CONFIG_JSON as ChainConfig;
  const {
    OPTION: { CONTRACTS },
  } = getChainOptionById(CHAIN_CONFIG, chainId);

  const { NFT_MINTER_CONTRACT, TRANSCEIVER_CONTRACT, WRAPPER_CONTRACT } =
    getContracts(CONTRACTS);

  const cwClient = await getCwClient(rpc, owner, signer);
  if (!cwClient) throw new Error("cwClient is not found!");

  const signingClient = cwClient.client as SigningCosmWasmClient;
  const _signAndBroadcast = signAndBroadcastWrapper(signingClient, owner);

  const nftMinterMsgComposer = new NftMinterMsgComposer(
    owner,
    NFT_MINTER_CONTRACT?.ADDRESS || ""
  );

  const transceiverMsgComposer = new TransceiverMsgComposer(
    owner,
    TRANSCEIVER_CONTRACT?.ADDRESS || ""
  );

  const wrapperMsgComposer = new WrapperMsgComposer(
    owner,
    WRAPPER_CONTRACT?.ADDRESS || ""
  );

  async function _msgWrapperWithGasPrice(
    msgs: MsgExecuteContractEncodeObject[],
    gasPrice: string,
    gasAdjustment: number = 1,
    memo?: string
  ) {
    const tx = await _signAndBroadcast(msgs, gasPrice, gasAdjustment, memo);
    l("\n", tx, "\n");
    return tx;
  }

  // utils

  async function cwTransferAdmin(
    contract: string,
    newAdmin: string,
    gasPrice: string,
    gasAdjustment: number = 1
  ) {
    const msg: MsgUpdateAdminEncodeObject = {
      typeUrl: "/cosmwasm.wasm.v1.MsgUpdateAdmin",
      value: MsgUpdateAdmin.fromPartial({
        sender: owner,
        contract,
        newAdmin,
      }),
    };

    const tx = await _signAndBroadcast([msg], gasPrice, gasAdjustment);
    l("\n", tx, "\n");
    return tx;
  }

  async function cwMigrateMultipleContracts(
    contractList: string[],
    codeId: number,
    migrateMsg: any,
    gasPrice: string,
    gasAdjustment: number = 1
  ) {
    const msgList: MsgMigrateContractEncodeObject[] = contractList.map(
      (contract) => ({
        typeUrl: "/cosmwasm.wasm.v1.MsgMigrateContract",
        value: MsgMigrateContract.fromPartial({
          sender: owner,
          contract,
          codeId: BigInt(codeId),
          msg: toUtf8(JSON.stringify(migrateMsg)),
        }),
      })
    );

    const tx = await _signAndBroadcast(msgList, gasPrice, gasAdjustment);
    l("\n", tx, "\n");
    return tx;
  }

  async function cwRevoke(
    collectionAddress: string,
    senderAddress: string,
    operator: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [getRevokeCollectionMsg(collectionAddress, senderAddress, operator)],
      gasPrice
    );
  }

  async function cwMintNft(
    collectionAddress: string,
    recipient: string,
    tokenIdList: number[],
    gasPrice: string
  ) {
    const msgList = tokenIdList.map((tokenId) => {
      const mintMsg = {
        mint: {
          owner: recipient,
          token_id: tokenId.toString(),
        },
      };

      return getSingleTokenExecMsg(collectionAddress, owner, mintMsg);
    });

    return await _msgWrapperWithGasPrice(msgList, gasPrice);
  }

  // nft-minter

  async function cwNftMinterAcceptAdminRole(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [nftMinterMsgComposer.acceptAdminRole()],
      gasPrice
    );
  }

  async function cwNftMinterUpdateConfig(admin: string, gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [nftMinterMsgComposer.updateConfig({ admin })],
      gasPrice
    );
  }

  async function cwNftMinterCreateCollection(name: string, gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [nftMinterMsgComposer.createCollection({ name })],
      gasPrice
    );
  }

  async function cwNftMinterMint(
    collection: string,
    tokenList: string[],
    recipient: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [nftMinterMsgComposer.mint({ collection, tokenList, recipient })],
      gasPrice
    );
  }

  async function cwNftMinterApproveAndBurn(
    collection: string,
    tokenList: string[],
    gasPrice: string
  ) {
    const targetOperator = NFT_MINTER_CONTRACT?.ADDRESS || "";
    const isCollectionApprovedFlag = await isCollectionApproved(
      signingClient,
      owner,
      targetOperator,
      collection
    );

    let msgList = !isCollectionApprovedFlag
      ? [getApproveCollectionMsg(collection, owner, targetOperator)]
      : [];
    msgList.push(nftMinterMsgComposer.burn({ collection, tokenList }));

    return await _msgWrapperWithGasPrice(msgList, gasPrice);
  }

  // transceiver

  async function cwTransceiverAcceptAdminRole(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [transceiverMsgComposer.acceptAdminRole()],
      gasPrice
    );
  }

  async function cwTransceiverPause(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [transceiverMsgComposer.pause()],
      gasPrice
    );
  }

  async function cwTransceiverUnpause(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [transceiverMsgComposer.unpause()],
      gasPrice
    );
  }

  async function cwTransceiverUpdateConfig(
    {
      admin,
      nftMinter,
      hubAddress,
      tokenLimit,
      minNtrnIbcFee,
    }: {
      admin?: string;
      nftMinter?: string;
      hubAddress?: string;
      tokenLimit?: number;
      minNtrnIbcFee?: number;
    },
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [
        transceiverMsgComposer.updateConfig({
          admin,
          nftMinter,
          hubAddress,
          tokenLimit,
          minNtrnIbcFee: minNtrnIbcFee ? minNtrnIbcFee.toString() : undefined,
        }),
      ],
      gasPrice
    );
  }

  async function cwTransceiverAddCollection(
    hubCollection: string,
    homeCollection: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [transceiverMsgComposer.addCollection({ hubCollection, homeCollection })],
      gasPrice
    );
  }

  async function cwTransceiverRemoveCollection(
    hubCollection: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [transceiverMsgComposer.removeCollection({ hubCollection })],
      gasPrice
    );
  }

  async function cwTransceiverSetChannel(
    prefix: string,
    fromHub: string,
    toHub: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [
        transceiverMsgComposer.setChannel({
          prefix,
          fromHub,
          toHub,
        }),
      ],
      gasPrice
    );
  }

  async function cwTransceiverApproveAndSend(
    hubCollection: string,
    homeCollection: string,
    tokenList: string[],
    {
      target,
    }: {
      target?: string;
    },
    amount: number,
    denom: string,
    gasPrice: string
  ) {
    const collection = NFT_MINTER_CONTRACT?.ADDRESS
      ? hubCollection
      : homeCollection;
    const targetOperator = TRANSCEIVER_CONTRACT?.ADDRESS || "";
    const isCollectionApprovedFlag = await isCollectionApproved(
      signingClient,
      owner,
      targetOperator,
      collection
    );

    let msgList = !isCollectionApprovedFlag
      ? [getApproveCollectionMsg(collection, owner, targetOperator)]
      : [];
    msgList.push(
      addSingleTokenToComposerObj(
        transceiverMsgComposer.send({
          hubCollection,
          tokenList,
          target,
        }),
        amount,
        {
          native: { denom },
        }
      )
    );

    return await _msgWrapperWithGasPrice(msgList, gasPrice);
  }

  async function cwTransceiverAccept(
    msg: string,
    timestamp: Timestamp,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [
        transceiverMsgComposer.accept({
          msg,
          timestamp,
        }),
      ],
      gasPrice
    );
  }

  // wrapper

  async function cwWrapperAcceptAdminRole(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [wrapperMsgComposer.acceptAdminRole()],
      gasPrice
    );
  }

  async function cwWrapperUpdateConfig(
    {
      admin,
      worker,
    }: {
      admin?: string;
      worker?: string;
    },
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [wrapperMsgComposer.updateConfig({ admin, worker })],
      gasPrice
    );
  }

  async function cwWrapperPause(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [wrapperMsgComposer.pause()],
      gasPrice
    );
  }

  async function cwWrapperUnpause(gasPrice: string) {
    return await _msgWrapperWithGasPrice(
      [wrapperMsgComposer.unpause()],
      gasPrice
    );
  }

  // TODO: merge with lending functions, implement splitting token lists
  async function cwWrapperApproveAndWrap(
    collectionIn: string,
    tokenList: string[],
    gasPrice: string
  ) {
    const targetOperator = WRAPPER_CONTRACT?.ADDRESS || "";
    const isCollectionApprovedFlag = await isCollectionApproved(
      signingClient,
      owner,
      targetOperator,
      collectionIn
    );

    let msgList = !isCollectionApprovedFlag
      ? [getApproveCollectionMsg(collectionIn, owner, targetOperator)]
      : [];
    msgList.push(wrapperMsgComposer.wrap({ collectionIn, tokenList }));

    return await _msgWrapperWithGasPrice(msgList, gasPrice);
  }

  async function cwWrapperApproveAndUnwrap(
    collectionOut: string,
    tokenList: string[],
    gasPrice: string
  ) {
    const targetOperator = WRAPPER_CONTRACT?.ADDRESS || "";
    const isCollectionApprovedFlag = await isCollectionApproved(
      signingClient,
      owner,
      targetOperator,
      collectionOut
    );

    let msgList = !isCollectionApprovedFlag
      ? [getApproveCollectionMsg(collectionOut, owner, targetOperator)]
      : [];
    msgList.push(wrapperMsgComposer.unwrap({ collectionOut, tokenList }));

    return await _msgWrapperWithGasPrice(msgList, gasPrice);
  }

  async function cwWrapperAddCollection(
    collectionIn: string,
    collectionOut: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [wrapperMsgComposer.addCollection({ collectionIn, collectionOut })],
      gasPrice
    );
  }

  async function cwWrapperRemoveCollection(
    collectionIn: string,
    gasPrice: string
  ) {
    return await _msgWrapperWithGasPrice(
      [wrapperMsgComposer.removeCollection({ collectionIn })],
      gasPrice
    );
  }

  return {
    utils: { cwTransferAdmin, cwMigrateMultipleContracts, cwRevoke, cwMintNft },
    nftMinter: {
      cwAcceptAdminRole: cwNftMinterAcceptAdminRole,
      cwUpdateConfig: cwNftMinterUpdateConfig,
      cwCreateCollection: cwNftMinterCreateCollection,
      cwMint: cwNftMinterMint,
      cwApproveAndBurn: cwNftMinterApproveAndBurn,
    },
    transceiver: {
      cwAcceptAdminRole: cwTransceiverAcceptAdminRole,
      cwPause: cwTransceiverPause,
      cwUnpause: cwTransceiverUnpause,
      cwUpdateConfig: cwTransceiverUpdateConfig,
      cwAddCollection: cwTransceiverAddCollection,
      cwRemoveCollection: cwTransceiverRemoveCollection,
      cwSetChannel: cwTransceiverSetChannel,
      cwApproveAndSend: cwTransceiverApproveAndSend,
      cwAccept: cwTransceiverAccept,
    },
    wrapper: {
      cwAcceptAdminRole: cwWrapperAcceptAdminRole,
      cwUpdateConfig: cwWrapperUpdateConfig,
      cwPause: cwWrapperPause,
      cwUnpause: cwWrapperUnpause,
      cwApproveAndWrap: cwWrapperApproveAndWrap,
      cwApproveAndUnwrap: cwWrapperApproveAndUnwrap,
      cwAddCollection: cwWrapperAddCollection,
      cwRemoveCollection: cwWrapperRemoveCollection,
    },
  };
}

async function getCwQueryHelpers(chainId: string, rpc: string) {
  const CHAIN_CONFIG = CONFIG_JSON as ChainConfig;
  const {
    OPTION: { CONTRACTS },
  } = getChainOptionById(CHAIN_CONFIG, chainId);

  const { NFT_MINTER_CONTRACT, TRANSCEIVER_CONTRACT, WRAPPER_CONTRACT } =
    getContracts(CONTRACTS);

  const cwClient = await getCwClient(rpc);
  if (!cwClient) throw new Error("cwClient is not found!");

  const cosmwasmQueryClient: CosmWasmClient = cwClient.client;

  const nftMinterQueryClient = new NftMinterQueryClient(
    cosmwasmQueryClient,
    NFT_MINTER_CONTRACT?.ADDRESS || ""
  );

  const transceiverQueryClient = new TransceiverQueryClient(
    cosmwasmQueryClient,
    TRANSCEIVER_CONTRACT?.ADDRESS || ""
  );

  const wrapperQueryClient = new WrapperQueryClient(
    cosmwasmQueryClient,
    WRAPPER_CONTRACT?.ADDRESS || ""
  );

  // utils

  async function cwQueryOperators(
    ownerAddress: string,
    collectionAddress: string,
    isDisplayed: boolean = false
  ) {
    const queryAllOperatorsMsg: QueryAllOperatorsMsg = {
      all_operators: {
        owner: ownerAddress,
      },
    };
    const res: QueryAllOperatorsResponse =
      await cosmwasmQueryClient.queryContractSmart(
        collectionAddress,
        queryAllOperatorsMsg
      );
    return logAndReturn(res, isDisplayed);
  }

  async function cwQueryApprovals(
    collectionAddress: string,
    tokenId: string,
    isDisplayed: boolean = false
  ) {
    const queryApprovalsMsg: QueryApprovalsMsg = {
      approvals: {
        token_id: tokenId,
      },
    };
    const res: ApprovalsResponse = await cosmwasmQueryClient.queryContractSmart(
      collectionAddress,
      queryApprovalsMsg
    );
    return logAndReturn(res, isDisplayed);
  }

  async function cwQueryBalanceInNft(
    owner: string,
    collectionAddress: string,
    isDisplayed: boolean = false
  ) {
    const MAX_LIMIT = 100;
    const ITER_LIMIT = 50;

    let tokenList: string[] = [];
    let tokenAmountSum: number = 0;
    let i: number = 0;
    let lastToken: string | undefined = undefined;

    while ((!i || tokenAmountSum === MAX_LIMIT) && i < ITER_LIMIT) {
      i++;

      try {
        const queryTokensMsg: QueryTokens = {
          tokens: {
            owner,
            start_after: lastToken,
            limit: MAX_LIMIT,
          },
        };

        const { tokens }: TokensResponse =
          await cosmwasmQueryClient.queryContractSmart(
            collectionAddress,
            queryTokensMsg
          );

        tokenList = [...tokenList, ...tokens];
        tokenAmountSum = tokens.length;
        lastToken = getLast(tokens);
      } catch (error) {
        l(error);
      }
    }

    const res: TokensResponse = { tokens: tokenList };
    return logAndReturn(res, isDisplayed);
  }

  async function cwQueryNftOwner(
    collectionAddress: string,
    tokenId: string,
    isDisplayed: boolean = false
  ) {
    const queryOwnerOfMsg: QueryOwnerOf = {
      owner_of: { token_id: tokenId },
    };
    const res: OwnerOfResponse = await cosmwasmQueryClient.queryContractSmart(
      collectionAddress,
      queryOwnerOfMsg
    );
    return logAndReturn(res, isDisplayed);
  }

  // nft-minter

  async function cwNftMinterQueryConfig(isDisplayed: boolean = false) {
    const res = await nftMinterQueryClient.config();
    return logAndReturn(res, isDisplayed);
  }

  async function cwNftMinterQueryCollection(
    address: string,
    isDisplayed: boolean = false
  ) {
    const res = await nftMinterQueryClient.collection({ address });
    return logAndReturn(res, isDisplayed);
  }

  async function cwNftMinterQueryCollectionList(
    amount: number = 100,
    startAfter: string | undefined = undefined,
    isDisplayed: boolean = false
  ) {
    const res = await nftMinterQueryClient.collectionList({
      amount,
      startAfter,
    });
    return logAndReturn(res, isDisplayed);
  }

  // transceiver

  async function cwTransceiverQueryConfig(isDisplayed: boolean = false) {
    const res = await transceiverQueryClient.config();
    return logAndReturn(res, isDisplayed);
  }

  async function cwTransceiverQueryPauseState(isDisplayed: boolean = false) {
    const res = await transceiverQueryClient.pauseState();
    return logAndReturn(res, isDisplayed);
  }

  async function cwTransceiverQueryOutposts(isDisplayed: boolean = false) {
    const res = await transceiverQueryClient.outposts();
    return logAndReturn(res, isDisplayed);
  }

  async function cwTransceiverQueryCollection(
    {
      hubCollection,
      homeCollection,
    }: {
      hubCollection?: string;
      homeCollection?: string;
    },
    isDisplayed: boolean = false
  ) {
    const res = await transceiverQueryClient.collection({
      hubCollection,
      homeCollection,
    });
    return logAndReturn(res, isDisplayed);
  }

  async function cwTransceiverQueryCollectionList(
    isDisplayed: boolean = false
  ) {
    const res = await transceiverQueryClient.collectionList();
    return logAndReturn(res, isDisplayed);
  }

  async function cwTransceiverQueryChannelList(isDisplayed: boolean = false) {
    const res = await transceiverQueryClient.channelList();
    return logAndReturn(res, isDisplayed);
  }

  // wrapper

  async function cwWrapperQueryConfig(isDisplayed: boolean = false) {
    const res = await wrapperQueryClient.config();
    return logAndReturn(res, isDisplayed);
  }

  async function cwWrapperQueryCollectionList(isDisplayed: boolean = false) {
    const res = await wrapperQueryClient.collectionList();
    return logAndReturn(res, isDisplayed);
  }

  async function cwWrapperQueryCollection(
    collectionIn: string,
    isDisplayed: boolean = false
  ) {
    const res = await wrapperQueryClient.collection({ collectionIn });
    return logAndReturn(res, isDisplayed);
  }

  return {
    utils: {
      cwQueryOperators,
      cwQueryApprovals,
      cwQueryBalanceInNft,
      cwQueryNftOwner,
    },
    nftMinter: {
      cwQueryConfig: cwNftMinterQueryConfig,
      cwQueryCollection: cwNftMinterQueryCollection,
      cwQueryCollectionList: cwNftMinterQueryCollectionList,
    },
    transceiver: {
      cwQueryConfig: cwTransceiverQueryConfig,
      cwQueryPauseState: cwTransceiverQueryPauseState,
      cwQueryOutposts: cwTransceiverQueryOutposts,
      cwQueryCollection: cwTransceiverQueryCollection,
      cwQueryCollectionList: cwTransceiverQueryCollectionList,
      cwQueryChannelList: cwTransceiverQueryChannelList,
    },
    wrapper: {
      cwQueryConfig: cwWrapperQueryConfig,
      cwQueryCollectionList: cwWrapperQueryCollectionList,
      cwQueryCollection: cwWrapperQueryCollection,
    },
  };
}

export { getCwExecHelpers, getCwQueryHelpers };
