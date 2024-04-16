# MetaAthlete Project Setup Guide
This guide explains how to set up the MetaAthlete project step-by-step.

## 1. Prerequisites
To set up the project, you need the following:

- Command line interface (CLI)
- Node.js and Yarn installed (basic JS development environment).
- Access to a MetaAthlete node (either locally or deployed somewhere).
- Make sure you have all the necessary JS dependencies installed by running the following command in the project's root directory:

`cd ts-client`
`yarn install`

If you don't have Yarn installed, run`npm install --global yarn` to install it.

### 1.1 Building the node locally (if needed)
This step can be skipped if you're using an already deployed node.

If you need to build the node locally, make sure you have the `wasm32-unknown-unknown` Rust toolchain installed. Then, in the root of the project, run the following command:

`cargo build --release`
To run the node in dev mode, run:

`target/release/meta-athlete-node`

## 2. Tweaking connection settings
Make sure the `NODE_ENDPOINT` constant in `ts-client/src/utils.ts` points to the correct node. For example:

`const NODE_ENDPOINT = 'ws://127.0.0.1:9944'` 
or 
`const NODE_ENDPOINT = 'ws://137.184.98.120:9944'`

### 2.1 Setting up a Substrate explorer
You can also set up a Substrate explorer using Docker with the following command:

`docker run --rm -it --name polkadot-ui -e WS_URL=ws://137.184.98.120:9944 -p 9999:80 jacogr/polkadot-js-apps:latest`

## 3. Tweaking uploaded data
By default, the script exports four athletes in the following format:

```ts
export const TEST_ATHLETES: Athlete[] = [
  {
    name: "John Doe",
    weight: 82.5,
    height: 1.80,
  },
  {
    name: "Bobby Smith",
    weight: 78,
    height: 1.83,
  },
  {
    name: "Kyle Abrams",
    weight: 75,
    height: 1.75,
  },
  {
    name: "David Simpson",
    weight: 85,
    height: 1.90,
  },
]
```
You can edit the list of athletes to be uploaded by modifying the ts-`client/src/athletes.ts` file.

## 4. Registering the athletes
To register the athletes, navigate to the `ts-client` directory and run the following command:

`yarn athletes:register`
This command performs the following tasks:

Reads athletes from `athletes.ts`
Submits a registration application for each athlete
Approves the registration for each athlete (placeholder for KYC process)

## 5. Minting the NFTs
To mint the NFTs, run the following command:

`yarn athletes:mint`
This command mints 10 Diamond, 50 Platinum, and 100 Gold NFT cards for each athlete. The cards are initially owned by the system account (i.e., no one).

## 6. Viewing on-chain summary
To view a short summary about each registered athlete, the number of cards available for each athlete, and a random card pulled from the blockchain state, run the following command:

`yarn athletes:summary`.

# Setting up the Dockerfile
To set up the project using Docker, follow the steps below:

1. Build the Docker image by running the following command in the root of the project:
`docker build -t meta-athlete .`

2. Run the Docker container by executing the following command:

`docker run -it -p 30333:30333 -p 9933:9933 -p 9944:9944 --name meta-athlete meta-athlete`

The above command will start a new container with the name `meta-athlete` and expose the ports 30333, 9933 and 9944. You can change the container name and port bindings if needed.