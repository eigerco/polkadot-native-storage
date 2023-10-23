# Testing guide
Welcome to the Testing Guide! This comprehensive guide is designed to equip you with the knowledge and tools necessary to understand and execute all prepared tests.

This tutorial is based on the official Substrate tutorials and documentation.

## Testing approach
Several different testing approaches were used during the various phases of software development. These approaches aim to ensure the quality and reliability of the software and provide a comprehensive look into the software quality.

Test layers refer to the testing stages that are performed to ensure the quality and reliability of the software. During this project, we will use the following layers:
- Source analysis.
- Unit testing.
- Integration testing.
- System (black-box) testing.

The testing approach will differ depending on where new or modified code appears. Our own code will be tested on all layers.

All changes done to the Filecoin codebase, Filecoin Virtual Machine or other parts or components related to the Filecoin project will be covered by the test set compatible with other functionality already present in that module. If we ever introduce new functions, they will be covered by the same test suite as the other parts.

Changes introduced to the Substrate parachain engine will be covered by the test set compatible with other functionality already present in the node unless they require new mechanisms that aren't yet used.

Finally, black-box testing will be applied to the whole parachain. It will be performed by any team (including the client) and will be based on the specifications and requirements.

## Infrastructure & testing environment
To verify ideas and test the project implementation, we have prepared a local testing environment. It consists of a local relay chain and a local parachain with proper configuration.

Required infrastructure:
- local relay chain (two nodes) to check parachain registration and parachain block production;
- local parachain with at least one collator - Polka Native Storage parachain (this repository);
- Polkadot UI - to interact with the local relay chain and local parachain;
- another sample parachain (it can be even template pallet) and sudo pallet (included in the local setup) to test XCM.

Required tools:
- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)
- [Yarn](https://classic.yarnpkg.com/en/docs/install/#debian-stable)

Users should also have a basic understanding of the Substrate framework and the Polkadot ecosystem.

The requirements we can test with the local setup and provided PoC code:
- parachain registration to local relay chain;
- parachain block production;
- XCM (Cross-Chain Messaging) - sending messages between parachains using sudo pallet and ping pallet;
- visibility of the Filecoin actors as pallets in the local parachain;
- ability to run parts of the FVM code in the Substrate runtime (WASM environment);
- the ability to interact with the actors (e.g. miner actor - add miner, change worker address) using extrinsics;
- RPC endpoint interaction (hardcoded returned values);
- ability to control collator selection set.


### Local relay chain
You'll need to download and compile some parts of `polkadot-sdk` repository in order to set up the local relay chain.

Download the repository and checkout the stable branch:
```bash
git clone https://github.com/paritytech/polkadot-sdk.git
git checkout polkadot-v1.1.0
```

Then, build the relay chain node:
```bash
cd polkadot
cargo build --release
```

### Polkadot UI
You'll need to download and run Polkadot UI to interact with local relay chain.
```bash
git clone https://github.com/polkadot-js/apps polkadot-js
```

Then, install dependencies and run the UI:
```bash
cd polkadot-js
yarn install
yarn start
```

### Local parachain (collator)
Download the `polka-storage` parachain repository:
```bash
git clone https://github.com/eigerco/polka-storage.git
```

Then build it:
```bash
cargo build --release
```

### Putting local parachain and relay chain together
To run a local parachain you'll need to run the local relay chain first. The following directory structure is assumed:
- root directory
- - polkadot-sdk
- - polka-storage
- - polkadot-js

Ensure that `polkadot-js` is running.

Run the local relay chain (two nodes):
```bash
cd polkadot-sdk
./target/release/polkadot --alice --validator --base-path /tmp/relay/alice --chain ../polka-storage/polkadot-launch/spec/raw-local-chainspec.json --port 30333 --rpc-port 9944
./target/release/polkadot --bob --validator --base-path /tmp/relay/bob --chain ../polka-storage/polkadot-launch/spec/raw-local-chainspec.json --port 30334 --rpc-port 9945
```

Check the UI to see if the local testnet is visible. If so, go to Network -> Parchains -> Parathreads and click "+" ParaId button. Ensure if the parachain id is 2000 (configuration in our repository assumes that's your only local parachain). If you have other parachains, get the first free parachain id and modify files under `polka-storage/polkadot-launch/spec/`.

At this point we're assuming that the default parachain id 2000 is being used (so you can use the provided files).

Go to `polka-storage` directory.

Export WASM verification code:
```bash
./target/release/polka-storage-node export-genesis-wasm --chain polkadot-launch/spec/raw-parachain-chainspec.json polkadot-launch/spec/para-2000-wasm
```

Export genesis state:
```bash
./target/release/polka-storage-node export-genesis-state --chain polkadot-launch/spec/raw-parachain-chainspec.json polkadot-launch/spec/para-2000-genesis-state
```

Run the local parachain:
```bash
cd polka-storage
./target/release/polka-storage-node --alice --collator --force-authoring --chain polkadot-launch/spec/raw-parachain-chainspec.json --base-path /tmp/parachain/alice --port 40333 --rpc-port 8844 -- --execution wasm --chain polkadot-launch/spec/raw-local-chainspec.json --port 30343 --rpc-port 9977
```

Now go to the UI and navigate to Developer -> Sudo. Choose `parasSudoWrapper` and `sudoScheduleParaInitialize` function. Enter 2000 as `id`, then click `file upload` on `genesisHead` field and choose `polka-storage/polkadot-launch/spec/para-2000-genesis-state`. Click `file upload` on `validationCode` field and select `polka-storage/polkadot-launch/spec/para-2000-wasm`. `paraKind` field should be set to `true`. Click the `Submit sudo` button and select `Sign and submit` when asked.

Navigate to Network -> Parachains and watch for events. Parachain should be initially registered (event should occur) and be visible as parathread, and after a while, it should be visible as parachain.

Congratulations - you have successfully set up a local testing environment! Next time, you'll only need to run the local relay chain and local parachain.

### Unit and integration testing
Testing can be performed also with single packages like `pallet-miner`. We've implemented sample testing suite. To run it, go to `polka-storage` root directory and execute:
```bash
cargo test -p pallet-miner
```

You should see all tests passing:
```bash
Running unittests src/lib.rs (target/debug/deps/pallet_miner-12c4a6dc2e70bd36)

running 18 tests
test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
test tests::change_owner_address_creates_proposal_with_valid_signer ... ok
test tests::change_owner_address_confirms_new_owner_with_valid_signer_and_proposal ... ok
test tests::change_owner_address_rejects_invalid_signer ... ok
test tests::change_owner_address_rejects_proposal_with_owner_account ... ok
test tests::change_owner_address_revokes_existing_proposal_with_valid_signer ... ok
test tests::change_peer_id_rejects_invalid_signer ... ok
test tests::change_peer_id_works_with_valid_owner ... ok
test tests::change_peer_id_works_with_valid_controller ... ok
test tests::change_worker_address_clears_pending_worker_with_valid_signer_and_old_worker ... ok
test tests::change_worker_address_works_with_valid_signer_and_new_worker ... ok
test tests::change_worker_address_keeps_old_controller_without_override ... ok
test tests::change_worker_address_rejects_invalid_signer ... ok
test tests::confirm_update_worker_accepts_effective_request_with_valid_signature ... ok
test tests::create_miner ... ok
test tests::change_worker_address_rejects_trigger_without_request ... ok
test tests::create_miner_first_miner_addr_is_correct ... ok
test tests::confirm_update_worker_key_rejects_trigger_before_effective_at ... ok
```

### RPC testing
RPC testing is performed by running the local parachain and sending requests to the RPC endpoint. The endpoint is available at `localhost:9977` by default (can be changed).

The parameters are passed as JSON objects. The following example shows how to get the miner address:
```bash
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "pns_chainGetBlock", "params": ["123"]}' http://localhost:9977/
```

For above example, the response should be:
```bash
{"jsonrpc":"2.0","result":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"id":1}
````

### XCM testing
XCM testing is performed by running the local parachain and sending XCM messages using the sudo pallet.

In order to run the XCM message exchange, make sure you can log into the UI and switch between the relay chain and the collator nodes (right WS ports).

Make sure you have enough capacity in your configuration. Click Developer and select Chain State -> configuration, then select activeConfig(). Check the parameter values for hrmpChannelMaxCapacity and hrmlChannelMaxMessageSize. They should be non-zero values.

To establish HRMP (it's XCMP-lite) communication channels, you can follow the [official guide](https://docs.substrate.io/tutorials/build-a-parachain/open-message-passing-channels/). Alternatively, you can try to do it using extrinsics: paraSudoWrapper (used previously to set up parachain) with `sudoEstabllishHrmpChannel`. Then you can just enter there two ParaIDs (like 2000 and 2001 - the second ID should come from other external parachain), max capacity and message size.

When the channel setup is finished you can try to send messages between parachains. You can use `pallet-ping` (incorporated as `pingbot`) to send pings between the parachains. There is also a possibility to execute an extrinsic or send a message using `polkadotXCM` pallet and `send` [extrinsic](https://docs.substrate.io/tutorials/build-a-parachain/transfer-assets-with-xcm/).

## Benchmarking
Benchmarking and updating weights should be done each time a new extrinsic is added to any pallet (weights are used to estimate transaction fees). Weights are obligatory for extrinsics that are available for users.

To update weights, the user can run the following command:
```bash
./target/release/polka-storage-node benchmark pallet --chain dev --pallet pallet-name --steps=50 --repeat=20 --wasm-execution=compiled --output pallets/pallet-name/src/weights.rs --template ./.maintain/frame-weight-template.hbs --extrinsic '*'
```
when being in the PNS root directory. The template for the weights is located under `./.maintain/frame-weight-template.hbs` directory and can be obtained from the Substrate repository.

Please note that the benchmarking process is not ready for the PoC code yet and will be available in the future. For now, the weights are set manually.
