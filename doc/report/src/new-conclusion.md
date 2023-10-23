# Polkadot Native Storage: Conclusion

- [1 Our Previous Proposal](#1-our-previous-proposal)
- [2 Other Storage Solutions](#2-other-storage-solutions)
- [3 Conclusion](#3-conclusion)


# 1 Our Previous Proposal

This is a link to the proposal we submitted two years ago.

https://github.com/common-good-storage/report/blob/master/src/SUMMARY.md


# 2 Other Storage Solutions

We've considered other storage solutions found over the network during our research.

We've evaluated only solutions which:
1. are open source and have a public repository along with a friendly license (or their ideas are described in a public document);
2. have a working implementation;
3. are mature enough to be used in production.

We've found and analyzed the following solutions:

| Name | Description | Language | License    | Status |
| ---- | ----------- | -------- |------------| ------ |
| [Akash](https://akash.network/) | Akash is a decentralized cloud computing marketplace and deployment platform. | Go | Apache 2.0 | Production |
| [Arweave](https://www.arweave.org/) | Arweave is a new type of storage that backs data with sustainable and perpetual endowments, allowing users and developers to truly store data forever – for the very first time. | Rust | Apache 2.0 | Production |
| [Filecoin](https://filecoin.io/) | Filecoin is a decentralized storage network that turns cloud storage into an algorithmic market. | Go | MIT        | Production |
| [IPFS](https://ipfs.io/) | IPFS is a protocol and peer-to-peer network for storing and sharing data in a distributed file system. | Go | MIT/Apache | Production |
| [Sia](https://sia.tech/) | Sia is a decentralized storage platform secured by blockchain technology. The Sia Storage Platform leverages underutilized hard drive capacity around the world to create a data storage marketplace that is more reliable and lower cost than traditional cloud storage providers. | Go | MIT        | Production |
| [Storj](https://storj.io/) | Storj is an open-source platform that leverages the blockchain to provide end-to-end encrypted cloud storage services. | Go | GNU        | Production |


After analyzing the above solutions, we've concluded that they cannot be integrated out-of-the-box with Polkadot. The main reasons are very similar to those described in the case of Filecoin, which we've described now and previously. That makes us believe that the best way to implement a storage solution for Polkadot is to port Filecoin to Polkadot and continue the ideas described in the CGS report.

# 3 Conclusion

## 3.1 Implementation


This is a structured listing of tasks and subtasks, 28 in all.  The end result would be the MVP of the Polkadot Native Storage.  Several of the individual subtasks may be implemented in parallel to others.  For instance, #1 and #2 could largely be implemented simultaneously, by two engineers or engineering groups.

1. Collator node
    - `[1.1]` Research FVM: See [section 3.3 in the solution page](./new-solution.md#33-alternative-bring-the-entire-fvm-into-a-pallet) for more detail here.  The result of this research is a clear plan forward.
    - `[1.2]` Research JSON-RPC: Decide what parts of the API to keep, what to cut, and what needs to be added
    - Implementation Milestones
      - `[1.3.1]` JSON-RPC
      - `[1.3.2]` Running actors in pallets or FVM
      - `[1.3.3]` Serialize blockchain state to disk
      - `[1.3.4]` Block production
      - `[1.3.5]` Common consensus
      - `[1.3.6]` Parachain integration

2. `polka-store`: this is a Rust executable, corresponding to a Filecoin storage provider.
   - `[2.1]` Port `dagstore` to Rust
   - `[2.2]` add JSON-RPC listener.
   - link in forked proof libraries
     - `[2.3.1]` [`rust-fil-proofs`](https://github.com/filecoin-project/rust-fil-proofs)
     - `[2.3.2 ]` [`bellperson`](https://github.com/filecoin-project/bellperson)
   - Implementation Milestones
     - `[2.4.1]` perform a sector store, ending with PoRep returned
     - `[2.4.2]` perform a Winning PoSt
     - `[2.4.3]` perform a Window PoSt

3. `polka-index`
   - `[3.1]` Research phase:
      - determine the processing that this executable should do.
      - define how publish/subscribe will work, and implications for `polka-store` and possibly the runtime.
      - define how `gossipsub` will work.
   - `[3.2]` Fork [`storethehash`](https://github.com/vmx/storethehash)
   - `[3.3]` Implementation

4. `polka-fetch`
   - `[4.1]` fork [`rs-graphsync`](https://github.com/retrieval-markets-lab/rs-graphsync)
   - `[4.2]` return a stored file via CID.
   - `[4.3]` Research phase: figure out caching: technical details, incentivization.
   - `[4.4]` Implement file caching

5. Deployment
   - `[5.1]` Create necesary scripts and instructions to bootstrap a new collator node
   - `[5.2]` Create necesary scripts and instructions to bootstrap a new mining system.  Docker may play a role here.

6. `Delia`: This is a Web-based onramp to creating and executing storage deals.  This handles larger amounts of data, e.g., 4G and up.
   - `[6.1]` Research phase
   - `[6.2]` Implementation

7. `Gregor`: This is a file aggregator.  For smaller sized file content, e.g., a 5MB Work Package.
   - `[7.1]` Research phase
   - `[7.2]` Implementation


### 3.2 Our Team

We have an experienced team ready to work on such an endeavor -- including, but not limited to:

- Mark Henderson - is the VP of Engineering at Equilibrium. He has led the team starting with the original Rust IPFS grant in late 2019, through engagements with many of the largest names in Web3, and is now circling back to finish the critical work the team started with the original Ziggurat proposal. Core contributor to OrbitDB, Rust IPFS, and Ziggurat.
- Piotr Olszewski - is a Software Engineer at Eiger, and has over 13 years of professional experience, with a strong academic background in distributed computing. He has a large bag of experiences, ranging from military appliances, cryptographic projects, telecommunication software to embedded platforms. During his career, Piotr took different roles, from developer to team and tech leader. His main tools are C/C++ and Rust. One of the last works is a port of Move Virtual Machine to Substrate ecosystem.
- Karlo Mardesic - is a Software Engineer at Eiger and has experience with telecommunications and low-level drivers in C/C++. These days his expertise has shifted to blockchain technology and P2P protocols, where he primarily uses Rust to tackle exciting problems. One of the last works is a port of Move Virtual Machine to Substrate ecosystem.
- Kyle Granger - is a Software Engineer at Eiger. He has wide experience in 3D graphics, audio, video, WebRTC. A lifelong interest in cryptography led to creating a block cipher interoperable between C++ and GPU shaders. At Eiger, he participated in the early development of Gevulot (https://www.gevulot.com), integrating proof systems for Filecoin, Marlin, and Groth16, both for WASM and Rust. Kyle has researched GPU applications for cryptography and applied 3D visualization to p2p networks
- Tomek Piotrowski - is a Software Engineer at Eiger with extensive Rust experience. Since joining Eiger he has worked exclusively with Zcashd and Zebra codebases and supported Eiger’s efforts in the Zcash ecosystem.

## 3.3 Risks

1. Implementation details can affect architecture and design decisions. Although we've made a PoC modelling some of the design decisions, there is a chance that some of the technical constraints may still be unknown. We will need to be flexible and adapt to the situation.
2. The project is complex and requires a lot of work (and still, some research). We will need to be careful with the time estimations and make sure that we can deliver the project on time.
3. Technology changes - we must be aware of the changes in the Polkadot ecosystem and adapt to them.
4. Easy onboarding - we must provide an easy onboarding process for storage providers and collators. It may not be easy to achieve, and it may occur it will be iterative process which can take a longer time to be polished.
5. Storage providers and collators may not be interested in joining the network for other than technical reasons. We should also provide a good business (reward) model for them to make it attractive.
6. Task complexity (storing and retrieving) may be too high for some users. We will need to provide a simple and intuitive interface for them to make it easy to use to be attractive for the vast majority of users.

## 3.4 Future Plans
This project is a part of a bigger vision. We believe that the storage solution is a crucial component of the Polkadot ecosystem. We would like to continue our work and provide a full-featured storage solution for Polkadot. To achieve that we need to divide our future work into three areas: remaining research to be done, implementation, and support.

Implementation is the most important part in short term. It aims to deliver a fully functional solution for Polkadot in the form of a parachain. While fully functional, it does not mean the end of our work. The first working version is needed to verify all of the requirements and absorb initial feedback from users and testers. That can signal the start of the support phase, where we will react to feedback, delivering improvements to the parachain.

Research is a long term task. It aims to deepen the knowledge of areas that may be done better as well as providing new features that can be implemented. It may be done in parallel with the storage implementation.







