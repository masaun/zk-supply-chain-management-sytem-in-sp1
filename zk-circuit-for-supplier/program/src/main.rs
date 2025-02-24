//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

//use alloy_sol_types::SolType;
use serde::{Deserialize, Serialize};

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

pub fn main() {
    // @dev - Read the input data (input stream) from an app.
    let manufacturing_order: ManufacturingOrder = sp1_zkvm::io::read::<ManufacturingOrder>();
    //let supplier_name: String = sp1_zkvm::io::read::<String>(); // Shuld be the "private" input (Not to be commited as a public value. This value keep a "private" state)

    println!("ManufacturingOrder - gtin: {}", manufacturing_order.gtin);
    println!("ManufacturingOrder - batch_number: {}", manufacturing_order.batch_number);
    println!("ManufacturingOrder - supplier_name: {}", manufacturing_order.supplier_name);
    println!("ManufacturingOrder - gln_source_supplier: {}", manufacturing_order.gln_source_supplier);
    println!("ManufacturingOrder - gln_destination_manufacturer: {}", manufacturing_order.gln_destination_manufacturer);
    println!("ManufacturingOrder - purchase_order_date: {}", manufacturing_order.purchase_order_date);
    println!("ManufacturingOrder - manufacturing_order_date: {}", manufacturing_order.manufacturing_order_date);
    println!("ManufacturingOrder - signature_of_purchase_order: {}", manufacturing_order.signature_of_purchase_order);

    //println!("Supplier Name (private state): {}", supplier_name);  // Shuld be the "private Output" (Not to be commited as a public value)

    // Write the result (true or false) to the output. (NOTE: Only value to be "public Output" should be commited)
    sp1_zkvm::io::commit(&manufacturing_order.gtin);
    sp1_zkvm::io::commit(&manufacturing_order.batch_number);
    sp1_zkvm::io::commit(&manufacturing_order.gln_source_supplier);
    sp1_zkvm::io::commit(&manufacturing_order.gln_destination_manufacturer);
    sp1_zkvm::io::commit(&manufacturing_order.manufacturing_order_date);
}
