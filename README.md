# Polka Native Storage

This repository contains the implementation of the Polka Native Storage (PNS) project. It is a Substrate-based system parachain intended to introduce decentralized storage technology to Polkadot.

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
cargo build --release
```

The above is enough to have your own node. To test it please refer to the [testing guide](./doc/testing_guide.md) for more information about how to setup local relay chain and how to incorporate PNS parachain into local development environment.

To join the other networks please refer to the appropriate documentation of the network you would like to join.

## Docker
It is possible to generate a docker image containing a working `polka-storage-node` with all pallets built-in. It also contains a local relay-chain for testing. To generate an image, run:
```bash
sudo docker build -t "pns:Dockerfile" .
```

When the build is ready, you can check if you can see the image in the docker repository by running:
```bash
sudo docker images
```

To run the image, enter:
```bash
sudo docker run pns:Dockerfile
```

It will start the `polka-storage-node` on the local interface and two relay chain nodes. You can change the default behavior by passing your own command when running the docker image. All available options are in the [node template](https://docs.substrate.io/reference/command-line-tools/node-template/) documentation.

