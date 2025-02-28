# ZK circuit for Distributor

## Introduction

- This is the ZK circuit (on top of SP1 zkVM) for the Distributor (i.e. Logistics company) to deliever these batch/lot of goods to a Retailer, who originally order the goods.

<br>

## Installation
- 1/ Compile program
```shell
cd program
cargo prove build
```

- 2/ Executing program without generating a proof.
```shell
cd script
RUST_LOG=info cargo run --release -- --execute
```

- 3/ Executing program with generating a proof.
```shell
cd script
RUST_LOG=info cargo run --release -- --prove
```

<br>

## Installation - Script
- Run the program and script above at the same time:
```bash
(at the root directory)

sh run_program_and_script.sh
```

<br>

## References

- Succinct:
  - SP1 (zkVM): https://docs.succinct.xyz/docs/sp1/introduction
  - Prover Network: https://docs.succinct.xyz/docs/network/introduction

<br>

- Actors in the Supply Chain industory  
  https://www.debutinfotech.com/blog/blockchain-in-supply-chain-challenges-benefits-use-cases-considerations

<br>

- Authentic Barcodes Powered by GS1 Standards: 
  - Fresh Fruit and Vegetable Traceability Guideline:  
    https://www.gs1.org/standards/fresh-fruit-and-vegetable-traceability-guideline/current-standard#2-Supply-chain-context+2-1-Supply-chain-overview
