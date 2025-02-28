# ZK Supply Chain Management System

## Overview

- In this ZK (Zero-Knowledge) based Supply Chain Management System, we assume a scenario in supply chain industory that:
  - 1/ A Retailer (i.e. Supermarket) would order a Supplier to supply certain batch/lot of goods.
  - 2/ The Supplier would order a Manufacturer to manufacture these batch/lot of goods.
  - 3/ The Manufacturer would order a Distributor (i.e. Logistics company) to deliever these batch/lot of goods.
  - 4/ The Distributor (i.e. Logistics company) to deliever these batch/lot of goods to the Retailer, who order this goods when the step 1/.

<br>

- In each step, an actor above would prove a ZK Proof via a ZK circuit for the actor and send it to the SupplyChainManagementSystem contract (`SupplyChainManagementSystem.sol`) and stored into there and associated each other in the form of its supply chain history - so that its supply chain history can be stored on blockchain **without** that each actor reveal all their confidential business information.
  - A ZK Proof-sent would be verifiered via the verification function in the SupplyChainVerifier contract (`SupplyChainVerifier.sol`), which is called inside the SupplyChainManagementSystem contract (`SupplyChainManagementSystem.sol`).
  - The ZK circuits for four actors would be implemented under the `./circuits` directory.

<br>

- In terms of the smart contracts above, due to that only invited-users are allowd to use the Succinct Prover Network for the moment, I can not prove (generate) a `Groth16`/`Plonk` proof, which can verify on EVM Chains. 
  - Hence, I'm going to restart to implement this smart contract - once anyone are allowed to use the Succinct Prover Network.

<br>

## Actors in this scenario of the Supply Chain industory

- Retailer (i.e. Super Market)
- Supplier 
- Manufacturer
- Distributor (i.e. Logistics company)


<br>

## References

- Succinct:
  - SP1 (zkVM): https://docs.succinct.xyz/docs/sp1/introduction
  - Prover Network: https://docs.succinct.xyz/docs/network/introduction

<br>

- Actors in the Supply Chain industory  
  https://www.debutinfotech.com/blog/blockchain-in-supply-chain-challenges-benefits-use-cases-considerations


