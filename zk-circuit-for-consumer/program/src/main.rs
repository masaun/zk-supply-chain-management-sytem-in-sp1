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
struct ProductDetail {
//struct RfidData { // RFID data
    serial_number: u32,
    pub upc_or_ean: u64, // Product Code (1) - UPC or EAN barcode (12 ~ 13 digits)
    pub gtin: u64,       // Product Code (2) - GTIN-12 code (12 digits)
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

pub fn main() {
    // @dev - Read the input data (input stream) from RFID
    let product_detail: ProductDetail = sp1_zkvm::io::read::<ProductDetail>();
    //let rfid_data: RfidData = sp1_zkvm::io::read::<RfidData>();
    let supplier_name: String = sp1_zkvm::io::read::<String>(); // Shuld be the "private" input (Not to be commited as a public value. This value keep a "private" state)

    println!("ProductDetail - serial_number: {}", product_detail.serial_number);
    println!("ProductDetail - upc_or_ean: {}", product_detail.upc_or_ean);
    println!("ProductDetail - gtin: {}", product_detail.gtin);
    println!("ProductDetail - product_description: {}", product_detail.product_description);
    println!("ProductDetail - batch_number: {}", product_detail.batch_number);
    println!("ProductDetail - is_food: {}", product_detail.is_food);
    println!("ProductDetail - expiration_date: {}", product_detail.expiration_date);
    println!("ProductDetail - temperature: {}", product_detail.temperature);
    println!("ProductDetail - location_origin: {}", product_detail.location_origin);
    println!("ProductDetail - location_destination: {}", product_detail.location_destination);
    println!("ProductDetail - order_date: {}", product_detail.order_date);
    println!("ProductDetail - shipping_date: {}", product_detail.shipping_date);
    println!("ProductDetail - supplier_ids: {}", product_detail.supplier_ids);
    println!("ProductDetail - supplier_certificates: {}", product_detail.supplier_certificates);
    println!("ProductDetail - supplier_signatures: {}", product_detail.supplier_signatures);
    println!("ProductDetail - supplier_wallet_addresses: {}", product_detail.supplier_wallet_addresses);

    println!("Supplier Name (private state): {}", supplier_name);  // Shuld be the "private Output" (Not to be commited as a public value)

    // Write the result (true or false) to the output. (NOTE: Only value to be "public Output" should be commited)
    sp1_zkvm::io::commit(&product_detail.upc_or_ean);
    sp1_zkvm::io::commit(&product_detail.gtin);
    sp1_zkvm::io::commit(&product_detail.is_food);
    //sp1_zkvm::io::commit(&rfid_data);
}
