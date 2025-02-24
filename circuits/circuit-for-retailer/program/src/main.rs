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
struct PurchaseOrder {
    gtin: u64,          // GTIN (Global Trading International Number), which is a global "produce code"  - i.e. 9504000219109
    batch_number: u32,  // "Lot number"
    product_name: String,
    product_description: String,
    gln_source_retailer: u64,      // GLN (Global Location Number) of the sender (= Retailer) - i.e. 9506000111247
    gln_destination_supplier: u64, // GLN (Global Location International Number) of the receiver (= Supplier) - i.e. 9516000111258
    order_date: u32,               // UNIX Timestamp
}

pub fn main() {
    // @dev - Read the input data (input stream) from RFID
    let purchase_order: PurchaseOrder = sp1_zkvm::io::read::<PurchaseOrder>();
    //let supplier_name: String = sp1_zkvm::io::read::<String>(); // Shuld be the "private" input (Not to be commited as a public value. This value keep a "private" state)

    println!("PurchaseOrder - gtin: {}", purchase_order.gtin);
    println!("PurchaseOrder - batch_number: {}", purchase_order.batch_number);
    println!("PurchaseOrder - product_name: {}", purchase_order.product_name);
    println!("PurchaseOrder - product_description: {}", purchase_order.product_description);
    println!("PurchaseOrder - gln_source_retailer: {}", purchase_order.gln_source_retailer);
    println!("PurchaseOrder - gln_destination_supplier: {}", purchase_order.gln_destination_supplier);
    println!("PurchaseOrder - order_date: {}", purchase_order.order_date);

    //println!("Supplier Name (private state): {}", supplier_name);  // Shuld be the "private Output" (Not to be commited as a public value)

    // Write the result (true or false) to the output. (NOTE: Only value to be "public Output" should be commited)
    sp1_zkvm::io::commit(&purchase_order.gtin);
    sp1_zkvm::io::commit(&purchase_order.batch_number);
    sp1_zkvm::io::commit(&purchase_order.gln_source_retailer);
    sp1_zkvm::io::commit(&purchase_order.gln_destination_supplier);
    sp1_zkvm::io::commit(&purchase_order.order_date);
}
