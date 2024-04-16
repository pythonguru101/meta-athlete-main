import { ApiPromise } from "@polkadot/api";
import { AddressOrPair } from "@polkadot/api/types";
import { sendTransactionAsync } from "../src/utils";

import "../src/interfaces/augment-api";

async function mintAthletes(api: ApiPromise, signer: AddressOrPair, id_: string): Promise<any> {
  const cardHashes: Set<string> = new Set();
  
  const tx = api.tx.athletes.mintCards(id_);
  console.log(`Minting card ${id_}`)
  const eventMint = await sendTransactionAsync(api, signer, tx, `mint cards for ${id_}`);

  for (const record of eventMint.events) {
    const event = record.event;
    console.log(`event ${event}`);
    if (api.events.athletes.CardMinted.is(event)) {
      const cardHash = event.data.toString();
      console.log(`Card minted: ${cardHash}`);
      return cardHash;
      cardHashes.add(cardHash); // Only one Gold card minted
    }
  }
  
}
export { mintAthletes };

