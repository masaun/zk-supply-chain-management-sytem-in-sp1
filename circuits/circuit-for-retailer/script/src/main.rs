//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

// use env_logger;
// use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use sp1_sdk::{include_elf, utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin, HashableKey};


/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_elf!("zk-for-supply-chain-in-sp1-program");
//const ELF: &[u8] = include_elf!("chess-program"); /// @dev - "chess-program" is referenced from the program/Cargo.toml

#[derive(Serialize, Deserialize)]

struct PurchaseOrder {
    gtin: u64,          // GTIN (Global Trading International Number), which is a global "produce code"  - i.e. 9504000219109
    batch_number: u32,  // "Lot number"
    product_name: String,
    product_description: String,
    gln_source_retailer: u64,      // GLN (Global Location Number) of the sender (= Retailer) - i.e. 9506000111247
    gln_destination_supplier: u64, // GLN (Global Location International Number) of the receiver (= Supplier) - i.e. 9516000111258
    order_date: u32,               // UNIX Timestamp
}

/**
 * @notice - The test of the main() in the main.rs of the /program.
 */
fn main() {
    // Setup logging.
    utils::setup_logger();
    dotenv::dotenv().ok();

    // Create an input stream
    let purchase_order: PurchaseOrder = PurchaseOrder {
        gtin: 123456789123,
        batch_number: 100,
        product_name: "Apple".to_string(),
        product_description: "Apple made in Washington state".to_string(),
        gln_source_retailer: 9506000111247,
        gln_destination_supplier: 9516000111258,
        order_date: 1740312091
    };

    //let supplier_name: String = "Sunny Logistics, Inc".to_string();  // Shuld be the "private Output" (Not to be commited as a "public Output")

    // The "input stream" that the program will read from using `sp1_zkvm::io::read`.
    let mut stdin = SP1Stdin::new();
    stdin.write(&purchase_order);
    //stdin.write(&supplier_name); // Shuld be the "private Output" (Not to be commited as a "public Output")

    println!("PurchaseOrder - gtin: {}", purchase_order.gtin);
    println!("PurchaseOrder - batch_number: {}", purchase_order.batch_number);
    println!("PurchaseOrder - product_name: {}", purchase_order.product_name);
    println!("PurchaseOrder - product_description: {}", purchase_order.product_description);
    println!("PurchaseOrder - gln_source_retailer: {}", purchase_order.gln_source_retailer);
    println!("PurchaseOrder - gln_destination_supplier: {}", purchase_order.gln_destination_supplier);
    println!("PurchaseOrder - order_date: {}", purchase_order.order_date);

    //println!("Supplier Name (private state): {}", supplier_name);  // Shuld be the "private" input (Not to be commited as a public value)

    // Create a `ProverClient` method.
    let client = ProverClient::from_env();

    // Execute the program using the `ProverClient.execute` method, without generating a proof.
    let (public_output, report) = client.execute(ELF, &stdin).run().unwrap();
    println!("public_output: {:?}", public_output); // [Log]: SP1PublicValues { buffer: Buffer { data: [1, 0, 0, 0, 1], ptr: 0 } }
    println!("report: {:?}", report);
    println!(
        "executed program with {} cycles",
        report.total_instruction_count()
    );

    // Setup the prover client.
    let (pk, vk) = client.setup(ELF);

    // Print the verification key.
    println!("Program Verification Key: {}", vk.bytes32());

    // Generate the proof for the given program and input.
    let mut proof = client.prove(&pk, &stdin).run().unwrap();                 // Generating a STARK proof (Not Compressed)
    //let mut proof = client.prove(&pk, &stdin).compressed().run().unwrap();  // Generating a STARK proof (Compressed)
    //let mut proof = client.prove(&pk, &stdin).groth16().run().unwrap();     // Generating a SNARK proof with Groth16
    //let mut proof = client.prove(&pk, &stdin).plonk().run().unwrap();       // Generating a SNARK proof with Plonk
    println!("proof (raw): {:?}", proof);
    println!("Successfully generated proof!");

    // Get the public values as bytes.
    let public_values = proof.public_values.as_slice();
    println!("public values: 0x{}", hex::encode(public_values)); // [Log]: Proof type Core is not supported for onchain verification. Only Plonk and Groth16 proofs are verifiable onchain

    // Get the proof as bytes. (NOTE: Only Groth16/Plonk proofs are supported for onchain verification)
    let solidity_proof = proof.bytes();
    println!("proof (for Solidity Verifier): 0x{}", hex::encode(solidity_proof));

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");

    // Save the proof.
    proof.save("retailer-groth16-proof.bin").expect("saving proof failed");
    //proof.save("retailer-mock-proof.bin").expect("saving proof failed");
    println!("Successfully save the proof!");
    
    let deserialized_proof = SP1ProofWithPublicValues::load("proof-with-pis.bin").expect("loading proof failed");
    println!("Successfully deserialized a proof-saved: {:?}", deserialized_proof);
}
