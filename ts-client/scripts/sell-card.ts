import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { connect, sendTransactionAsync } from "../src/utils";
import type { AccountId32, H256 } from '@polkadot/types/interfaces/runtime';
import { hexToU8a } from '@polkadot/util/hex'

import "../src/interfaces/augment-api";

(async () => {
  let { api, keyring } = await connect();

  const bob = keyring.addFromUri('//Bob', { name: 'Bob' });
  const cardHex = process.argv[2];
  const cardKey: H256 = api.createType("H256", hexToU8a(cardHex));

  const tx = api.tx.athletes.sellCard(cardKey);
  await sendTransactionAsync(api, bob, tx, `bob sells card ${cardHex}`);
})()
