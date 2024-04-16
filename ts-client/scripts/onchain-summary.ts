import { ApiPromise} from "@polkadot/api";
import { AddressOrPair} from "@polkadot/api/types";
import { Option } from "@polkadot/types-codec"
import type { H256 } from '@polkadot/types/interfaces/runtime';

import { MetaAthletePrimitivesCard } from "@polkadot/types/lookup";

function hex_to_ascii(str: string) {
	var hex  = str.toString();
	var res = '';
	for (var n = 0; n < hex.length; n += 2) {
		res += String.fromCharCode(parseInt(hex.substr(n, 2), 16));
	}
	return res;
}

async function getAthletes(api: ApiPromise, signer: AddressOrPair, athlete_id: string, card_hash: H256): Promise<any> {
  const athlete = (await api.query.athletes.athletes(athlete_id)).unwrap();
  const card = (await api.query.athletes.cards(card_hash)).unwrap();

  const name = hex_to_ascii(athlete.name.toString());
  const kind = hex_to_ascii(athlete.kind.toString());
  const sports = hex_to_ascii(athlete.sports.toString());
  const birthdate = hex_to_ascii(athlete.birthdate.toString());
  const birthplace = hex_to_ascii(athlete.birthplace.toString());
  const schoolgrade = hex_to_ascii(athlete.schoolgrade.toString());
  const height = Number(athlete.height.millimeters.toBigInt()) / 10;
  const weight = Number(athlete.weight.grams.toBigInt()) / 1000;
  console.log(`Queried athlete ${name}: kind=${kind} sports=${sports} birthdate=${birthdate} birthplace=${birthplace} schoolgrade=${schoolgrade} height=${height}cm weight=${weight}kg`);
  console.log(`Queried card ${card}`);
  
  return athlete;
}

export { getAthletes };
