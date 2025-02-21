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
struct RfidData { // RFID data
    serial_number: u32,
    pub product_code: u32,
    product_description: String,
    batch_number: u16,    // Same with "Lot number"
    is_food: u8,
    //is_food: bool,          // for Food
    expiration_date: u32,  // for Food (in timestamp)
    temperature: u8,     // for Food. The templature that should be maintained at.
    location_origin: String,          // i.e. "New York" -> Field type -> Slice it + Measure length of it -> Array Field type ([u8; N])
    location_destination: String,     // i.e. "London"
    order_date: u32,      // Timestamp
    shipping_date: u32,   // Timestamp
    supplier_ids: u32,
    supplier_certificates: u32,
    supplier_signatures: String,
    supplier_wallet_addresses: String
}

/**
 * @notice - The test of the main() in the main.rs of the /program.
 */
fn main() {
    // Setup logging.
    utils::setup_logger();
    dotenv::dotenv().ok();

    // Create an input stream (image data, which is generated by AI)
    let rfid_data: RfidData = RfidData {
        serial_number: 1,
        product_code: 1,
        product_description: "Apple".to_string(),
        batch_number: 15,
        is_food: 1,
        expiration_date: 1630000000,
        temperature: 4,
        location_origin: "New York".to_string(),
        location_destination: "London".to_string(),
        order_date: 1630000000,
        shipping_date: 1630000000,
        supplier_ids: 1,
        supplier_certificates: 1,
        supplier_signatures: "0x7e4693d2d8cb28382a4ed4401cab7689ae57b7598199060dbdb03abf539106b42add2f24cfc7dad1ec1246f3ca4791b37a36a831588599d3e22075f0d772f99d1b".to_string(),
        supplier_wallet_addresses: "0x7eF0C96322148918c4A47C430CFA65C6A16dcDed".to_string()
    };

    let supplier_name: String = "Sunny Logistics, Inc".to_string();  // Shuld be the "private Output" (Not to be commited as a "public Output")

    // The "input stream" that the program will read from using `sp1_zkvm::io::read`.
    let mut stdin = SP1Stdin::new();
    stdin.write(&rfid_data);
    stdin.write(&supplier_name); // Shuld be the "private Output" (Not to be commited as a "public Output")

    println!("RFID - serial_number: {}", rfid_data.serial_number);
    println!("RFID - product_code: {}", rfid_data.product_code);
    println!("RFID - product_description: {}", rfid_data.product_description);
    println!("RFID - batch_number: {}", rfid_data.batch_number);
    println!("RFID - is_food: {}", rfid_data.is_food);
    println!("RFID - expiration_date: {}", rfid_data.expiration_date);
    println!("RFID - temperature: {}", rfid_data.temperature);
    println!("RFID - location_origin: {}", rfid_data.location_origin);
    println!("RFID - location_destination: {}", rfid_data.location_destination);
    println!("RFID - order_date: {}", rfid_data.order_date);
    println!("RFID - shipping_date: {}", rfid_data.shipping_date);
    println!("RFID - supplier_ids: {}", rfid_data.supplier_ids);
    println!("RFID - supplier_certificates: {}", rfid_data.supplier_certificates);
    println!("RFID - supplier_signatures: {}", rfid_data.supplier_signatures);
    println!("RFID - supplier_wallet_addresses: {}", rfid_data.supplier_wallet_addresses);

    println!("Supplier Name (private state): {}", supplier_name);  // Shuld be the "private" input (Not to be commited as a public value)

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
