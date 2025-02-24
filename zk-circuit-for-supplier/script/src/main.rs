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
use sp1_sdk::{include_elf, utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin};


/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_elf!("zk-for-supply-chain-in-sp1-program");
//const ELF: &[u8] = include_elf!("chess-program"); /// @dev - "chess-program" is referenced from the program/Cargo.toml

#[derive(Serialize, Deserialize)]

struct ManufacturingOrder {
    gtin: u64,          // GTIN (Global Trading International Number), which is a global "produce code"  - i.e. 9504000219109
    batch_number: u32,  // "Lot number"
    supplier_name: String,
    gln_source_supplier: u64,           // GLN (Global Location Number) of the sender (= Retailer) - i.e. 9506000111247
    gln_destination_manufacturer: u64,  // GLN (Global Location International Number) of the receiver (= Supplier) - i.e. 9516000111258
    purchase_order_date: u32,           // UNIX Timestamp
    manufacturing_order_date: u32,      // UNIX Timestamp
    signature_of_purchase_order: String // The ECDSA signature, which was signed by a Retailer when the Retailer ordered. This will be retrieved via the SC storage.
}

/**
 * @notice - The test of the main() in the main.rs of the /program.
 */
fn main() {
    // Setup logging.
    utils::setup_logger();
    dotenv::dotenv().ok();

    // Create an input stream
    let manufacturing_order: ManufacturingOrder = ManufacturingOrder {
        gtin: 123456789123,
        batch_number: 100,
        supplier_name: "Example Supplier, Inc".to_string(),
        gln_source_supplier: 9516000111258,
        gln_destination_manufacturer: 9516000111276,
        purchase_order_date: 1740312091,
        manufacturing_order_date: 1740512091,
        signature_of_purchase_order: "0x7e4693d2d8cb28382a4ed4401cab7689ae57b7598199060dbdb03abf539106b42add2f24cfc7dad1ec1246f3ca4791b37a36a831588599d3e22075f0d772f99d1b".to_string()
    };

    //let supplier_name: String = "Sunny Logistics, Inc".to_string();  // Shuld be the "private Output" (Not to be commited as a "public Output")

    // The "input stream" that the program will read from using `sp1_zkvm::io::read`.
    let mut stdin = SP1Stdin::new();
    stdin.write(&manufacturing_order);
    //stdin.write(&supplier_name); // Shuld be the "private Output" (Not to be commited as a "public Output")

    println!("ManufacturingOrder - gtin: {}", manufacturing_order.gtin);
    println!("ManufacturingOrder - batch_number: {}", manufacturing_order.batch_number);
    println!("ManufacturingOrder - supplier_name: {}", manufacturing_order.supplier_name);
    println!("ManufacturingOrder - gln_source_supplier: {}", manufacturing_order.gln_source_supplier);
    println!("ManufacturingOrder - gln_destination_manufacturer: {}", manufacturing_order.gln_destination_manufacturer);
    println!("ManufacturingOrder - purchase_order_date: {}", manufacturing_order.purchase_order_date);
    println!("ManufacturingOrder - manufacturing_order_date: {}", manufacturing_order.manufacturing_order_date);
    println!("ManufacturingOrder - signature_of_purchase_order: {}", manufacturing_order.signature_of_purchase_order);

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

    // Generate the proof for the given program and input.
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, &stdin).run().unwrap();
    println!("Successfully generated proof!");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");
}
