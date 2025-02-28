// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test, console} from "forge-std/Test.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {SupplyChainVerifier} from "../src/SupplyChainVerifier.sol";
import {SP1VerifierGateway} from "@sp1-contracts/SP1VerifierGateway.sol";

contract SupplyChainVerifierTest is Test {
    // [TODO]: Once the Succinct Prover Network is ready for everyone (NOTE: Currently, it is for invited-users only), I will implement this test for the SupplyChainVerifier contract.
}