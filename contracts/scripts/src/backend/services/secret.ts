import { Wallet, SecretNetworkClient } from "secretjs";
import { l, li } from "../../common/utils";
import { getWallets } from "./utils";
import { readFile } from "fs/promises";
import { rootPath } from "../envs";
import { gzip } from "pako";

async function main() {
  const testWallets = await getWallets("main");
  const seed = testWallets.SEED_ADMIN;
  const wallet = new Wallet(seed, { coinType: 118 });
  l(wallet.address);

  const secretjs = new SecretNetworkClient({
    url: "https://rest.lavenderfive.com:443/secretnetwork",
    chainId: "secret-4",
    wallet,
    walletAddress: wallet.address,
  });

  const { balances } = await secretjs.query.bank.allBalances({
    address: wallet.address,
  });
  li({ balances });

  // Upload the wasm of a simple contract
  const wasmBinary = await readFile(rootPath(`../artifacts/transceiver.wasm`));
  const compressed = gzip(wasmBinary, { level: 9 });

  const tx = await secretjs.tx.compute.storeCode(
    {
      sender: wallet.address,
      wasm_byte_code: compressed,
      source: "",
      builder: "",
    },
    {
      gasLimit: 1_500_000,
    }
  );

  const codeId = Number(
    tx.arrayLog?.find((log) => log.type === "message" && log.key === "code_id")
      ?.value
  );

  li({ codeId });
}

main();
