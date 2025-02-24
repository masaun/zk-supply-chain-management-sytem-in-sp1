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
    signature_of_delivery_order: String       // The ECDSA signature, which was signed by a Manufacturer when the Manufacturer ordered. This will be retrieved via the SC storage.
}

pub fn main() {
    // @dev - Read the input data (input stream)
    let delivery_order: DeliveryOrder = sp1_zkvm::io::read::<DeliveryOrder>();
    //let supplier_name: String = sp1_zkvm::io::read::<String>(); // Shuld be the "private" input (Not to be commited as a public value. This value keep a "private" state)

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

    //println!("Supplier Name (private state): {}", supplier_name);  // Shuld be the "private Output" (Not to be commited as a public value)

    // Write the result (true or false) to the output. (NOTE: Only value to be "public Output" should be commited)
    sp1_zkvm::io::commit(&delivery_order.gtin);
    sp1_zkvm::io::commit(&delivery_order.batch_number);
    sp1_zkvm::io::commit(&delivery_order.gln_source_distributor);
    sp1_zkvm::io::commit(&delivery_order.gln_destination_retailer);
    sp1_zkvm::io::commit(&delivery_order.delivery_order_date);
}
