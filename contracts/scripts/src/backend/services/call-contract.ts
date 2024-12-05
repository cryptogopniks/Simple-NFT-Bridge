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

    const TRANSCEIVER_HUB_CONTRACT = getContractByLabel(
      CONTRACTS,
      "transceiver-hub"
    );

    const TRANSCEIVER_OUTPOST_CONTRACT = getContractByLabel(
      CONTRACTS,
      "transceiver-outpost"
    );

    const gasPrice = `${GAS_PRICE_AMOUNT}${DENOM}`;
    const testWallets = await getWallets(TYPE);
    const { signer, owner } = await getSigner(PREFIX, testWallets.SEED_ADMIN);

    const sgQueryHelpers = await getSgQueryHelpers(RPC);
    const sgExecHelpers = await getSgExecHelpers(RPC, owner, signer);

    const { utils, transceiver } = await getCwQueryHelpers(chainId, RPC);
    const h = await getCwExecHelpers(chainId, RPC, owner, signer);

    const { getBalance, getAllBalances, getTimeInNanos } = sgQueryHelpers;
    const { sgMultiSend, sgIbcHookCall, sgSend } = sgExecHelpers;
    console.clear();

    // await getBalance(owner, TOKEN.NEUTRON.TESTNET.USDC);

    const collectionList = [
      COLLECTION.TESTNET?.PIGEON?.NEUTRON || "",
      COLLECTION.TESTNET?.BAD_KIDS?.NEUTRON || "",
      COLLECTION.TESTNET?.SLOTH?.NEUTRON || "",
    ];
  } catch (error) {
    l(error);
  }
}

main();
