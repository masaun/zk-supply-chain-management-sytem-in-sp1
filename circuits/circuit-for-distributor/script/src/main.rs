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

struct DeliveryOrder {
    gtin: u64,          // GTIN (Global Trading International Number), which is a global "produce code"  - i.e. 9504000219109
    batch_number: u32,  // "Lot number"
    distributor_name: String,
    gln_source_distributor: u64,         // GLN (Global Location Number) of the sender (= Supplier) - i.e. 9506000111247
    gln_destination_retailer: u64,       // GLN (Global Location International Number) of the receiver (= Manufacturer) - i.e. 9516000111258
    purchase_order_date: u32,            // UNIX Timestamp
    manufacturing_order_date: u32,       // UNIX Timestamp
    shipping_order_date: u32,            // UNIX Timestamp
    delivery_order_date: u32,            // UNIX Timestamp
    signature_of_purchase_order: String,      // The ECDSA signature, which was signed by a Retailer when the Retailer ordered. This will be retrieved via the SC storage.
    signature_of_manufacturing_order: String, // The ECDSA signature, which was signed by a Supplier when the Supplier ordered. This will be retrieved via the SC storage.
    signature_of_shipping_order: String       // The ECDSA signature, which was signed by a Manufacturer when the Manufacturer ordered. This will be retrieved via the SC storage.
}

/**
 * @notice - The test of the main() in the main.rs of the /program.
 */
fn main() {
    // Setup logging.
    utils::setup_logger();
    dotenv::dotenv().ok();

    // Create an input stream
    let delivery_order: DeliveryOrder = DeliveryOrder {
        gtin: 123456789123,
        batch_number: 100,
        distributor_name: "Example Logistics Company, Inc".to_string(),
        gln_source_distributor: 9506000111247,
        gln_destination_retailer: 9516000111258,
        purchase_order_date: 1740312091,
        manufacturing_order_date: 1740512091,
        shipping_order_date: 1740543016,
        delivery_order_date: 1740562105,
        signature_of_purchase_order: "0x7e4693d2d8cb28382a4ed4401cab7689ae57b7598199060dbdb03abf539106b42add2f24cfc7dad1ec1246f3ca4791b37a36a831588599d3e22075f0d772f99d1b".to_string(),
        signature_of_manufacturing_order: "0x82fc43164299d16c5427dd1707378039e391709cdd0038bf9c1ac74581f327a24cbb910d375a1754b7d0ea9e97cfe7195c453c6ce65615273677fa0125bd18111c".to_string(),
        signature_of_shipping_order: "0x4cd4a85524cb591d09b931fa8cb7de43eba910f7eb7bc490da9a98da030efb4843ffbaf8685421a33b348128f7a5b94cbb82de6c784d67bfdb9b43a8be02ca511b".to_string()
    };

    //let supplier_name: String = "Sunny Logistics, Inc".to_string();  // Shuld be the "private Output" (Not to be commited as a "public Output")

    // The "input stream" that the program will read from using `sp1_zkvm::io::read`.
    let mut stdin = SP1Stdin::new();
    stdin.write(&delivery_order);
    //stdin.write(&supplier_name); // Shuld be the "private Output" (Not to be commited as a "public Output")

    println!("DeliveryOrder - gtin: {}", delivery_order.gtin);
    println!("DeliveryOrder - batch_number: {}", delivery_order.batch_number);
    println!("DeliveryOrder - distributor_name: {}", delivery_order.distributor_name);
    println!("DeliveryOrder - gln_source_distributor: {}", delivery_order.gln_source_distributor);
    println!("DeliveryOrder - gln_destination_retailer: {}", delivery_order.gln_destination_retailer);
    println!("DeliveryOrder - purchase_order_date: {}", delivery_order.purchase_order_date);
    println!("DeliveryOrder - manufacturing_order_date: {}", delivery_order.manufacturing_order_date);
    println!("DeliveryOrder - shipping_order_date: {}", delivery_order.shipping_order_date);
    println!("DeliveryOrder - delivery_order_date: {}", delivery_order.delivery_order_date);
    println!("DeliveryOrder - signature_of_purchase_order: {}", delivery_order.signature_of_purchase_order);
    println!("DeliveryOrder - signature_of_manufacturing_order: {}", delivery_order.signature_of_manufacturing_order);
    println!("DeliveryOrder - signature_of_shipping_order: {}", delivery_order.signature_of_shipping_order);

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
