#!/bin/bash
polkadot --alice --validator --base-path /tmp/relay/alice --chain /usr/local/etc/raw-local-chainspec.json --port 30333 --rpc-port 9944 &
polkadot --bob --validator --base-path /tmp/relay/bob --chain /usr/local/etc/raw-local-chainspec.json --port 30334 --rpc-port 9945 &

polka-storage-node --alice --collator --force-authoring --chain /usr/local/etc/raw-parachain-chainspec.json --base-path /tmp/parachain/alice --port 40333 --rpc-port 8844 -- --execution wasm --chain /usr/local/etc/raw-local-chainspec.json --port 30343 --rpc-port 9977

