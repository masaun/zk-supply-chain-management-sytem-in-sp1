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
    product_id: u32,
    purchase_order_id: u32,
    batch_number: u16,    // Same with "Lot number"
    is_food: u8,
    location_destination: String,     // i.e. "London"
    order_date: u32,      // Timestamp
}

pub fn main() {
    // @dev - Read the input data (input stream) from RFID
    let purchase_order: PurchaseOrder = sp1_zkvm::io::read::<PurchaseOrder>();
    //let supplier_name: String = sp1_zkvm::io::read::<String>(); // Shuld be the "private" input (Not to be commited as a public value. This value keep a "private" state)

    println!("PurchaseOrder - product_id: {}", purchase_order.product_id);
    println!("PurchaseOrder - location_destination: {}", purchase_order.location_destination);
    println!("PurchaseOrder - batch_number: {}", purchase_order.batch_number);
    println!("PurchaseOrder - is_food: {}", purchase_order.is_food);
    println!("PurchaseOrder - location_destination: {}", purchase_order.location_destination);
    println!("PurchaseOrder - order_date: {}", purchase_order.order_date);

    //println!("Supplier Name (private state): {}", supplier_name);  // Shuld be the "private Output" (Not to be commited as a public value)

    // Write the result (true or false) to the output. (NOTE: Only value to be "public Output" should be commited)
    sp1_zkvm::io::commit(&product_detail.product_id);
    sp1_zkvm::io::commit(&product_detail.purchase_order_id);
    sp1_zkvm::io::commit(&product_detail.is_food);
}
