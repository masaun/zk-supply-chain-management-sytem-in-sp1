// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import { SupplyChainVerifier } from "./SupplyChainVerifier.sol";


contract SupplyChainManagementSystem {
    SupplyChainVerifier public supplyChainVerifier;

    constructor(SupplyChainVerifier _supplyChainVerifier) {
        supplyChainVerifier = _supplyChainVerifier;
    }

    // [TODO]: Once the Succinct Prover Network is ready for everyone (NOTE: Currently, it is for invited-users only), I will implement this SupplyChainManagementSystem contract.
}