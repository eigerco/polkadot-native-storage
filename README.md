# Polka Native Storage Research

> [!IMPORTANT]
> This repository contains the research and poc implementation of the Polka Native Storage grant project completed in Q4 2023. A Substrate-based system parachain intended to introduce decentralized storage technology to Polkadot.
> You can read the results [here](https://github.com/eigerco/polkadot-native-storage/blob/main/doc/report/polkadot-native-storage-v1.0.0.pdf)

A significant part of the research was the implementation of a proof-of-concept, illustrating several aspects of our planned architecture. The PoC demonstrates:
- parachain registration to a local relay chain (test environment)
- parachain block production
- XCM: sending messages between parachains using sudo pallet and ping pallet (in a non-test environment, the sudo pallet would be a system vulnerability and a proper governance mechanism should be used.)
- visibility of the Filecoin actors as pallets in the local parachain
- ability to run parts of the FVM code in the Substrate runtime (WASM)
- the ability to interact with the actors (e.g. miner actor - add miner, change worker address) using extrinsics
- RPC endpoint interaction
- ability to control collator selection set


Repository structure:
- `doc` - contains the documentation of the project
- `node` - contains the implementation of the node along with the RPC features
- `pallets` - contains the implementation of the PNS runtime modules
- `polkadot-launch` - contains the configuration of the local development environment (chains)
- `runtime` - contains the implementation of the parachain runtime
- `scripts` - contains the scripts used to build or test the project

## Building the node
Download the `polka-native-storage` parachain repository:
```bash
git clone https://github.com/eigerco/polkadot-native-storage polka-storage
```

Then build it:
```bash
cd polka-storage
cargo build --release
```

The above is enough to have your own node. To test it please refer to the [testing guide](./doc/testing_guide.md) for more information about how to setup local relay chain and how to incorporate PNS parachain into local development environment.

To join the other networks please refer to the appropriate documentation of the network you would like to join.

## Docker
It is possible to generate a docker image containing a working `polka-storage-node` with all pallets built-in. It also contains a local relay-chain for testing. To generate an image, run:
```bash
sudo docker build -t "pns:Dockerfile" .
```

Please note that building may require a lot of time and resources. Linux system is preferred to build the image.

When the build is ready, you can check if you can see the image in the docker repository by running:
```bash
sudo docker images
```

To run the image, enter:
```bash
sudo docker run pns:Dockerfile
```

It will start the `polka-storage-node` on the local interface and two relay chain nodes. You can change the default behavior by passing your own command when running the docker image. All available options are in the [node template](https://docs.substrate.io/reference/command-line-tools/node-template/) documentation.

To stop the nodes you may use `docker stop` command:
```bash
sudo docker stop <container_id>
```

## About [Eiger](https://www.eiger.co)

We are engineers. We contribute to various ecosystems by building low-level implementations and core components. We contribute to Polkadot because we believe in the vision. In addition to our efforts to implement a storage network we also brought Move to Substrate, read more about that on [our blog](https://www.eiger.co/blog/eiger-brings-move-to-polkadot). We are also currently working on Strawberry, our Go implementation of JAM.

Contact us at hello@eiger.co
Follow us on [X/Twitter](https://x.com/eiger_co)
