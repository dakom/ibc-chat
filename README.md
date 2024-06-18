
<h1 align="center">
   IBC Chat 
</h1>
<h1 align="center">
   The world's worst* chat application
</h1>
<p align="center" width="100%">
    <img width="33%" src="media/ibc-chat-logo-small.jpg">
</p>


<div align="center">
(*that is, the slowest, most expensive, and fragile... but technically interesting!)
</div>

# Table of Contents

- [Cool stuff](#cool-stuff)
- [Live demo](#live-demo)
- Setup
   - [Prerequisites](#prerequisites)
   - [Local chains](#local-chains)
   - [Easy setup](#easy-setup)
   - [Detailed setup](#detailed-setup)
- [IBC Flow](#ibc-flow)
- Development
   - [Configuration](#configuration)
   - [Frontend](#frontend)
   - [End to End Tests](#end-to-end-tests)
   - [Contracts](#end-to-end-tests)
   - [CLI](#cli)

# Cool stuff

* 100% Fullstack Rust, types are shared between contracts, deploy, on-chain tests, off-chain/multitest, and frontend!! (all powered by wasm with cosmjs bindings :))
* Auto-generated Rustdoc for [shared types](https://dakom.github.io/ibc-chat/doc/shared/) contract messages, including IBC and events (from the `shared` package) and [wallet types](https://dakom.github.io/ibc-chat/doc/wallet/) for wallet crate.
* Client-side "sdk-like" methods (i.e. contract API) are shared between all clients (including on-chain, off-chain, and frontend)
  - e.g. the [ContractClient](https://dakom.github.io/ibc-chat/doc/wallet/contract_traits/trait.ContractClient.html) and [ContractServer](https://dakom.github.io/ibc-chat/doc/wallet/contract_traits/trait.ContractServer.html) - which are traits with implementations everywhere. Just call the methods and it works, whether in multi-test, on-chain tests, or frontend!
* Taskfile with simple commands to make setup and development _much_ less painful (`task --list-all` to skip past the Readme, sorta).
* Shared config, one file to configure the network, one auto-generated file to maintain contract addresses, ibc ports, etc. across all delivery flows

# [Live Demo](https://dakom.github.io/ibc-chat/)

#### Requires running a relayer!

#### RPC endpoints may also rate-limit, block cors, etc... real-world would require a private node, most likely

# Setup 

## Prerequisites

* [Rust](https://www.rust-lang.org/)
* [Go](https://go.dev/)
* [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) (nodejs package manager)
* [Taskfile](https://taskfile.dev) (overall task runner) 
* [jq](https://jqlang.github.io/jq/download/) (parse json on commandline)
* [Trunk](https://trunkrs.dev/) (for frontend dev/deploy)
* [http-server-rs](https://github.com/http-server-rs/http-server) (for frontend local media serving)
* anything else the commandline tells you to install :)

## Local chains

1. Create docker images for chains:
   - Neutron: 
      - clone [https://github.com/neutron-org/neutron](https://github.com/neutron-org/neutron)
      - see https://docs.neutron.org/neutron/build-and-run/neutron-build#1-make-sure-you-have-the-required-golang-version for the correct release version
      - `docker buildx build --load --build-context app=. -t local-neutron --build-arg BINARY=neutrond .`
   - Kujira
      - clone [https://github.com/Team-Kujira/pond-images](https://github.com/Team-Kujira/pond-images)
      - in `kujira/` subdirectory: `docker build --build-arg go_version=1.21.3 --build-arg kujira_version=03985a2 -t local-kujira .`
      - the exact go and kujira versions are taken from https://github.com/Team-Kujira/pond-images/blob/main/versions.yml
   - Stargaze
      - clone [https://github.com/public-awesome/stargaze](https://github.com/public-awesome/stargaze)
      - `docker build -t local-stargaze .`
   - Nois
      - clone [https://github.com/noislabs/noisd](https://github.com/noislabs/noisd)
      - `docker build -t local-nois .`
2. Create containers for chains: `task chains-create`
3. Start chains: `task chains-start`

Then you can stop and start the local chains as-needed with `task chains-stop` / `task chains-start`

Debugging individual chains by shell can be done via `task chain-sh-[neutron | kujira | stargaze | nois]`.


## Easy setup 

If targeting locally, make sure you've started the local chains (see `Local Chains` above)

If you already have all the depenencies, wallet setup, etc., then it's as easy as:
1. edit `.env.example` to add your seed phrases and rename to `.env`
2. `task cli-prepare`
3. `task contracts-deploy-[local | testnet]`
4. `task relayer-setup-[local | testnet]`

That's all for initial setup, then, to get a live working environment

1. `task relayer-start-[local | testnet]` (in its own terminal) 
2. `task frontend-dev-[local | testnet]` (in its own terminal) 

As contracts change and you want to migrate, just `task contracts-migrate-[local | testnet]` and hard reload the frontend

If you need a full redeployment (new addresses, wipe the state, etc.):

1. kill the relayer and frontend if they're running 
2. `task contracts-deploy-[local | testnet]`
3. `task relayer-create-channels-[local | testnet]`
4. Restart the relayer and frontend (in their own terminals, as above) 

## Detailed setup

Step-by-step instructions to allow for debugging individual commands etc.

If targeting locally, make sure you've started the local chains (see `Local Chains` above)

1. (testnet-only) make sure you have all the testnets installed available in Keplr
   - Neutron: https://neutron.celat.one/pion-1 and hit "connect wallet"
   - Kujira: https://github.com/SynergyNodes/Add-Kujira-Testnet-to-Keplr (maybe use Polkachu RPC nodes instead, as in the network.json file here)
   - Stargaze: https://violetboralee.medium.com/stargaze-network-how-to-add-stargaze-testnet-to-keplr-cosmostation-leap-and-get-test-stars-5a6ae2ca494f
   - Nois: https://address.nois.network/
   - you may then need to go to keplr settings and "adjust chain visibility" to see balance / check address / etc.
2. (testnet-only) get some tokens
   - Neutron: https://docs.neutron.org/neutron/faq/#where-is-the-testnet-faucet
   - Kujira: via the #public-testnet-faucet channel on Discord
   - Stargaze: https://violetboralee.medium.com/stargaze-network-how-to-add-stargaze-testnet-to-keplr-cosmostation-leap-and-get-test-stars-5a6ae2ca494f
      - May need to manually add the #faucet channel
   - Nois: 
3. (one-time) edit .env.example to add your seed phrases (needed for deploying contracts, local chains, running the relayer, not as a regular user), and rename to .env
4. (one-time) setup the relayer
   - Install the Go relayer: https://github.com/cosmos/relayer
   - Initialize the Go relayer: `rly config init`
   - Configure the chains: `task relayer-add-chains-[local | testnet]`
   - Configure the wallet: `task relayer-add-wallet-[local | testnet]`
   - Make sure the wallet has funds: `task relayer-check-wallet-[local | testnet]`
      - each chain should have non-zero balance
   - Create paths: `task relayer-create-paths-[local | testnet]`
   - Create clients: `task relayer-create-clients-[local | testnet]`
   - Create connections: `task relayer-create-connections-[local | testnet]`
   - Create channels: gotcha! don't do that yet :) it will read the deploy file to get the ibc ports, so we do that after deploying contracts
5. (one-time) Setup CLI
   - in `deployer`, run `npm install` and `npm run build`
   - in `onchain-tests`, run `npm install` and `npm run build`
   - (`task cli-prepare` will do those altogether)
6. (one-time and hacking) build and deploy contracts
   - `task contracts-deploy-[local | testnet]` (see notes about contract building below)
   - `task relayer-create-channels-[local | testnet]`
7. (one-time and hacking) run the frontend locally
   - `task frontend-dev-[local | testnet]` (in its own terminal)
8. (one-time and hacking) start relaying
   - `task relayer-start-[local | testnet]` (in its own terminal)

The order in all the above is somewhat important, but once you're off to the races, different parts can be iterated (e.g. redeploying contracts and recreating ibc channels)

# IBC Flow

Neutron: server
Kujira, Stargaze, Nois: clients

Client sends a single message to server.
Server broadcasts this message to all clients.

All over IBC

# Configuration

## Deploy 
A deploy.json is written to locally, keeping addresses, ports, and hash digests in sync - don't touch it!!!

This allows simply deploying from commandline and then importing the same config file in the frontend.

If contracts don't change, it checks the hash to avoid rebuilding

## Network

The root-level `network.json` file is used to configure both the frontend *and* cli tools (deployer and test runner). *do* touch this - though all the values are currently setup and shouldn't need changing :)

For hacking around, various packages have their own config.rs for things like jumping straight into a page to debug, reload timer length, etc.

# Frontend

* `task frontend-dev-[local | testnet]` to start a local server and hack away
* Deployment is done to github pages via CI. Can be run manually too of course (see CI commands)

Strings are handled via Fluent project, different languages can be added (currently English and Hebrew-ish)
(this was only partially done due to time, but it's all setup)

Rust bindings to wallets are done via global-level UMD imports

Those UMD scripts are from:

* CosmWasmJS: https://github.com/dakom/CosmWasmJS
   - pretty much just re-exports the native cosmjs modules since the official project is no longer maintained

# Tests

* onchain (end-to-end): `task test-[only?]-onchain-[local | testnet]`
* offchain (multitest): `task test-offchain`

Onchain tests use the `cli` semantics and follow the same "only"-style optimizations, described below

# Contracts

* `task contracts-[deploy | migrate]-[local | testnet]` will do everything needed (build and deploy/migrate)

For more fine-grained control: contracts can be pre-built via `task contracts-build`

The build tool used is set by the `CONTRACTS_BUILD_TOOL` env var

* native: uses tools installed on the system, not the docker optimizer. Requires that `binaryen` and `sha256sum` be installed
* docker: the docker optimizer tool
* docker_arm: the docker optimizer tool for arm systems (e.g. apple silicon)

For the sake of speed, the `deployer` tool is _not_ rebuilt every time it's run - which is why the setup instructions included preparing the cli. If the tool does need to be rebuilt, e.g. if Instantiation or Migration messages change, remember to `task deployer-build` to update the tool itself.

# CLI

Currently there are two cli tools: the deployer and the onchain-tests. This could easily be expanded to bots etc.

Similar to the frontend, they are written in Rust/WASM with JS bindings to CosmJS.

If the tool itself doesn't need to be rebuilt, then it can be run with the `-only` segment... but this is typically not called directly, and in some cases hidden under an `internal` Taskfile rule. 

# WALLET

The wallet is a shared crate between frontend and CLI tools.

It depends on being passed an object with the following from `cosmjs`:

* SigningCosmWasmClient
* GasPrice
* createBatchQueryClient

On the frontend, this is just `window.CosmWasmJS` (since it imports the UMD build).
On the CLI side, it's extracted via the rollup bundle, which has the CosmJS modules.