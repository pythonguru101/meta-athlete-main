import { ApiPromise, HttpProvider, WsProvider } from "@polkadot/api";
import * as fs from "fs";

(async () => {
  const wsProvider = new WsProvider('ws://127.0.0.1:9944');
  // const wsProvider = new WsProvider('ws://139.144.62.226:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  const metadata = await api.rpc.state.getMetadata();
  fs.writeFile("metadata-human.json", JSON.stringify(metadata.toJSON(), null, 4), () => {});
})()

