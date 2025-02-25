// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

contract SupplyChainVerifier {
    /// @notice The address of the SP1 verifier contract.
    /// @dev This can either be a specific SP1Verifier for a specific version, or the
    ///      SP1VerifierGateway which can be used to verify proofs for any version of SP1.
    ///      For the list of supported verifiers on each chain, see:
    ///      https://github.com/succinctlabs/sp1-contracts/tree/main/contracts/deployments
    address public verifier;

    /// @notice The verification key for each program.
    bytes32 public retailerProgramVKey;
    bytes32 public supplierProgramVKey;
    bytes32 public manufacturerProgramVKey;
    bytes32 public distributorProgramVKey;

    constructor(
        address _verifier, 
        bytes32 _retailerProgramVKey, 
        bytes32 _supplierProgramVKey, 
        bytes32 _manufacturerProgramVKey, 
        bytes32 _distributorProgramVKey
    ) {
        verifier = _verifier;
        retailerProgramVKey = _retailerProgramVKey;
        supplierProgramVKey = _supplierProgramVKey;
        manufacturerProgramVKey = _manufacturerProgramVKey;
        distributorProgramVKey = _distributorProgramVKey;
    }

    /// @notice The entrypoint for verifying the proof of a retailer's purchase order.
    /// @param _proofBytes The encoded proof.
    /// @param _publicValues The encoded public values.
    function verifyRetailerProof(bytes calldata _publicValues, bytes calldata _proofBytes)
        public
        view
        returns (bool)
    {
        ISP1Verifier(verifier).verifyProof(retailerProgramVKey, _publicValues, _proofBytes);
        //return ISP1Verifier(verifier).verifyProof(retailerProgramVKey, _publicValues, _proofBytes);
    }

    /// @notice The entrypoint for verifying the proof of a supplier's manufacturing order.
    /// @param _proofBytes The encoded proof.
    /// @param _publicValues The encoded public values.
    function verifySupplierProof(bytes calldata _publicValues, bytes calldata _proofBytes)
        public
        view
        returns (bool)
    {
        ISP1Verifier(verifier).verifyProof(supplierProgramVKey, _publicValues, _proofBytes);
        //return ISP1Verifier(verifier).verifyProof(supplierProgramVKey, _publicValues, _proofBytes);
    }

    /// @notice The entrypoint for verifying the proof of a manufacturer's shipping order.
    /// @param _proofBytes The encoded proof.
    /// @param _publicValues The encoded public values.
    function verifyManufacturerProof(bytes calldata _publicValues, bytes calldata _proofBytes)
        public
        view
        returns (bool)
    {
        ISP1Verifier(verifier).verifyProof(manufacturerProgramVKey, _publicValues, _proofBytes);
        //return ISP1Verifier(verifier).verifyProof(manufacturerProgramVKey, _publicValues, _proofBytes);
    }

    /// @notice The entrypoint for verifying the proof of a distributor's delivery order.
    /// @param _proofBytes The encoded proof.
    /// @param _publicValues The encoded public values.
    function verifyDistributorProof(bytes calldata _publicValues, bytes calldata _proofBytes)
        public
        view
        returns (bool)
    {
        ISP1Verifier(verifier).verifyProof(distributorProgramVKey, _publicValues, _proofBytes);
        //return ISP1Verifier(verifier).verifyProof(distributorProgramVKey, _publicValues, _proofBytes);
    }
}
