# ZK Supply Chain Management System


# 【Architecture】

## Actors in Supply Chain

- (0/ Producer)
- 1/ Supplier 
- 2/ Manufacturer
- 3/ Distributor (i.e. Logistics company)
- 4/ Retailer (i.e. Super Market) 🟣<-- In this project, this actor will order. 
- 5/ Consumer 🔵<-- In this project, this actor will **not** be included.
   <img width="497" alt="Image" src="https://github.com/user-attachments/assets/e0332e9c-9a28-4533-9f4d-0368db6d61a6" />

(Source：https://www.debutinfotech.com/blog/blockchain-in-supply-chain-challenges-benefits-use-cases-considerations )

<br>

## Userflow

> Example of a simple supply chain bill flow: 🟣
>
> (- In case of Online Shopping, the **Step 1:** A **Consumer** places a purchase order with a **Retailer** + The **Retailer send a invoice to the **Customer**)
> 
> -   **Step 1:**
>     
>     A **Retailer** places a purchase order with a **Distributor**.
>     
> -   **Step 2:**
>     
>     The **Distributor** then places a purchase order with a **Manufacturer** to `fulfill` the **Retailer**'s order.
>     
> -   **Step 3:**
>     
>     The **Manufacturer** produces the goods and sends an invoice to the **Distributor**.
>     
> -   **Step 4:**
>     
>     The **Distributor** pays the **Manufacturer** and then sends an invoice to the **Retailer**.
>     
> -   **Step 5:**
>     
>     The **Retailer** pays the **Distributor** upon receiving the goods and `verifying` the invoice.
>

<br>
 
> Challenges in managing supply chain bill flow:
> 
> -   **Complex supply chains:** With multiple intermediaries, managing invoices and payments can become intricate.
>     
> -   **International transactions:** Currency fluctuations and different payment methods can add complexity.
>     
> -   **Data discrepancies:** Inaccuracies in purchase orders, delivery receipts, and invoices can lead to payment delays and disputes.

<br>

## Architecture

- Prepare 3 ZK circuit for proving 3 ZKPs:
  - 1/ Proving a ZKP for a consumer/buyer (/w the `Order Detail` struct data)
    - icl. Non ZKP in the `Order Detail` struct data
  - 2/ Proving a ZKP for a supplier/producer (/w the `Product Detail` struct data)
    - icl. a `Order ZKP` in the `Product Detail` struct data
  - 3/ Proving a ZKP for a logistics company (/w the `Logistics Detail` struct data)
    - icl. a `Order ZKP` + `Product Detail ZKP` in the `Logistics Detail` struct data
  - 4/ When a product would be delivered at the consumer's house🏡, the consumer will be able to confirm whether or not the product is the product that the consumer him/herself actually ordered - by calling a view function on blockchain /w the `proof` that was submitted onto blockchain when the step 1/ and they hold. 


<br>

- When a ZKP is proved in each stage, the `submitProof()` should be called with a ZKP (`proof`) and a `signature` (signed with an actor's wallet `PK`)  via SC on the Blockchain in each stage.
  - Each ZKP would be stored into the **"private" `mapping storage` of product order**🟣, which will become the `ZKP-tree` 🌳

<br>

## Directory composition

```bash
zk-for-supply-chain-in-sp1
├─── zk-circuit-for-consumer
│    └─── program
│    └─── script
└─── zk-circuit-for-supplier
│    └─── program
│    └─── script
├─── zk-circuit-for-logistics-company
│    └─── program
│    └─── script
```




<br>

## Flow
- **Smart Contract ①**:
  - 1/ Buyer would pay the fees for logistics -> The payment will be deposited into the SC. 

<br>

- **ZK circuit** for proving a ZKP:
  - 2/ In each check point, each supplier generate a ZKP + send (submit) it to the blockchain.
    


<br>

- **Smart Contract ②**:
  - 3/ **All** ZKPs, which was submitted from each supplier in each check point, would be stored in the SC.🟣
     => Using the mapping storages + the array storage

  - 4/ Once the logistics process would be done (finalized), each supplier can receive the fees-earned via calling the `claim()` /w their `proof`. 🟣
     => These fees will be transferred from the SC①.


<br>

<hr>

# Product Code

## Product Code - GS1 - Fresh Fruit and Vegetable Traceability Guideline
- Overview of the Produce Supply Chain
  <img width="1371" alt="Image" src="https://github.com/user-attachments/assets/3bd0ea4b-e2b5-4bab-b5f5-e43f06fcb3b0" />

- GS1 - A full list of relevant GS1 Identification keys
  <img width="768" alt="Image" src="https://github.com/user-attachments/assets/734db89a-b63c-47cf-b854-a16444fbaf1c" />

- Overview of the Produce Supply Chain
  <img width="1139" alt="Image" src="https://github.com/user-attachments/assets/ca4d2114-1bfb-413b-871a-916dc95d44d8" />

- Fruit & Vegetable product and data flows for end-to-end traceability
  <img width="1399" alt="Image" src="https://github.com/user-attachments/assets/8d17bd9d-2921-403a-bdc9-55678c9e3064" />

- GS1 standards enabling traceability in the fruit and vegetable supply chain
  <img width="765" alt="Image" src="https://github.com/user-attachments/assets/a45f99bb-f07a-4b3e-a522-6c51533d69fb" />

- GS1 standards enabling traceability in the fruit and vegetable supply chain
  <img width="744" alt="Image" src="https://github.com/user-attachments/assets/7c954040-710c-4068-b77c-4e76fa74209d" />
  <img width="742" alt="Image" src="https://github.com/user-attachments/assets/34273b21-0943-4d66-a365-17f32598f543" />
  <img width="742" alt="Image" src="https://github.com/user-attachments/assets/370006a9-f10f-43fa-b6c8-c18c8a840e71" />
  <img width="746" alt="Image" src="https://github.com/user-attachments/assets/e654ac97-c192-4c01-90f6-435b7dd57edf" />


- Key Data Elements
  <img width="295" alt="Image" src="https://github.com/user-attachments/assets/cf40da2c-7058-4e58-a0e5-dd82c3ec0b6e" />

- Data Requirement for each Actors
  <img width="775" alt="Image" src="https://github.com/user-attachments/assets/11df83da-842d-4005-a2aa-927d421b655c" />
  <img width="770" alt="Image" src="https://github.com/user-attachments/assets/02dc28eb-aad1-43ea-9026-9915bbb0d7c5" />
  <img width="1137" alt="Image" src="https://github.com/user-attachments/assets/713fba5a-3042-4a57-9ed9-0b5f46be0bd2" />
  <img width="1147" alt="Image" src="https://github.com/user-attachments/assets/b343f9eb-03e3-4306-a904-8a29640cdda5" />

- Critical Tracking Events (CTEs) for the Produce Industry
  <img width="680" alt="Image" src="https://github.com/user-attachments/assets/532c7db9-b50c-47eb-9135-94a3b93281ed" />
  <img width="685" alt="Image" src="https://github.com/user-attachments/assets/b45c72b4-9759-4506-a19a-d6d4b5b66675" />
  <img width="606" alt="Image" src="https://github.com/user-attachments/assets/489c17be-a1cb-4bf2-957e-078b86b7722b" />
  <img width="651" alt="Image" src="https://github.com/user-attachments/assets/0bb3f42c-4254-4b96-a245-16d267483286" />
  <img width="737" alt="Image" src="https://github.com/user-attachments/assets/025ee8ef-bb77-4a4a-8037-05398c31fa61" />
  <img width="583" alt="Image" src="https://github.com/user-attachments/assets/1a2e9bc6-bc34-4599-81bb-f43d85567c19" />
  <img width="685" alt="Image" src="https://github.com/user-attachments/assets/15d4b307-4007-49c0-bc9f-e3342636e524" />
  <img width="683" alt="Image" src="https://github.com/user-attachments/assets/7f820e77-77ee-4394-9e0b-20f542bf8521" />
  <img width="809" alt="Image" src="https://github.com/user-attachments/assets/e77185c7-a609-4592-a7d3-033977e33a70" />
  <img width="632" alt="Image" src="https://github.com/user-attachments/assets/99f1be2c-499d-4fb2-af4b-fa4910c88ad7" />

- Traceability data collection in business process steps
  <img width="1075" alt="Image" src="https://github.com/user-attachments/assets/57c38ffc-6129-47aa-96cc-dd19ae54bf26" />
  <img width="1074" alt="Image" src="https://github.com/user-attachments/assets/cd14b041-0044-483c-9aee-b14acc5991f8" />

- The Produce Traceability Initiative (US and Canada)
  <img width="987" alt="Image" src="https://github.com/user-attachments/assets/29c4318b-627d-4cae-97eb-3b832f6c01cd" />

(Source -  GS1: https://www.gs1.org/standards/fresh-fruit-and-vegetable-traceability-guideline/current-standard#2-Supply-chain-context+2-1-Supply-chain-overview )

<br>


## Product Code - others

- ideally, 3 type of data should be attached: 
  - `UPC` / `GTIN`
      <img width="779" alt="Image" src="https://github.com/user-attachments/assets/700f65ca-7cb3-4e69-a728-9a95f05c87a2" />    
     ↓
      - In Europe, `EAN` would be used - instead of `UPC` 
        <img width="751" alt="Image" src="https://github.com/user-attachments/assets/e7c5faee-4b88-49f1-aa9c-0d0524159552" />
        <img width="799" alt="Image" src="https://github.com/user-attachments/assets/4d39097d-c1bb-4857-a6e1-45aac4e71c3b" /> 


   - `ISBN` (in case of Book)
      <img width="588" alt="Image" src="https://github.com/user-attachments/assets/118e995a-1dc4-40c5-beb0-2b2806a809f3" />

  - `ASIN` (= The `Amazon Standard Identification Number`)


  - `MPN` (= `Manufacturer Part Numbers`)
     <img width="754" alt="Image" src="https://github.com/user-attachments/assets/fa26a938-08b9-4853-8174-c169cd3235a2" />

  
  - `FNSKU` (= The `Fulfillment Network Stock Keeping Unit`)
      <img width="749" alt="Image" src="https://github.com/user-attachments/assets/a556e1ea-e201-45af-b3a2-d4dcd155d410" />

  - Merchant `SKU`
    <img width="759" alt="Image" src="https://github.com/user-attachments/assets/df711b3d-ef55-4efa-875d-f464983c0129" />


  

(Source - Making sense of Amazon’s product codes: https://emplicit.co/making-sense-of-amazons-product-codes/ )
(Source - Product Codes: A Comprehensive Guide: https://blog.lengow.com/product-code/ )

