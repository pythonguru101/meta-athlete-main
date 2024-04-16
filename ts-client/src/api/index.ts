export * as connection from "./connection"
export * as athletes from "./athletes"

import express, { Request, Response } from 'express';
import cors from 'cors';
import bodyParser from 'body-parser';

import { registerAthletes } from "../../scripts/register-athletes";
import { mintAthletes } from "../../scripts/mint-cards";
import { getAthletes } from "../../scripts/onchain-summary";

import { connect } from "../../src/utils";

const app = express();

app.use(cors());
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

const PORT = process.env.PORT || 3000;

app.listen(PORT, () => {
  console.log(`Server listening on port ${PORT}`);
});

let card_hashes: Set<string> = new Set();

app.get('/', (req: Request, res: Response) => {
  res.send('Hello World!');
});


// Define a new endpoint for registering an athlete
app.post('/register-athlete', async function (req: Request, res: Response) {
  try {
    const { athlete_body } = req.body;
    let { api, alice } = await connect();
    const registered_id = await registerAthletes(api, alice, athlete_body);
    res.send({ message: "Athlete registered", registered_id });
  } catch (error) {
    console.error(error);
    res.status(500).send('Internal server error');
  }
});

// Define a new endpoint for minting tokens
app.post('/mint/:id', async function (req: Request, res: Response) {
  try {
    // const { athleteId, name } = req.body;
    let { api, alice } = await connect();
    card_hashes = await mintAthletes(api, alice, req.params.id);
    res.send({ message: "NFT Hashes Gold/Platinum/Diamond", card_hashes });

  } catch (error) {
    console.error(error);
    res.status(500).send('Internal server error');
  }
});

  // Define a new endpoint for getting athlete details
  app.get('/get/:athlete_id/', async function (req: Request, res: Response) {
    try {

      let { api, alice } = await connect();
      const athleteId = req.params.id;

      console.log(`Getting NFT Athlete ${card_hashes}`);

      api.registerTypes({
        AthleteCardClass: {
          _enum: ['Gold', 'Platinum', 'Diamond']
        }
      })

      let card_hashes_array = Array.from(card_hashes); // convert Set to Array
      console.log(`Getting NFT Cards ${card_hashes_array}`);

      const athleteDetails = [];
      for (let i = 0; i < card_hashes_array.length; i++) {
        const card_hash = card_hashes_array[i];
        console.log(`Card hash ${card_hash}`, typeof card_hash);
      
        const info = await getAthletes(api, alice, athleteId, card_hash);
        console.log(`NFT Card info ${info}`);
        athleteDetails.push(info);
      }

      res.send(`NFT information onchain for Athlete ${athleteId}:\n${athleteDetails.join('\n')}`);
    } catch (error) {
      console.error(error);
      res.status(500).send('Internal server error');
    }
  });

