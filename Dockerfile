# syntax=docker/dockerfile:1
FROM rust:slim as builder
RUN apt-get update && apt-get upgrade -y && apt-get install -y make gcc g++ curl clang protobuf-compiler git
RUN rustup default nightly
RUN rustup default stable
RUN rustup target add wasm32-unknown-unknown --toolchain nightly
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/
RUN git clone https://github.com/paritytech/polkadot-sdk.git && cd polkadot-sdk && git checkout polkadot-v1.1.0 && cargo build --release

# Delete below three lines when polka-storage repo will become public
WORKDIR /usr/src/polka-storage
COPY . .
RUN cargo build --release

# Uncomment when polkadot-native-storage repo will become public and available for cloning for everyone
#RUN git clone https://github.com/eigerco/polkadot-native-storage polka-storage && cd polka-storage && cargo build --release --features runtime-benchmarks

FROM debian:bookworm-slim
RUN apt-get update && apt-get upgrade -y && rm -rf /var/lib/apt/lists/*

# Copy binaries
COPY --from=builder /usr/src/polkadot-sdk/target/release/polkadot /usr/local/bin/
COPY --from=builder /usr/src/polkadot-sdk/target/release/polkadot-execute-worker /usr/local/bin/
COPY --from=builder /usr/src/polkadot-sdk/target/release/polkadot-parachain /usr/local/bin/
COPY --from=builder /usr/src/polkadot-sdk/target/release/polkadot-prepare-worker /usr/local/bin/
COPY --from=builder /usr/src/polkadot-sdk/target/release/polkadot-voter-bags /usr/local/bin/
COPY --from=builder /usr/src/polka-storage/target/release/polka-storage-node /usr/local/bin/polka-storage-node

# Copy chain spec
COPY --from=builder /usr/src/polka-storage/polkadot-launch/spec/raw-parachain-chainspec.json /usr/local/etc/
COPY --from=builder /usr/src/polka-storage/polkadot-launch/spec/raw-local-chainspec.json /usr/local/etc/

# Copy script
COPY --chmod=755 scripts/run_nodes.sh /usr/sbin/run_nodes.sh

# Initialize chain
WORKDIR /usr/local/polka
RUN polka-storage-node export-genesis-wasm --chain /usr/local/etc/raw-parachain-chainspec.json /usr/local/polka/para-2000-wasm
RUN polka-storage-node export-genesis-state --chain /usr/local/etc/raw-parachain-chainspec.json /usr/local/polka/para-2000-genesis-state


EXPOSE 8844 9944 9945 9977 30333 30334 30343 40333 40334
CMD ["run_nodes.sh", ""]
