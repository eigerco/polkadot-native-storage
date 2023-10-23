# Polkadot Native Storage: Scenarios


## 1 General

We've identified the following scenarios for the storage solution. All of them are written from the user's perspective.

1. Parachain (or Polkadot ecosystem) users shall be able to:
   - store a file which is on their hard drives;
   - store a file that is placed somewhere in the network (URL);
   - retrieve a file from the PNS, transferring it to their computer;
   - get file metadata;
   - delete a file from the storage.
2. All the above use cases, but with a user employing an external client, not using a parachain directly (optional scenario).
3. All of the above use cases, but with the user using our PNS directly.
4. Administrative actions:
   - retrieve information about storage nodes;
   - add/remove a storage node;
   - add/remove the collator from the pool;
   - retrieve information about the system as a whole (free space, etc.).
   - receive alerts (free space shortage).

Besides the above scenarios, we've identified internal scenarios which map the high-level behavior of the system to the low-level implementation details. Mainly, those scenarios are connected with the user stories and are a part of them but are too complicated to describe in the user story form. Those scenarios are:
1. Publishing a deal in the market.
2. Slashing - in case of missed Window-PoSt.

## 2 User scenarios

### 2.1 Store a large CAR file

- Client discovers miners
  - `jsonsrpc::StateListMiners`
- Iteratively queries miner properties
  - `jsonrpc::StateMinerInfo`
- Get a deal quote;  Returns price (per GB-epoch), and range of piece sizes
  - `jsonrpc::ClientQueryAsk`
- Add funds to the market actor, which serve as an escrow
  - `market-actor::AddBalance`
- Propose a deal
  - `market-actor::AddBalance`
- Uploads data to a web server (TBD)
- When the storage provider has collected data to fill a sector, he is then ready to publish the deal.  This happens on-chain.
  - `market-actor::PublishStorageDeals`


## 3 Onboarding Storage Providers and Collators

One of the most important administrative tasks is the management of collators and storage providers. Even the best network will be dead when there will be no one who would like to participate in it.

Our vision is to create an easy and intuitive onboarding process, to make it as simple as possible for the user to become a storage provider or collator. We believe it will make the network more attractive for users.

Onboarding should be a process, not a single action, which should lead a new storage provider or collator right from the zero point to complete setup from the technical and business perspectives. We're convinced that anyone can join the network and start providing some storage services for others (and earn money).

To achieve this goal, we need to provide tools and documentation to guide the user through the whole process. We can divide the process into the following steps:
1. The user decides to become a storage provider or collator based on the information about the network (e.g. how much money can be earned, how much time is needed to be spent, etc.).
2. The user decides to join the network and starts the onboarding process - the user is redirected to the onboarding page with the first step.
3. The user is asked to provide basic information about himself (name, email, etc.).
4. The user is asked to provide technical information (e.g. storage capabilities, network link params, etc.).
5. The pre-verification process - automatically checks if the user can be a storage provider or collator (e.g. if the user has enough free space if the network link is fast enough, etc.).
6. The user is asked to provide some business information (e.g. how much money he would like to earn, etc.) and the potential earnings based on the given information.
7. The user is redirected to the documentation page, where he can find information about the next steps.
8. The user is redirected to the technical setup page, where he can find information about the technical preparation, node setup, etc.
9. The user ends the onboarding process and starts providing storage services for the others.

