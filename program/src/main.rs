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
    supplier_certificates: u32
}

pub fn main() {
    // @dev - Read the input data from RFID
    let rfid_data: RfidData = sp1_zkvm::io::read::<RfidData>();
    let supplier_name: String = sp1_zkvm::io::read::<String>(); // Shuld be the "private" input (Not to be commited as a public value. This value keep a "private" state)

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

    println!("Supplier Name (private state): {}", supplier_name);  // Shuld be the "private" input (Not to be commited as a public value)

    // Write the result (true or false) to the output.
    sp1_zkvm::io::commit(&rfid_data);
}
