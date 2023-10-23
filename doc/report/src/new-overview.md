
# Polkadot Native Storage

# Summary
- [Overview](#polkadot-native-storage-overview)
- [Solution](./new-solution.md)
- [Scenarios](./new-scenarios.md)
- [Conclusion](./new-conclusion.md)

# Polkadot Native Storage: Overview
A storage solution for Polkadot

- [1 Introduction](#1-introduction)
- [2 Methodology](#2-methodology)
- [3 General Description](#3-general-description)


# 1. Introduction


Our main goal is to describe how to implement and maintain a Filecoin-like **system parachain** -- a parachain that uses `DOT` as the native token and is easily usable for all kinds of parachain projects in the ecosystem via XCM.

Part of this vision is to support Polkadot's app-centric future, where resilient data storage and retrieval will be significant components.  To name but one example, a Work Package of 5MB in size, could be retrieved from Polkadot Network Storage.  This shows some simplified code based on [RFC-0031](https://github.com/polkadot-fellows/RFCs/blob/d6bb203ae62a4457e88b52f3a22d21efd96f2275/text/0031-corejam.md):

```
type WorkClass = u32;
type WorkPayload = Vec<u8>;
struct WorkItem {
  class: WorkClass,
  payload: WorkPayload,
}

struct WorkPackage {
  authorization: Authorization,
  context: Context,
  items: WorkItem[]
}

type MaxWorkPackageSize = ConstU32<5 * 1024 * 1024>;
struct EncodedWorkPackage {
  version: u32,
  encoded: BoundedVec<u8, MaxWorkPackageSize>,
}
```

One can even start to think about replacing certain package blobs with CIDs.

We have updated research previously done -- research reflecting technical changes in Polkadot, Substrate, and Filecoin itself. We have taken a detailed stab at figuring out what it would take to implement an entire solution as a parachain without depending on 3rd party chains.  Among the architectural elements we evaluated:
- what can be used as is
- what could be adapted for use
- what could be ported to Rust
- what components would need to be written from scratch

It is often said that **Filecoin is complicated.**  The engineers at Protocol Labs have solved a hard problem:
- prove you have replicated a large chunk of data.
- prove that it's unique, not a copy
- prove that you're not cheating by being your own client
- continually prove you are still storing the data

Protocol Labs has built -- and built on -- a great deal of software, which works in the real world. The complex ecosystem is well-documented, used and tested by many users. It is, therefore, logical for us to build on what they have done.

This proposal reflects that perspective.  Essentially, we will run the Filecoin system of data storage on a parachain, albeit massively (and thoughtfully) adapted for Polkadot.


# 2. Methodology

Research, also known as software engineering or computer science research, involves investigating and advancing various aspects of software development, from algorithms and languages to tools and methodologies. It is a process of solving problems, and it is a process of learning. It is also a journey of discovery and invention.

There are several steps to this, including (but not limited to):
- understanding the problem - identify specific conditions, user scenarios and the desired outcome
- gathering information - conduct a comprehensive literature and existing solutions review to understand the current state of knowledge in the area of decentralized storage solutions. Identify gaps, challenges, and opportunities that our research can address.
- formulating questions and hypotheses - define clear and concise research questions or ideas we aim to answer through our document. Our base was also a CGS report, which addressed some problems we would like to solve.
- choose a research approach or methodology that aligns with our research questions - we have decided to do the analysis covered by this document and prototyping by updating the CGS code, including new features and changes in the Substrate ecosystem.
- formulating a solution - based on the research, analysis and experiments with the code, we have prepared a solution that will be a good fit for the Polkadot ecosystem.

The research process is iterative, and we have gone through several rounds of the above steps (such as presenting drafts or checking different code solutions). The next steps should follow:
- implementing the solution - we have already started this process by creating a prototype and will continue it in the further milestones, should the stakeholders collectively decide on such a course of action.
- evaluating the solution - we will assess the solution in the next milestones when there will be more testing code, and we'll be able to run the solution to collect usage data.
- documenting the solution - we will document the solution in the next milestones just as we did in this milestone.

After that, we will go through the following steps with the client:
- communicating the solution - there will be a need to communicate the solution to the community properly.
- maintaining the solution - there will be a need to maintain the solution, and we will need to define the maintenance process.
- improving the solution - there will be a need to improve the solution over time, and we will need to define the improvement process.

The first step from the above methodology we used was to first gain a general understanding of the Filecoin ecosystem, and then to dive into the details of the various components. We had already had some experience with Filecoin's zk-SNARK prover/verifier (`bellperson`), packaging that into a runtime kernel.  To gain a more complete view, we gathered information about Filecoin from several sources:
- online specifications
- white papers
- analysis of code repositories
- running `lotus` and `lotus-miner` nodes
- viewing selected videos from some of the key players:
  - general overview, IPFS, IPLD: Juan Benet
  - FVM: Matt Hamilton
  - Saturn: Patrick Woodhead, Ansgar Grunseid

One must bear in mind that the Filecoin ecosystem is quite dynamic; things are changing at a fast clip.  For instance, FVM and Saturn both went live in the last year.

The Polkadot ecosystem is equally dynamic, and time was similarly spent getting up to speed with changes there.

# 3 General Description

The principle idea from which this proposal flows may be simply stated: &nbsp;  `Port Filecoin to Polkadot`

It is also not our intention to slavishly follow everything that Filecoin has done, for those parts we eventually do keep.  It is merely a model that we know works.  And Polkadot is not Filecoin.

## 3.1 Porting Filecoin to Polkadot

"Never mistake a clear view for a short distance." -- an old Klingon saying

### 3.1.1 General Strategy

As we want to avoid reinventing the wheel, we have decided to keep the basic methodology of Filecoin.  This includes:
- the storage-proof system
- the block transactions (called messages in Filecoin) and their parameter structures
- corresponding actors and methods, porting them to pallets and extrinsics.
- the blockchain state, although with several differences.

See also the section on JSON-RPC, regarding the Filecoin API.

As far as required functionality goes, any storage solution ultimately consists of three main tasks:
- **storage**
- **indexing**
- **retrieval**

### 3.1.2 Divergence from Filecoin

This is a non-exhaustive list of porting aspects that need to be explicitly observed.

- `DOT`, not `FIL`:  This is a MUST have.
- CID vs. hash:  CID's are used for both file ID's (for content addressable IPFS) as well as for all Merkle tree hash values.
  - Indeed, they do contain a hash, in addition to
    - multihash fields: type, length, value
    - multicodec identifier, specifying the encoding.
  - We will use simple (and standard) Substrate Merkle tree hashes throughout, and employ CID's only in reference to files, pieces, and sectors.  Care must be taken so nothing cryptographic breaks.
  - We note that the XCM message self-description properties were inspired by those aspects of IPFS and IPLD.
- The consensus model will not observe tipsets, being a Filecoin-centric paradigm.

There are also constraints that could be loosened.  For example:
- the minimum size of sectors, currently 32G/64G
- the minimum duration of a storage deal, currently 180 days.
- one challenge Proof-of-Spacetime per 6, 12, or 24-hour period, chosen randomly, i.e., a somewhat more optimistic proving. All the proofs eventually go on-chain.

With Coretime, there is a trend towards more agile procurement and utilization.  This is a philosophy that could be folded into Polkadot Native Storage, where appropriate.


### 3.1.3 Consensus

Substrate has undergone many changes during the last two years since the CGS report was published. For example, we may now use AURA consensus in custom chains -- present in Substrate, which was planned when we wrote the CGS report.

There are still many differences in consensus mechanisms between Filecoin and Substrate-based chains. One of the key issues raised in our previous work was collator selection methods. At that time, there did not exist a method to actually choose the collator. According to the Substrate [documentation](https://docs.substrate.io/reference/how-to-guides/parachains/select-collators/), collator selection can now be done in the user-defined way. During our research, we've ported a simple collator selection pallet, similar to the one from Cumulus, to confirm that we can provide a custom collator set. We've changed the selection method to use the `Power` factor.

Our tests have been successful, and we could register collators of our choice, selecting them with different factors (like storage power, etc.) and choosing subsets with custom algorithms. Registration can be done on demand, and we can change the collator set for each session (pallet-session was used during our research).

There are known other possibilities to select collators, like pallets for [parachain staking](https://github.com/moonbeam-foundation/moonbeam/tree/master/pallets/parachain-staking), which can also choose the active set of block producers and handle reward mechanisms.

However, if we select more than one collator, each may try to produce a block, but the chain can still accept only one block at a time. The relay chain selects the block from candidates (2/3 validators must accept such a candidate to start backing it). We can use the previously described collator selection mechanisms to control how many collators can produce blocks simultaneously.

In fact, the above problem is deeper when we consider the rewards. We intend to provide a fair and transparent mechanism allowing every collator to receive rewards periodically.

The active collator set can be renewed every round (1200 blocks or approx 4hrs). The mechanism of selecting collators in the active set can be based on the total power of the collator (and associated actors). The more power, the more likely the collator will be selected. Then, to avoid situations where collators with the highest power will be chosen constantly the same, some randomness can be added to the selection process, and the set can be shuffled, including those collators with lower power factor. It will build the sets of collators mostly with high power factor but allow others to participate in the reward system. The exact mechanism of selection collators should be a subject for further discussions with the stakeholders as it can influence the business model of the parachain.

If parachain staking is chosen during implementation, we can use similar mechanisms. It will have an additional impact on the network as it will introduce delegators - who can stake some funds to support collators. Still, it will need to be subject to further changes, as we'd like to create collator sets based on their storage power and promote those with the highest power.

A high priority will be to keep block *finality* to a minimum latency, e.g., so funds show up as quickly as possible when executing a simple transfer (while storage will take longer).

The same problems and solutions can be applied to Sassafras consensus, which will be used in the future.

### 3.1.4 `lotus` and `lotus-miner`

In the Filecoin ecosystem, these are the main processes that miners run as storage providers.
- `lotus daemon`:
  - runs the blockchain node
  - responsible for maintaining and syncing the blockchain state
  - has Filecoin Virtual Machine, a host for actors
  - handles the JSON-RPC API
  - maintains an indexer
  - `libp2p` market protocols:
    - for proposing storage deals: `/fil/storage/mk/`
    - for querying miners their current StorageAsk:  `/fil/storage/ask`
    - querying miners for the deal status: `/fil/storage/status/`
    - for querying information about retrieval: `/fil/retrieval/qry/` 
- `lotus-miner`
  - the storage provider
  - a.k.a, miner
  - maintains storage (dagstore)
  - performs storage proofs
  - responds to retrieval requests

The parts of those not belonging to a Substrate collator mode will be moved to another process, and possibly another system.

### 3.1.5 Storage mining

Storage fees are based on the miner regularly submitting Window Proofs-of-Storage.
- Deadlines
  - a 24-hour proving period is assigned when a miner is created.  The period has a specific starting time
  - in Filecoin, this is broken down into 48 deadlines, one every 30 minutes
- Storage fee payment
  - the client has deposited escrow to pay for the storage
  - fees are automatically deposited to the storage provider regularly for continuously proving the data is being securely stored
- Slashing
  - The storage provider's collateral may be slashed if the storage provider failed to submit timely Proof-of-Spacetime proofs.
  - there are various stages involved in handling this.

If a miner is selected to create a new block, they receive block rewards, dependent upon if they submit a timely (and correct) Winning-Proof-of-Storage.

## 3.2 Components

This is an outline of the elements required for the parachain under consideration.  The collator node will usually be paired with a storage node; both may be running on the same system, although this is not necessary.

- Collator Node
  - Pallets
    - Miner
    - Market
    - Power
    - Rewards
  - JSON-RPC API modules
    - `Chain`:
    - `Client`:
    - `Market`:
    - `State`:
    - `Sync`:
  - polka-storage: blockchain state
    - blocks, messages, deals, miners, sector info, power table, storage market
  - storage market
  - retrieval market
- Storage Node
  - `polka-store`
    - a Rust executable
    - storage miner
    - maintain file storage, sectors
    - storage proving system
    - paired with collator node, communicating over JSON-RPC
    - publishes new content
  - `polka-index`
    - handles announces and advertisements
    - stores indices: CID-provider mappings + metadata
    - responds to queries
  - `polka-fetch`
    - responds to retrieval requests
    - negotiates with retrieval client
    - sends data via graphsync
<br>
<br>


![architecture](arch_substrate.png)
<br>


