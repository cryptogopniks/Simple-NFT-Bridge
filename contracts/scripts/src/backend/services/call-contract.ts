import { getSigner } from "../account/signer";
import { getLast, l, li, wait } from "../../common/utils";
import { readFile } from "fs/promises";
import { ChainConfig } from "../../common/interfaces";
import { ADDRESS, TOKEN } from "../../common/config";
import { COLLECTION } from "./constants";
import {
  ENCODING,
  PATH_TO_CONFIG_JSON,
  getWallets,
  parseStoreArgs,
} from "./utils";
import {
  getChainOptionById,
  getContractByLabel,
} from "../../common/config/config-utils";
import {
  getSgQueryHelpers,
  getSgExecHelpers,
} from "../../common/account/sg-helpers";
import {
  getCwExecHelpers,
  getCwQueryHelpers,
} from "../../common/account/cw-helpers";

async function main() {
  try {
    const { chainId } = parseStoreArgs();
    const configJsonStr = await readFile(PATH_TO_CONFIG_JSON, {
      encoding: ENCODING,
    });
    const CHAIN_CONFIG: ChainConfig = JSON.parse(configJsonStr);
    const {
      NAME,
      PREFIX,
      OPTION: {
        RPC_LIST: [RPC],
        DENOM,
        CONTRACTS,
        GAS_PRICE_AMOUNT,
        TYPE,
      },
    } = getChainOptionById(CHAIN_CONFIG, chainId);

    // const TRANSCEIVER_HUB_CONTRACT = getContractByLabel(
    //   CONTRACTS,
    //   "transceiver_hub"
    // );

    // const TRANSCEIVER_OUTPOST_CONTRACT = getContractByLabel(
    //   CONTRACTS,
    //   "transceiver_outpost"
    // );

    const gasPrice = `${GAS_PRICE_AMOUNT}${DENOM}`;
    const testWallets = await getWallets(TYPE);
    const { signer, owner } = await getSigner(PREFIX, testWallets.SEED_ADMIN);

    const sgQueryHelpers = await getSgQueryHelpers(RPC);
    const sgExecHelpers = await getSgExecHelpers(RPC, owner, signer);

    const { utils, transceiver, nftMinter } = await getCwQueryHelpers(
      chainId,
      RPC
    );
    const h = await getCwExecHelpers(chainId, RPC, owner, signer);

    const { getBalance, getAllBalances, getTimeInNanos } = sgQueryHelpers;
    const { sgMultiSend, sgIbcHookCall, sgSend } = sgExecHelpers;
    console.clear();

    const hubCollection = COLLECTION.MAINNET?.PIGEON?.NEUTRON || "";
    const homeCollection = COLLECTION.MAINNET?.PIGEON?.STARGAZE || "";
    const tokenList = ["1321", "1356"];

    // neutron
    try {
      // await h.nftMinter.cwCreateCollection("Pigeons Bandits", gasPrice);
      // const [[hubCollection]] = await nftMinter.cwQueryCollectionList();
      // li({ hubCollection, homeCollection });
      // await h.transceiver.cwAddCollection(
      //   hubCollection,
      //   homeCollection,
      //   gasPrice
      // );

      await h.transceiver.cwApproveAndSend(
        hubCollection,
        homeCollection,
        tokenList,
        {},
        100_001,
        TOKEN.NEUTRON.MAINNET.NTRN,
        gasPrice
      );

      await utils.cwQueryBalanceInNft(owner, hubCollection, true);
      await wait(6_000);
    } catch (e) {
      l(e);
    }

    // stargaze
    try {
      // await h.transceiver.cwAddCollection(
      //   hubCollection,
      //   homeCollection,
      //   gasPrice
      // );

      await h.transceiver.cwApproveAndSend(
        hubCollection,
        homeCollection,
        tokenList,
        {},
        1,
        TOKEN.STARGAZE.MAINNET.STARS,
        gasPrice
      );

      await utils.cwQueryBalanceInNft(owner, homeCollection, true);
      await wait(6_000);
    } catch (e) {
      l(e);
    }
  } catch (error) {
    l(error);
  }
}

main();
